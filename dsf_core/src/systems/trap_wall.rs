use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::{AssetStorage, SoundType};
use crate::loading::entities::inflate::spawn_from_def;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::dimens::Dimens;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::pos::Pos;

const COOLDOWN: f32 = 1.;

#[derive(Component, Default)]
pub struct TrappedWall {
    /// Countdown to trigger the trap.
    pub timer: Option<Timer>,
}

pub fn trigger_trap_walls(
    tile_map: Res<TileMap>,
    query_player: Query<&Coords, With<Player>>,
    mut query_wall: Query<(&mut TrappedWall, &Coords)>,
) {
    if let Ok(player_coords) = query_player.get_single() {
        let mut vec: Vec<Pos> = tiles_inside(player_coords, &tile_map.world_bounds)
            .iter()
            .filter(|pos| tile_map.get_tile(pos).map_or(false, |def| def.is_trapped()))
            .map(|trap_pos| {
                let mut offset = 0;
                loop {
                    offset += 1;
                    let up_is_also_trapped = tile_map
                        .get_tile(&trap_pos.append_y(offset))
                        .map_or(false, |def| def.is_trapped());
                    if !up_is_also_trapped {
                        return trap_pos.append_y(offset - 1);
                    }
                }
            })
            .collect();
        vec.sort();
        vec.dedup();
        for (mut trap, _) in query_wall
            .iter_mut()
            .filter(|(trap, wall_coords)| vec.contains(&wall_coords.pos) && trap.timer.is_none())
        {
            trap.timer = Some(Timer::from_seconds(COOLDOWN, false));
        }
    }
}

// TODO: This was copied almost verbatim from tools. Get rid of duplicate code.
//          - It's also very similar to code in steering systems.
fn tiles_inside(coords: &Coords, bounds: &WorldBounds) -> Vec<Pos> {
    (0..coords.dimens.x)
        .flat_map(|x| (0..coords.dimens.y).map(move |y| (x, y)))
        .map(|(x_offset, y_offset)| Pos::new(coords.pos.x + x_offset, coords.pos.y + y_offset))
        .map(|pos| bounds.wrapped(&pos))
        .collect()
}

pub fn trap_mechanism(
    mut commands: Commands,
    time: Res<Time>,
    storage: Res<AssetStorage>,
    mut tile_map: ResMut<TileMap>,
    mut audio: EventWriter<SoundEvent>,
    mut query_trap: Query<(Entity, &mut TrappedWall, &Coords)>,
    query_player: Query<&Coords, With<Player>>,
) {
    if let Ok(player) = query_player.get_single() {
        let next = query_trap
            .iter_mut()
            .filter_map(|(entity, mut trap, coords)| {
                if let Some(timer) = &mut trap.timer {
                    timer.tick(time.delta());
                    if timer.finished() && !player.overlaps_pos(&coords.pos) {
                        audio.send(SoundEvent::Sfx(SoundType::TrapWallCreated, false));
                        commands.entity(entity).despawn_recursive();
                        spawn_from_def(
                            &mut commands,
                            &storage,
                            coords.pos,
                            tile_map.tile_defs.get("Block2"),
                        );
                        tile_map.put_tile(&coords.pos, Dimens::new(1, 1), "Block2".to_string());
                        let there_is_another_one = tile_map
                            .get_tile(&coords.pos.append_y(-1))
                            .map_or(false, |def| def.is_trapped());
                        if there_is_another_one {
                            Some(coords.pos.append_y(-1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Pos>>();

        for (_, mut trap, coords) in query_trap.iter_mut() {
            if next.contains(&coords.pos) {
                trap.timer = Some(Timer::from_seconds(COOLDOWN, false));
            }
        }
    }
}
