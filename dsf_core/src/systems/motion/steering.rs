use bevy::prelude::{EventWriter, Query, Res, ResMut, Time, Transform, Without};

use crate::audio::sound_event::SoundEvent;
use crate::levels::tiles::tile_defs::TileDefinition;
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::loading::assets::SoundType;
use crate::systems::death::death_anim::Dying;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::direction::{Direction1D, Direction2D};
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;
use crate::systems::motion::structs::steering_mode::SteeringMode;
use crate::systems::rewind::structs::{Frame, History};

pub fn steering_system(
    tile_map: Res<TileMap>,
    mut history: ResMut<History>,
    time: Res<Time>,
    mut audio: EventWriter<SoundEvent>,
    mut query: Query<
        (
            &mut SteeringIntent,
            &mut Transform,
            &mut Steering,
            &mut Coords,
        ),
        Without<Dying>,
    >,
) {
    for (mut intent, mut transform, mut steering, mut coords) in query.iter_mut() {
        let old_pos = coords.pos;
        let (anchored_x, anchored_y) = coords.to_anchor_coords(&transform);
        coords.pos = Pos::new(anchored_x.round() as i32, anchored_y.round() as i32);
        wrap(
            &tile_map.world_bounds,
            &mut steering,
            &mut coords,
            &mut transform,
        );

        if steering.is_mid_air() {
            steering.mode = steering.mode.add_to_duration(time.delta_seconds());
        }

        if steering.is_grounded() && !intent.face.is_neutral() {
            steering.facing.x = intent.face;
        }

        // The following if-else construction checks if the steering mode should be changed.
        let has_ground_beneath_feet = is_grounded(&coords, &tile_map);
        if steering.is_falling()
            && anchored_y <= coords.pos.y as f32
            && has_ground_beneath_feet
            && on_solid_ground(&coords, &tile_map)
        {
            // If falling and you reached the floor, set to grounded.
            steering.mode = SteeringMode::Grounded;
            steering.destination = coords.pos;
        } else if (steering.is_grounded()
            && !has_ground_beneath_feet
            && aligned_with_grid(steering.destination.x as f32, anchored_x, intent.walk))
            || (steering.is_climbing() && intent.jump)
        {
            steering.mode = SteeringMode::Falling {
                x_movement: Direction1D::Neutral,
                starting_y_pos: transform.translation.y,
                duration: 0.,
            };
        } else if steering.is_grounded() && intent.jump {
            if is_underneath_ceiling(&coords, &tile_map) {
                audio.send(SoundEvent::Sfx(SoundType::CannotPerformAction, false));
            } else {
                audio.send(SoundEvent::Sfx(SoundType::Jump, false));
                steering.mode = SteeringMode::Jumping {
                    x_movement: intent.face,
                    starting_y_pos: transform.translation.y,
                    duration: 0.,
                };
            }
        } else if steering.jump_has_peaked() {
            steering.mode = steering.mode.jump_to_fall();
        } else if steering.is_grounded()
            && aligned_with_grid(steering.destination.x as f32, anchored_x, intent.walk)
            && ((intent.climb.is_positive() && can_climb_up(&coords, &tile_map))
                || (intent.climb.is_negative() && can_climb_down(&coords, &tile_map)))
        {
            steering.mode = SteeringMode::Climbing;
            if !intent.walk.is_neutral() {
                intent.walk_invalidated = true;
            }
        } else if steering.is_climbing()
            && aligned_with_grid(steering.destination.y as f32, anchored_y, intent.climb)
            && !intent.walk_invalidated
            && ((intent.walk.is_positive()
                && !is_against_wall_right(&coords, coords.pos.y as f32, &tile_map))
                || (intent.walk.is_negative()
                    && !is_against_wall_left(&coords, coords.pos.y as f32, &tile_map)))
        {
            steering.mode = SteeringMode::Grounded;
        }

        // This match will adjust the steering based on the current steering mode.
        match steering.mode {
            SteeringMode::Grounded => {
                if !intent.walk.is_neutral() {
                    steering.facing = Direction2D::from(intent.walk, Direction1D::Neutral);
                    let offset_from_destination = steering.destination.x as f32 - anchored_x;
                    if offset_from_destination < f32::EPSILON && intent.walk.is_positive() {
                        if !is_against_wall_right(&coords, coords.pos.y as f32, &tile_map) {
                            steering.destination.x = coords.pos.x + 1;
                            audio.send(SoundEvent::Sfx(SoundType::Step, false));
                        }
                    } else if offset_from_destination > -f32::EPSILON && intent.walk.is_negative() {
                        if !is_against_wall_left(&coords, coords.pos.y as f32, &tile_map) {
                            steering.destination.x = coords.pos.x - 1;
                            audio.send(SoundEvent::Sfx(SoundType::Step, false));
                        }
                    } else if !intent
                        .walk
                        .aligns_with((steering.destination.x - coords.pos.x) as f32)
                    {
                        // TODO: Maybe remove, this doesn't seem to do anything.
                        // Player wants to go back where they came from.
                        steering.destination.x = coords.pos.x;
                    }
                }
            }
            SteeringMode::Climbing => {
                if !intent.climb.is_neutral() {
                    steering.facing = Direction2D::from(Direction1D::Neutral, intent.climb);
                    let offset_from_discrete_pos = steering.destination.y as f32 - anchored_y;
                    if offset_from_discrete_pos < f32::EPSILON && intent.climb.is_positive() {
                        if can_climb_up(&coords, &tile_map) {
                            audio.send(SoundEvent::Sfx(SoundType::LadderStep, false));
                            steering.destination.y = coords.pos.y + 1;
                        } else {
                            steering.mode = SteeringMode::Grounded;
                        }
                    } else if offset_from_discrete_pos > -f32::EPSILON && intent.climb.is_negative()
                    {
                        if can_climb_down(&coords, &tile_map) {
                            audio.send(SoundEvent::Sfx(SoundType::LadderStep, false));
                            steering.destination.y = coords.pos.y - 1;
                        } else if above_air(&coords, &tile_map) {
                            steering.mode = SteeringMode::Falling {
                                x_movement: Direction1D::Neutral,
                                starting_y_pos: transform.translation.y,
                                duration: 0.,
                            };
                        } else {
                            steering.mode = SteeringMode::Grounded;
                        }
                    } else if !intent
                        .climb
                        .aligns_with((steering.destination.y - coords.pos.y) as f32)
                    {
                        // TODO: Maybe remove, this doesn't seem to do anything.
                        // Player wants to go back where they came from.
                        steering.destination.y = coords.pos.y;
                    }
                }
            }
            SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration,
            } => {
                if x_movement.is_neutral() {
                    // No horizontal movement.
                    steering.destination.x = coords.pos.x;
                } else if x_movement.is_positive() {
                    // Moving towards the right.
                    if is_against_wall_right(&coords, anchored_y, &tile_map) {
                        steering.mode = SteeringMode::Falling {
                            x_movement: Direction1D::Neutral,
                            starting_y_pos,
                            duration,
                        };
                    } else if aligned_with_grid(
                        steering.destination.x as f32,
                        anchored_x,
                        x_movement,
                    ) {
                        steering.destination.x = coords.pos.x + 1;
                    }
                } else {
                    // Moving towards the left.
                    if is_against_wall_left(&coords, anchored_y, &tile_map) {
                        steering.mode = SteeringMode::Falling {
                            x_movement: Direction1D::Neutral,
                            starting_y_pos,
                            duration,
                        };
                    } else if aligned_with_grid(
                        steering.destination.x as f32,
                        anchored_x,
                        x_movement,
                    ) {
                        steering.destination.x = coords.pos.x - 1;
                    }
                }
            }
            SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration,
            } => {
                if !intent.jump_direction.is_neutral() {
                    steering.mode = SteeringMode::Jumping {
                        x_movement: intent.jump_direction,
                        starting_y_pos,
                        duration,
                    };
                    steering.facing =
                        Direction2D::from(intent.jump_direction, Direction1D::Neutral);
                }
                if x_movement.is_neutral() {
                    // No horizontal movement.
                    steering.destination.x = coords.pos.x;
                } else if x_movement.is_positive() {
                    // Moving towards the right.
                    if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                        && !is_against_wall_right(&coords, coords.pos.y as f32, &tile_map)
                    {
                        steering.destination.x = coords.pos.x + 1;
                    }
                } else {
                    // Moving towards the left.
                    if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                        && !is_against_wall_left(&coords, coords.pos.y as f32, &tile_map)
                    {
                        steering.destination.x = coords.pos.x - 1;
                    }
                }
            }
        };

        // Push frame on history if player position changed.
        if old_pos != coords.pos || history.force_key_frame {
            history.push_frame(Frame::new(coords.pos));
        }
    }
}

