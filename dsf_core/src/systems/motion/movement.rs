use bevy::prelude::*;

use crate::config::movement_config::MovementConfig;
use crate::systems::death::death_anim::Dying;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_mode::SteeringMode;
use crate::systems::motion::structs::velocity::Velocity;

/// For every entity with a velocity and a transform, updates the transform according to the
/// velocity.
pub fn velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), Without<Dying>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x = transform.translation.x + time.delta_seconds() * velocity.x;
        transform.translation.y = transform.translation.y + time.delta_seconds() * velocity.y;
    }
}

/// Sets velocity for all entities with steering.
pub fn movement_system(
    config: Res<MovementConfig>,
    mut query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &Steering,
            &mut Velocity,
        ),
        Without<Dying>,
    >,
) {
    for (mut transform, mut sprite, steering, mut velocity) in query.iter_mut() {
        // Flip sprite if character is facing left:
        sprite.flip_x = steering.facing.x == Direction1D::Positive;

        let (centered_x, centered_y) = steering.to_centered_coords(steering.pos);
        let (desired_pos_x, desired_pos_y) = steering.to_centered_coords(steering.destination);
        match steering.mode {
            SteeringMode::Grounded => {
                // If grounded, correct y translation and zero out y velocity.
                transform.translation.y = centered_y;
                velocity.y = 0.0;
            }
            SteeringMode::Climbing => {
                // If climbing, correct x translation and zero out x velocity.
                transform.translation.x = centered_x;
                velocity.x = 0.0;
                // If climbing:
                let delta = desired_pos_y - transform.translation.y;
                if steering.facing.y.aligns_with(delta) {
                    velocity.y = steering.facing.y.signum() * config.player_speed;
                } else {
                    velocity.y = 0.0;
                    transform.translation.y = centered_y;
                }
            }
            SteeringMode::Falling {
                starting_y_pos,
                duration,
                ..
            } => {
                // Set y-position directly, based on movement function. We don't use velocity for this.
                velocity.y = 0.0;
                transform.translation.y = starting_y_pos + steering.mode.calc_delta_y(duration);
            }
            SteeringMode::Jumping {
                starting_y_pos,
                duration,
                ..
            } => {
                // Set y-position directly, based on movement function. We don't use velocity for this.
                velocity.y = 0.0;
                transform.translation.y = starting_y_pos + steering.mode.calc_delta_y(duration);
            }
        }

        // Set x-velocity based on current and desired position.
        // If necessary, adjust x-position, snap to grid.
        let delta = desired_pos_x - transform.translation.x;
        if steering.facing.x.aligns_with(delta) {
            velocity.x = steering.facing.x.signum() * config.player_speed;
        } else {
            velocity.x = 0.0;
            transform.translation.x = centered_x;
        }
    }
}
