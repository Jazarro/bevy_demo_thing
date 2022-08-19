use std::fs;

use bevy::prelude::*;

use crate::camera::camera_components::FocalPoint;
use crate::config::settings::debug_settings::DebugSettings;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::levels::bundles::PlayerBundle;
use crate::levels::keys_on_door::add_key_displays_to_door;
use crate::levels::tiles::background::{BackgroundEyes, BackgroundHeads, BackgroundTag};
use crate::levels::tiles::objects::{Block, ExitDoor, Key, Tool};
use crate::levels::tiles::tile_defs::{Archetype, DepthLayer, TileDefinition, TileDefinitions};
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::{AssetStorage, AssetType, SpriteType};
use crate::states::LevelLoaded;
use crate::systems::animations::structs::AnimationTimer;
use crate::systems::enemy::spawner::Spawner;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::player::{DebugPosGhostTag, DebugSteeringGhostTag};
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::revolving_door::RevolvingDoor;
use crate::systems::rewind::structs::History;
use crate::systems::trap_wall::TrappedWall;
use crate::systems::win_checking::WinCondition;
use crate::util::files::{get_world_dir, load_level_file};

pub fn load_level(
    mut commands: Commands,
    mut events: EventWriter<LevelLoaded>,
    mut win_condition: ResMut<WinCondition>,
    storage: Res<AssetStorage>,
    instruction: Res<LevelSelectionInstruction>,
    debug_settings: Res<DebugSettings>,
) {
    win_condition.reset();
    let display_debug_frames = debug_settings.display_debug_frames;
    let tile_defs = load_tile_definitions();
    let level = load_level_file(instruction.level.as_ref().unwrap());
    add_background(&level.world_bounds, &mut commands, &storage);
    let mut door_pos = None;
    level.tiles.iter().for_each(|(pos, tile_def_key)| {
        // info!("Load {:?} at {:?}.", tile_def_key, pos);
        let mut entity = commands.spawn();
        let tile_def = tile_defs.get(tile_def_key);
        if let Some(AssetType(sprite_type, index)) = tile_def.asset {
            let atlas = storage.get_atlas(&sprite_type);
            let flip = tile_def.archetype == Some(Archetype::RevolvingDoor(Direction1D::Negative));
            entity.insert_bundle(SpriteSheetBundle {
                texture_atlas: atlas,
                transform: load_transform(pos, &tile_def.dimens, &tile_def.depth),
                sprite: TextureAtlasSprite {
                    index,
                    flip_x: flip,
                    ..default()
                },
                ..default()
            });
        }
        entity.insert(Block { pos: *pos });

        match tile_def.archetype {
            Some(Archetype::Player) => {
                entity.insert_bundle(PlayerBundle {
                    steering: Steering::new(*pos, tile_def.dimens),
                    anim: AnimationTimer::for_player(),
                    ..default()
                });
                if display_debug_frames {
                    build_frames(&mut commands, &storage, tile_def);
                }
            }
            Some(Archetype::Key) => {
                win_condition.add_key(*pos);
                entity.insert(Key::new(*pos));
            }
            Some(Archetype::Tool(tool_type)) => {
                if let Some(AssetType(sprite, sprite_nr)) = tile_def.asset {
                    entity.insert(Tool::new(tool_type, sprite, sprite_nr));
                } else {
                    error!(
                        "Tool definition {:?} did not have still asset.",
                        tile_def_key
                    );
                }
            }
            Some(Archetype::Door) => {
                entity.insert(ExitDoor {
                    pos: *pos,
                    dimens: tile_def.dimens,
                });
                door_pos = Some(pos);
            }
            Some(Archetype::Spawner) => {
                entity.insert(Spawner::new(*pos));
            }
            Some(Archetype::TrapWall) => {
                entity.insert(TrappedWall::new(*pos));
            }
            Some(Archetype::RevolvingDoor(direction)) => {
                entity.insert(RevolvingDoor::new(direction));
            }
            _ => (),
        };
    });

    if let Some(door_pos) = door_pos {
        add_key_displays_to_door(&mut commands, *door_pos, &win_condition, &storage);
    };
    // let hud_pos = level.world_bounds.pos.append_y(-2);
    // let hud_dimens = IVec2::new(32, 2);
    // let transform = load_transform(
    //     &hud_pos,
    //     &hud_dimens,
    //     &DepthLayer::UiElements,
    // );
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.2, 0.2, 0.6),
    //             custom_size: Some(Vec2::new(
    //                 128. * hud_dimens.x as f32,
    //                 128. * hud_dimens.y as f32,
    //             )),
    //             ..default()
    //         },
    //         transform,
    //         ..default()
    //     });

    commands.insert_resource(TileMap::for_play(&level, tile_defs));
    commands.insert_resource(History::default());
    events.send(LevelLoaded);
}