/// Returns true iff the player is aligned with the grid.
/// This function can be used for both horizontal and vertical coordinates.
fn aligned_with_grid(destination_pos: f32, actual_pos: f32, input: Direction1D) -> bool {
    let offset = actual_pos - destination_pos;
    // Actual pos equal or greater than destination. Moving towards the positive.
    (offset > -f32::EPSILON && input.is_positive())
        // Actual pos equal or smaller than destination. Moving towards the negative.
        || (offset < f32::EPSILON && input.is_negative())
        // Actual pos basically equal to the destination.
        || (offset.abs() < f32::EPSILON)
}

/// You cannot jump onto the middle of a ladder, so use this function to check if you
/// should set steering to Grounded.
/// Returns true iff entity is on solid ground; meaning the very top of a ladder or a proper,
/// solid block that is not climbable.
///
/// This definition excludes the middle of a ladder. While the middle of a ladder can be walked on,
/// it cannot be landed on from a jump or fall.
fn on_solid_ground(coords: &Coords, tile_map: &TileMap) -> bool {
    (0..coords.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(coords.pos.x + i, coords.pos.y - 1));
        let tile_above = tile_map.get_tile(&Pos::new(coords.pos.x + i, coords.pos.y));
        tile.map_or(false, |tile| {
            tile.provides_platform()
                && (!tile.climbable || !tile_above.map_or(false, |tile_above| tile_above.climbable))
        })
    })
}

