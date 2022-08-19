use bevy::prelude::*;

use crate::config::settings::debug_settings::DebugSettings;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::rewind::structs::{CurrentState, History, Rewind};

pub fn rewind_control_system(
    keys: Res<Input<KeyCode>>,
    mut current_state: ResMut<CurrentState>,
    mut rewind: ResMut<Rewind>,
    mut history: ResMut<History>,
    time: Res<Time>,
    config: Res<DebugSettings>,
) {
    history.force_key_frame = false;
    if keys.pressed(KeyCode::LShift) {
        rewind.cooldown = match *current_state {
            CurrentState::Running => config.seconds_per_rewind_frame,
            CurrentState::Rewinding => {
                if rewind.is_ready() {
                    rewind.cooldown + config.seconds_per_rewind_frame
                } else {
                    rewind.cooldown - time.delta_seconds()
                }
            }
        };
        *current_state = CurrentState::Rewinding;
    } else {
        if CurrentState::Rewinding == *current_state {
            history.force_key_frame = true;
        }
        *current_state = CurrentState::Running;
    }
}

pub fn rewind_system(
    rewind: Res<Rewind>,
    mut history: ResMut<History>,
    mut query: Query<(&mut Transform, &mut Steering, &mut Coords), With<Player>>,
) {
    if rewind.is_ready() {
        if let Some(frame) = history.pop_frame() {
            info!("Rewinding player to {:?}", frame);
            for (mut transform, mut steering, mut coords) in query.iter_mut() {
                let (centered_x, centered_y) = coords.to_centered_coords(frame.player_position);
                transform.translation.x = centered_x;
                transform.translation.y = centered_y;
                coords.pos = frame.player_position;
                steering.destination = frame.player_position;
            }
        }
    }
}
