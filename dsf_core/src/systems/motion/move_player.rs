use bevy::prelude::*;

use crate::config::movement_config::MovementConfig;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;

/// Sets the player intention to move.
pub fn set_player_steering_intent(
    mut query: Query<(&mut Player, &mut SteeringIntent, &Steering, &Coords)>,
    keys: Res<Input<KeyCode>>,
    config: Res<MovementConfig>,
    time: Res<Time>,
) {
    let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
    let down = keys.any_pressed([KeyCode::S, KeyCode::Down]);
    let up = keys.any_pressed([KeyCode::W, KeyCode::Up]);
    let jump_pressed = keys.any_pressed([KeyCode::Space]);
    let new_walk = Direction1D::from_input(left, right);
    let new_climb = Direction1D::from_input(down, up);

    for (mut player, mut intent, steering, coords) in query.iter_mut() {
        if let Some(target) = intent.forced_walk {
            let direction = Direction1D::new((target.x - coords.pos.x) as f32);
            intent.walk = direction;
            return;
        }

        let initiate_jump = jump_pressed && !player.pressing_jump;
        player.pressing_jump = jump_pressed;
        player.jump_grace_timer = if initiate_jump {
            Some(0.)
        } else if let Some(time_passed) = player.jump_grace_timer {
            let time_passed = time_passed + time.delta_seconds();
            if time_passed < config.jump_allowance {
                Some(time_passed)
            } else {
                None
            }
        } else {
            None
        };
        let old_walk = intent.walk;
        let turn_around = steering.is_grounded()
            && steering.facing.x.is_opposite(&new_walk)
            && old_walk.is_neutral();
        player.turn_around_timer = if turn_around {
            // Player wants to turn around, initialise turn-around timer.
            Some(0.)
        } else if new_walk.is_neutral() {
            // Player has let go of controls, forcefully reset timer.
            None
        } else if let Some(time_passed) = player.turn_around_timer {
            let time_passed = time_passed + time.delta_seconds();
            if time_passed < config.turn_allowance {
                Some(time_passed)
            } else {
                None
            }
        } else {
            None
        };

        if player.turn_around_timer.is_none() {
            intent.walk = new_walk;
        }
        intent.face = new_walk;
        if intent.walk_invalidated && old_walk != intent.walk {
            intent.walk_invalidated = false;
        }
        intent.climb = new_climb;
        intent.jump = player.equipped.is_none() && initiate_jump;
        intent.jump_direction = if player.jump_grace_timer.is_some() {
            intent.walk
        } else {
            Direction1D::Neutral
        };
    }
}
