use bevy::prelude::*;

use crate::levels::tiles::objects::{ExitDoor, Key, KeyDisplay};
use crate::levels::tiles::tile_defs::DepthLayer;
use crate::loading::assets::{AssetStorage, SpriteType};
use crate::systems::motion::structs::coords::Coords;
use crate::systems::win_checking::WinCondition;

pub fn add_key_displays_to_door(
    mut commands: Commands,
    storage: Res<AssetStorage>,
    mut win_condition: ResMut<WinCondition>,
    query_keys: Query<&Coords, With<Key>>,
    query_door: Query<&Coords, With<ExitDoor>>,
) {
    if let Some(door_coords) = query_door.iter().next() {
        for (index, key_coords) in query_keys.iter().enumerate() {
            win_condition.add_key(key_coords.pos);
            // Temporary bit of code to arrange the key displays on the door in a
            // visually pleasing manner. Rewrite this later, when we know exactly what we
            // want to do with the door.
            let i = if index < 2 {
                index + 5
            } else if index < 4 {
                index + 7
            } else if index < 5 {
                index
            } else if index < 7 {
                index - 5
            } else if index < 9 {
                index
            } else if index < 11 {
                index - 7
            } else {
                index
            };
            let mut transform = Transform::default();
            let x_offset = i % 4;
            let y_offset = i / 4;
            transform.translation.x = door_coords.pos.x as f32 + x_offset as f32 + 0.5;
            transform.translation.y = door_coords.pos.y as f32 + y_offset as f32 + 0.5;
            transform.translation.z = DepthLayer::FloatingBlocks.z();
            transform.scale = Vec3::new(1. / 256., 1. / 256., 1.0);
            let texture_atlas = storage.get_atlas(&SpriteType::Tools);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas,
                    transform,
                    sprite: TextureAtlasSprite {
                        index: 6,
                        ..default()
                    },
                    ..default()
                })
                .insert(KeyDisplay::new(key_coords.pos));
        }
    }
}
