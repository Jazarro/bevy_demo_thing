use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::tiles::tilemap::TileMap;
use crate::loading::assets::SoundType;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::dimens::Dimens;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;

const TIME_ACTIVATION: f32 = 0.5;
const TIME_ANIMATION: f32 = 0.5;

#[derive(Component)]
pub struct RevolvingDoor {
    pub state: RevolvingState,
    /// The direction the player would be facing if they used the door.
    pub facing: Direction1D,
}

impl RevolvingDoor {
    pub fn new(direction: Direction1D) -> Self {
        RevolvingDoor {
            state: RevolvingState::Idle,
            facing: direction,
        }
    }
}

pub enum RevolvingState {
    Idle,
    TurningTowards,
}

/// Only the top door fragment is the controller and does all the logic etc.
#[derive(Component)]
pub struct RevolvingController {
    pub state: ControllerState,
    /// How many door fragments in this door.
    pub size: usize,
    /// The direction the player would be facing if they used the door.
    pub facing: Direction1D,
}

#[derive(Debug)]
pub enum ControllerState {
    Idle,
    Activation(Timer),
    Animation(Timer),
}

pub fn control_revolving_doors(
    mut tile_map: ResMut<TileMap>,
    mut audio: EventWriter<SoundEvent>,
    time: Res<Time>,
    mut query_ctrl: Query<(&mut RevolvingController, &Coords)>,
    mut query_door: Query<(&mut RevolvingDoor, &Coords)>,
    mut query_player: Query<(&mut SteeringIntent, &Steering, &Coords), With<Player>>,
) {
    if let Ok((mut intent, steering, coords)) = query_player.get_single_mut() {
        for (mut ctrl, ctrl_coords) in query_ctrl.iter_mut() {
            let player_in_position = coords.pos.x
                == ctrl_coords.pos.x + ctrl.facing.signum_i() * -2
                && coords.pos.y <= ctrl_coords.pos.y
                && coords.pos.y > ctrl_coords.pos.y - ctrl.size as i32;
            let player_walking_at_door = steering.is_grounded() && intent.walk == ctrl.facing;
            match &mut ctrl.state {
                ControllerState::Idle => {
                    if player_in_position && player_walking_at_door {
                        ctrl.state = ControllerState::Activation(Timer::from_seconds(
                            TIME_ACTIVATION,
                            false,
                        ));
                    }
                }
                ControllerState::Activation(timer) => {
                    timer.tick(time.delta());
                    if timer.finished() {
                        ctrl.state =
                            ControllerState::Animation(Timer::from_seconds(TIME_ANIMATION, false));
                        audio.send(SoundEvent::Sfx(SoundType::SpawnerOpenClose, false));
                        intent.forced_walk = Some(coords.pos.append_x(ctrl.facing.signum_i() * 4));
                        for (mut door, child_block) in
                            query_door.iter_mut().filter(|(_, child_block)| {
                                child_block.pos.x == ctrl_coords.pos.x
                                    && child_block.pos.y <= ctrl_coords.pos.y
                                    && child_block.pos.y > ctrl_coords.pos.y - ctrl.size as i32
                            })
                        {
                            door.state = RevolvingState::TurningTowards;
                            tile_map.put_tile(
                                &child_block.pos,
                                Dimens::new(2, 1),
                                "RevolvingDoorNonColliding".to_string(),
                            );
                        }
                    } else if !player_in_position || !player_walking_at_door {
                        ctrl.state = ControllerState::Idle;
                    }
                }
                ControllerState::Animation(timer) => {
                    timer.tick(time.delta());
                    if timer.finished() {
                        ctrl.state = ControllerState::Idle;
                        ctrl.facing = !ctrl.facing;
                        intent.forced_walk = None;
                        for (mut door, child_coords) in
                            query_door.iter_mut().filter(|(_, child_coords)| {
                                child_coords.pos.x == ctrl_coords.pos.x
                                    && child_coords.pos.y <= ctrl_coords.pos.y
                                    && child_coords.pos.y > ctrl_coords.pos.y - ctrl.size as i32
                            })
                        {
                            door.state = RevolvingState::Idle;
                            door.facing = ctrl.facing;
                            tile_map.put_tile(
                                &child_coords.pos,
                                Dimens::new(2, 1),
                                "RevolvingDoorFacingPositive".to_string(),
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn control_revolving_sprites(mut query: Query<(&RevolvingDoor, &mut TextureAtlasSprite)>) {
    for (door, mut sprite) in query.iter_mut() {
        sprite.flip_x = door.facing.is_negative();
        sprite.index = if matches!(door.state, RevolvingState::Idle) {
            0
        } else {
            1
        };
    }
}

pub fn set_revolving_controllers(
    mut commands: Commands,
    tile_map: Res<TileMap>,
    query: Query<(Entity, &Coords, &RevolvingDoor), Without<RevolvingController>>,
) {
    for (entity, coords, door) in query.iter().filter(|(_, block, _)| {
        // This filters out non-controllers.
        !tile_map
            .get_tile(&block.pos.append_y(1))
            .map_or(false, |def| def.is_revolving())
    }) {
        let count = (0..)
            .into_iter()
            .position(|i| {
                !tile_map
                    .get_tile(&coords.pos.append_y(-i))
                    .map_or(false, |def| def.is_revolving())
            })
            .unwrap();
        commands.entity(entity).insert(RevolvingController {
            state: ControllerState::Idle,
            size: count,
            facing: door.facing,
        });
    }
}
