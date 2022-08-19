use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::load_level_system::load_transform;
use crate::levels::tiles::objects::Block;
use crate::levels::tiles::tile_defs::DepthLayer;
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::{AssetStorage, SoundType, SpriteType};
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;

const COOLDOWN: f32 = 1.;

#[derive(Component)]
pub struct TrappedWall {
    pub pos: Pos,
    /// Countdown to trigger the trap.
    pub timer: Option<Timer>,
}

impl TrappedWall {
    pub fn new(pos: Pos) -> Self {
        TrappedWall { pos, timer: None }
    }
}

pub fn trigger_trap_walls(
    tile_map: Res<TileMap>,
    query_player: Query<&Steering, With<Player>>,
    mut query_wall: Query<&mut TrappedWall>,
) {
    if let Ok(steering) = query_player.get_single() {
        let mut vec: Vec<Pos> = tiles_inside(steering, &tile_map.world_bounds)
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
        for mut trap in query_wall
            .iter_mut()
            .filter(|trap| vec.contains(&trap.pos) && trap.timer.is_none())
        {
            trap.timer = Some(Timer::from_seconds(COOLDOWN, false));
        }
    }
}

// TODO: This was copied almost verbatim from tools. Get rid of duplicate code.
//          - It's also very similar to code in steering systems.
fn tiles_inside(steering: &Steering, bounds: &WorldBounds) -> Vec<Pos> {
    (0..steering.dimens.x)
        .flat_map(|x| (0..steering.dimens.y).map(move |y| (x, y)))
        .map(|(x_offset, y_offset)| Pos::new(steering.pos.x + x_offset, steering.pos.y + y_offset))
        .map(|pos| bounds.wrapped(&pos))
        .collect()
}

pub fn trap_mechanism(
    mut commands: Commands,
    time: Res<Time>,
    storage: Res<AssetStorage>,
    mut tile_map: ResMut<TileMap>,
    mut audio: EventWriter<SoundEvent>,
    mut query_trap: Query<(Entity, &mut TrappedWall)>,
    query_player: Query<&Steering, With<Player>>,
) {
    if let Ok(player) = query_player.get_single() {
        let next = query_trap
            .iter_mut()
            .map(|(entity, mut trap)| {
                if let Some(timer) = &mut trap.timer {
                    timer.tick(time.delta());
                    if timer.finished() && !player.overlaps_pos(&trap.pos) {
                        audio.send(SoundEvent::Sfx(SoundType::TrapWallCreated, false));
                        commands.entity(entity).despawn_recursive();
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: storage.get_atlas(&SpriteType::Blocks),
                                transform: load_transform(
                                    &trap.pos,
                                    &IVec2::new(1, 1),
                                    &DepthLayer::Blocks,
                                ),
                                sprite: TextureAtlasSprite {
                                    index: 1,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Block { pos: trap.pos });
                        tile_map.put_tile(&trap.pos, "Block2".to_string(), IVec2::new(1, 1));

                        let there_is_another_one = tile_map
                            .get_tile(&trap.pos.append_y(-1))
                            .map_or(false, |def| def.is_trapped());
                        if there_is_another_one {
                            Some(trap.pos.append_y(-1))
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
            .flatten()
            .collect::<Vec<Pos>>();

        for (_, mut trap) in query_trap.iter_mut() {
            if next.contains(&trap.pos) {
                trap.timer = Some(Timer::from_seconds(COOLDOWN, false));
            }
        }
    }
}
