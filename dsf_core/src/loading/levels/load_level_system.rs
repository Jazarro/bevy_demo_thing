use std::fs;

use bevy::prelude::*;

use crate::camera::camera_components::FocalPoint;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::levels::tiles::background::{BackgroundEyes, BackgroundHeads, BackgroundTag};
use crate::levels::tiles::tile_defs::{DepthLayer, TileDefinitions};
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::{AssetStorage, SpriteType};
use crate::loading::entities::inflate::spawn_from_def;
use crate::states::LevelLoaded;
use crate::systems::rewind::structs::History;
use crate::systems::win_checking::WinCondition;
use crate::util::files::{get_world_dir, load_level_file};

pub fn load_level(
    mut commands: Commands,
    mut events: EventWriter<LevelLoaded>,
    mut win_condition: ResMut<WinCondition>,
    storage: Res<AssetStorage>,
    instruction: Res<LevelSelectionInstruction>,
) {
    win_condition.reset();
    let tile_defs = load_tile_definitions();
    let level = load_level_file(instruction.level.as_ref().unwrap());
    add_background(&level.world_bounds, &mut commands, &storage);
    level.tiles.iter().for_each(|(pos, tile_def_key)| {
        debug!("Load {:?} at {:?}.", tile_def_key, pos);
        let tile_def = tile_defs.get(tile_def_key);
        spawn_from_def(&mut commands, &storage, *pos, tile_def);
    });
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
    let bg = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: storage.get_atlas(&SpriteType::Background),
            transform: Transform::from_xyz(
                world_bounds.pos.x as f32 + world_bounds.dimens.x as f32 * 0.5,
                world_bounds.pos.y as f32 + world_bounds.dimens.y as f32 * 0.5,
                DepthLayer::Background.z(),
            ),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(world_bounds.dimens.as_vec2()),
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
        .id();
    let eyes = commands
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
        .id();
    commands.entity(heads).push_children(&[eyes]);
    commands.entity(bg).push_children(&[heads]);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0., 0., 0., 0.7),
            custom_size: Some(Vec2::new(
                world_bounds.dimens.x as f32,
                world_bounds.dimens.y as f32,
            )),
            ..default()
        },
        transform: Transform::from_xyz(
            world_bounds.pos.x as f32 + world_bounds.dimens.x as f32 * 0.5,
            world_bounds.pos.y as f32 + world_bounds.dimens.y as f32 * 0.5,
            DepthLayer::Background.z(),
        ),
        ..default()
    });
}

pub fn add_plain_background(world_bounds: &WorldBounds, commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.6),
                custom_size: Some(Vec2::new(
                    world_bounds.dimens.x as f32,
                    world_bounds.dimens.y as f32,
                )),
                ..default()
            },
            transform: Transform::from_xyz(
                world_bounds.pos.x as f32 + world_bounds.dimens.x as f32 * 0.5,
                world_bounds.pos.y as f32 + world_bounds.dimens.y as f32 * 0.5,
                DepthLayer::Background.z(),
            ),
            ..default()
        })
        .insert(BackgroundTag);
}