/// Loads the TileDefinitions from file.
#[must_use]
pub fn load_tile_definitions() -> TileDefinitions {
    let file = get_world_dir().join("tile_definitions.ron");
    let data = fs::read_to_string(file).expect("Unable to read TileDefinitions file");
    ron::de::from_str::<TileDefinitions>(&data).expect("Unable to deserialise TileDefinitions")
}

pub fn add_background(world_bounds: &WorldBounds, commands: &mut Commands, storage: &AssetStorage) {
    let transform = load_transform(
        &world_bounds.pos,
        &*world_bounds.dimens,
        &DepthLayer::Background,
    );
    let bg = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: storage.get_atlas(&SpriteType::Background),
            transform,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(BackgroundTag)
        .insert(FocalPoint)
        .id();
    let heads = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: storage.get_atlas(&SpriteType::BackgroundHeads),
            transform: Transform::from_xyz(0., 0., 1.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(BackgroundHeads::default())
        .insert(Parent(bg))
        .id();
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: storage.get_atlas(&SpriteType::BackgroundEyes),
            transform: Transform::from_xyz(0., 0., 1.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(BackgroundEyes::default())
        .insert(Parent(heads));
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0., 0., 0., 0.7),
            custom_size: Some(Vec2::new(
                128. * world_bounds.dimens.x as f32,
                128. * world_bounds.dimens.y as f32,
            )),
            ..default()
        },
        transform: load_transform(
            &world_bounds.pos,
            &*world_bounds.dimens,
            &DepthLayer::BackgroundScrim,
        ),
        ..default()
    });
}

pub fn add_plain_background(world_bounds: &WorldBounds, commands: &mut Commands) {
    let transform = load_transform(
        &world_bounds.pos,
        &*world_bounds.dimens,
        &DepthLayer::Background,
    );
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.6),
                custom_size: Some(Vec2::new(
                    128. * world_bounds.dimens.x as f32,
                    128. * world_bounds.dimens.y as f32,
                )),
                ..default()
            },
            transform,
            ..default()
        })
        .insert(BackgroundTag);
}

fn build_frames(commands: &mut Commands, storage: &Res<AssetStorage>, player_def: &TileDefinition) {
    let texture_atlas = storage.get_atlas(&SpriteType::Frame);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            transform: Transform::default()
                .with_scale(Vec3::new(
                    player_def.dimens.x as f32 / 50.,
                    player_def.dimens.x as f32 / 50.,
                    1.,
                ))
                .with_translation(Vec3::new(
                    player_def.dimens.x as f32 * 0.5,
                    player_def.dimens.y as f32 * 0.5,
                    DepthLayer::UiElements.z(),
                )),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(DebugSteeringGhostTag);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::default()
                .with_scale(Vec3::new(1. / 50., 1. / 50., 1.))
                .with_translation(Vec3::new(0.5, 0.5, DepthLayer::UiElements.z())),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(DebugPosGhostTag);
}

#[must_use]
pub fn load_transform(pos: &Pos, dimens: &IVec2, depth: &DepthLayer) -> Transform {
    let mut transform = Transform::default();
    transform.translation.x = pos.x as f32 + dimens.x as f32 * 0.5;
    transform.translation.y = pos.y as f32 + dimens.y as f32 * 0.5;
    transform.translation.z = depth.z();
    transform.scale = Vec3::new(1. / 128., 1. / 128., 1.0);
    transform
}