fn is_grounded(coords: &Coords, tile_map: &TileMap) -> bool {
    (0..coords.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(coords.pos.x + i, coords.pos.y - 1));
        tile.map_or(false, TileDefinition::provides_platform)
    })
}

/// The player cannot jump when underneath a 2-high ceiling.
/// This function returns true iff the player is underneath a 2-high ceiling.
fn is_underneath_ceiling(coords: &Coords, tile_map: &TileMap) -> bool {
    (0..coords.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(coords.pos.x + i, coords.pos.y + coords.dimens.y));
        tile.map_or(false, TileDefinition::collides_bottom)
    })
}

pub fn is_against_wall_left(coords: &Coords, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(coords, anchored_y, tile_map, -1, 0)
}

pub fn is_against_wall_right(coords: &Coords, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(
        coords,
        anchored_y,
        tile_map,
        coords.dimens.x,
        coords.dimens.x - 1,
    )
}

fn is_against_wall(
    coords: &Coords,
    anchored_y: f32,
    tile_map: &TileMap,
    x_offset: i32,
    x_offset_for_tile_in_front: i32,
) -> bool {
    let floored_y = anchored_y.floor();
    let nr_blocks_to_check = if (floored_y - anchored_y).abs() > f32::EPSILON {
        coords.dimens.y + 1
    } else {
        coords.dimens.y
    };
    (0..nr_blocks_to_check).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(coords.pos.x + x_offset, floored_y as i32 + i));
        let tile_in_front = tile_map.get_tile(&Pos::new(
            coords.pos.x + x_offset_for_tile_in_front,
            floored_y as i32 + i,
        ));
        tile.map_or(false, |tile| {
            tile.collides_horizontally()
                && tile_in_front
                    .map_or(true, |tile_in_front| !tile_in_front.collides_horizontally())
        })
    })
    // TODO: This tile_in_front seems to be a way to allow you to walk through walls
    //  if you're already inside one. It seems to just be a patch to cover up other bugs,
    //  and should maybe be removed.
}

fn can_climb_up(coords: &Coords, tile_map: &TileMap) -> bool {
    can_climb(coords, tile_map, (0, 1)) && !is_underneath_ceiling(coords, tile_map)
}

fn can_climb_down(coords: &Coords, tile_map: &TileMap) -> bool {
    can_climb(coords, tile_map, (-1, 0))
}

fn can_climb(coords: &Coords, tile_map: &TileMap, y_range: (i32, i32)) -> bool {
    (0..coords.dimens.x).all(|x_offset| {
        (y_range.0..y_range.1).all(|y_offset| {
            let tile =
                tile_map.get_tile(&Pos::new(coords.pos.x + x_offset, coords.pos.y + y_offset));
            tile.map_or(false, |tile| tile.climbable)
        })
    })
}

fn above_air(coords: &Coords, tile_map: &TileMap) -> bool {
    (0..coords.dimens.x).all(|x_offset| {
        let tile = tile_map.get_tile(&Pos::new(coords.pos.x + x_offset, coords.pos.y - 1));
        tile.map_or(false, |tile| tile.climbable)
    })
}

fn wrap(
    bounds: &WorldBounds,
    steering: &mut Steering,
    coords: &mut Coords,
    transform: &mut Transform,
) {
    let delta = Pos::new(
        if coords.pos.x < bounds.x() && steering.facing.x == Direction1D::Negative {
            bounds.width()
        } else if (coords.pos.x + coords.dimens.x) > bounds.upper_x()
            && steering.facing.x == Direction1D::Positive
        {
            -bounds.width()
        } else {
            0
        },
        if coords.pos.y < bounds.y()
            && (steering.mode != SteeringMode::Climbing
                || steering.facing.y == Direction1D::Negative)
        {
            bounds.height()
        } else if (coords.pos.y + coords.dimens.y) > bounds.upper_y()
            && (steering.mode != SteeringMode::Climbing
                || steering.facing.y == Direction1D::Positive)
        {
            -bounds.height()
        } else {
            0
        },
    );
    coords.pos = coords.pos + delta;
    steering.destination = steering.destination + delta;
    steering.mode = steering.mode.wrap(delta.y as f32);
    transform.translation.x += delta.x as f32;
    transform.translation.y += delta.y as f32;
}
