use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::tiles::objects::Tool;
use crate::levels::tiles::tile_defs::{TileDefinition, ToolType};
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::{AssetStorage, SoundType};
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::{EquippedTag, Player};
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;

/// Tool width and height, hardcoded for now.
/// TODO: Don't hardcode.
const TOOL_WIDTH: f32 = 2.;
const TOOL_HEIGHT: f32 = 2.;

/// Checks if the player intersects any tools.
/// If so, the tool will equipped by the player and will be removed from the game.
pub fn pickup_system(
    mut audio: EventWriter<SoundEvent>,
    mut commands: Commands,
    storage: Res<AssetStorage>,
    mut query_player: Query<(&mut Player, Entity, &Coords, &Transform)>,
    query_tools: Query<(&Tool, &Transform, Entity)>,
) {
    let player = query_player
        .iter_mut()
        .map(|(player, entity, coords, transform)| {
            (
                player,
                entity,
                Vec2::new(transform.translation.x, transform.translation.y),
                Vec2::new(coords.dimens.x as f32, coords.dimens.y as f32),
            )
        })
        .next();
    if let Some((mut player, player_entity, pos, dimens)) = player {
        if player.equipped.is_some() {
            return;
        }
        // Find the first tool that intersects with the player:
        let tool_opt = query_tools.iter().find(|(_, transform, _)| {
            let key_x = transform.translation.x;
            let key_y = transform.translation.y;
            pos.x - dimens.x / 2. < key_x + TOOL_WIDTH / 3.
                && pos.x + dimens.x / 2. > key_x - TOOL_WIDTH / 3.
                && pos.y - dimens.y / 2. < key_y + TOOL_HEIGHT / 3.
                && pos.y + dimens.y / 2. > key_y - TOOL_HEIGHT / 3.
        });
        if let Some((tool, _, tool_entity)) = tool_opt {
            audio.send(SoundEvent::Sfx(SoundType::PickupTool, false));
            player.equipped = Some(tool.tool_type);
            commands.entity(tool_entity).despawn_recursive();

            let atlas = storage.get_atlas(&tool.sprite);
            let equipped = commands
                .spawn()
                .insert(EquippedTag)
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas,
                    transform: Transform::from_xyz(0., 0., 0.),
                    sprite: TextureAtlasSprite {
                        index: tool.sprite_nr,
                        custom_size: Some(Vec2::new(TOOL_WIDTH, TOOL_HEIGHT)),
                        ..default()
                    },
                    ..default()
                })
                .id();
            commands.entity(player_entity).push_children(&[equipped]);
        }
    }
}

pub fn use_tool_system(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut query_player: Query<(&mut Player, &Steering, &Coords)>,
    query_tags: Query<(Entity, &EquippedTag)>,
    query_blocks: Query<(Entity, &Coords)>,
    mut tile_map: ResMut<TileMap>,
    keys: Res<Input<KeyCode>>,
) {
    let wants_to_use_tool = keys.just_pressed(KeyCode::Space);
    if !wants_to_use_tool {
        return;
    }
    for (mut player, steering, player_coords) in query_player.iter_mut() {
        if !steering.is_grounded() {
            return;
        }
        let targeted_blocks = match player.equipped {
            Some(ToolType::BreakBlocksHorizontally(depth)) => {
                let player_is_not_too_far_away_from_wall = at_least_one_is_breakable(
                    &tiles_to_side(1, steering, player_coords, &tile_map.world_bounds),
                    &tile_map,
                );
                if player_is_not_too_far_away_from_wall {
                    Some(tiles_to_side(
                        depth,
                        steering,
                        player_coords,
                        &tile_map.world_bounds,
                    ))
                } else {
                    None
                }
            }
            Some(ToolType::BreakBlocksBelow(depth)) => Some(tiles_below(
                depth,
                steering,
                player_coords,
                &tile_map.world_bounds,
            )),
            _ => None,
        };
        if let Some(targeted_blocks) = targeted_blocks {
            let at_least_one_is_breakable = at_least_one_is_breakable(&targeted_blocks, &tile_map);
            let none_are_unbreakable = none_are_unbreakable(&targeted_blocks, &tile_map);
            if at_least_one_is_breakable && none_are_unbreakable {
                audio.send(SoundEvent::Sfx(SoundType::Mining, false));
                player.equipped = None;
                for pos in &targeted_blocks {
                    tile_map.remove_tile(pos);
                }
                for (entity, _) in query_tags.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                for (entity, block_coords) in query_blocks.iter() {
                    if targeted_blocks.contains(&block_coords.pos) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn at_least_one_is_breakable(blocks: &[Pos], tile_map: &TileMap) -> bool {
    blocks.iter().any(|pos| {
        tile_map
            .get_tile(pos)
            .map_or(false, TileDefinition::is_breakable)
    })
}

fn none_are_unbreakable(blocks: &[Pos], tile_map: &TileMap) -> bool {
    blocks.iter().all(|pos| {
        tile_map
            .get_tile(pos)
            .map_or(true, TileDefinition::is_breakable)
    })
}

fn tiles_to_side(
    depth: u8,
    steering: &Steering,
    coords: &Coords,
    bounds: &WorldBounds,
) -> Vec<Pos> {
    let facing_offset = if steering.facing.x.is_positive() {
        coords.dimens.x
    } else {
        -1
    };
    (0..(i32::from(depth)))
        .flat_map(|x| {
            (0..coords.dimens.y).map(move |y| (x, y)) //???
        })
        .map(|(x_offset, y_offset)| {
            Pos::new(
                coords.pos.x + facing_offset + x_offset * steering.facing.x.signum_i(),
                coords.pos.y + y_offset,
            )
        })
        .map(|pos| bounds.wrapped(&pos))
        .collect()
}

fn tiles_below(depth: u8, steering: &Steering, coords: &Coords, bounds: &WorldBounds) -> Vec<Pos> {
    let facing_offset = if steering.facing.x.is_positive() {
        coords.dimens.x - 1
    } else {
        0
    };
    (0..coords.dimens.x)
        .flat_map(|x| (0..(i32::from(depth))).map(move |y| (x, y)))
        .map(|(x_offset, y_offset)| {
            Pos::new(
                coords.pos.x + facing_offset + x_offset * steering.facing.x.signum_i(),
                coords.pos.y - 1 - y_offset,
            )
        })
        .map(|pos| bounds.wrapped(&pos))
        .collect()
}
