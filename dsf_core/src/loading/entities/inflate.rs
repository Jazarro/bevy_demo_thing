use bevy::prelude::*;

use crate::levels::tiles::objects::{ExitDoor, Key, Tool};
use crate::levels::tiles::tile_defs::{Archetype, DepthLayer, TileDefinition};
use crate::loading::assets::{AssetStorage, AssetType, SpriteType};
use crate::loading::entities::bundles::{EnemyBundle, PlayerBundle};
use crate::systems::animations::structs::AnimationTimer;
use crate::systems::enemy::spawner::Spawner;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::revolving_door::RevolvingDoor;
use crate::systems::trap_wall::TrappedWall;

pub fn spawn_from_def(
    commands: &mut Commands,
    storage: &AssetStorage,
    pos: Pos,
    tile_def: &TileDefinition,
) {
    let mut entity = commands.spawn();
    entity.insert(Coords::new(pos, tile_def.dimens));
    if tile_def.asset.is_some() {
        entity.insert_bundle(inflate_sprite_sheet(
            pos,
            tile_def.asset.unwrap(),
            tile_def,
            storage,
        ));
    }
    match tile_def.archetype {
        Some(Archetype::Player) => {
            entity.insert_bundle(PlayerBundle {
                steering: Steering::new(pos),
                anim: AnimationTimer::for_player(),
                ..default()
            });
        }
        Some(Archetype::Key) => {
            entity.insert(Key);
        }
        Some(Archetype::Tool(tool_type)) => {
            if let Some(AssetType(sprite, sprite_nr)) = tile_def.asset {
                entity.insert(Tool::new(tool_type, sprite, sprite_nr));
            } else {
                error!("Tool definition {:?} did not have still asset.", tool_type);
            }
        }
        Some(Archetype::Door) => {
            entity.insert(ExitDoor);
        }
        Some(Archetype::Spawner) => {
            entity.insert(Spawner::default());
        }
        Some(Archetype::TrapWall) => {
            entity.insert(TrappedWall::default());
        }
        Some(Archetype::RevolvingDoor(direction)) => {
            entity.insert(RevolvingDoor::new(direction));
        }
        _ => (),
    };
}

pub fn spawn_enemy(commands: &mut Commands, storage: &AssetStorage, coords: Coords) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: storage.get_atlas(&SpriteType::EnemyAnims),
            transform: Transform::from_xyz(
                coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                DepthLayer::Enemies.z(),
            ),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(coords.dimens.as_vec2()),
                ..default()
            },
            ..default()
        })
        .insert_bundle(EnemyBundle {
            steering: Steering::new(coords.pos),
            anim: AnimationTimer::for_player(),
            ..default()
        })
        .insert(coords)
        .id()
}

pub fn inflate_sprite_sheet(
    pos: Pos,
    asset: AssetType,
    tile_def: &TileDefinition,
    storage: &AssetStorage,
) -> SpriteSheetBundle {
    let AssetType(sprite_type, index) = asset;
    let texture_atlas = storage.get_atlas(&sprite_type);
    let flip_x = tile_def.archetype == Some(Archetype::RevolvingDoor(Direction1D::Negative));
    SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index,
            flip_x,
            custom_size: Some(tile_def.dimens.as_vec2()),
            ..default()
        },
        texture_atlas,
        transform: Transform::from_xyz(
            pos.x as f32 + tile_def.dimens.x as f32 * 0.5,
            pos.y as f32 + tile_def.dimens.y as f32 * 0.5,
            tile_def.depth.z(),
        ),
        ..default()
    }
}
