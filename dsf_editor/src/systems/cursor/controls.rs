use crate::components::cursor::Cursor;
use crate::resources::level_edit::LevelEdit;
use crate::resources::status::editor_status::EditorStatus;
use crate::systems::refresh_previews::RefreshPreviewsEvent;
use bevy::prelude::*;
use dsf_core::config::editor_config::EditorConfig;
use dsf_core::systems::motion::structs::direction::Direction2D;

/// Responsible for moving the cursor across the screen and managing its blinking animation.
pub fn cursor_controls(
    mut channel: EventWriter<RefreshPreviewsEvent>,
    mut keys: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<EditorConfig>,
    mut status: ResMut<EditorStatus>,
    mut level_edit: ResMut<LevelEdit>,
    mut query: Query<(&mut Cursor, &mut Transform)>,
) {
    for (mut cursor, mut transform) in query.iter_mut() {
        let adjust_bounds = keys.pressed(KeyCode::LAlt);
        let shift = keys.pressed(KeyCode::LShift);
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
        let down = keys.any_pressed([KeyCode::S, KeyCode::Down]);
        let up = keys.any_pressed([KeyCode::W, KeyCode::Up]);
        let new_direction = Direction2D::from_input(left, right, down, up);
        let should_move = if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
            // Start movement now. Move once and set cooldown to High.
            cursor.movement_cooldown = config.cursor_move_high_cooldown;
            true
        } else if cursor.last_direction.is_opposite(&new_direction) {
            // Reset movement. Set cooldown to high.
            cursor.movement_cooldown = config.cursor_move_high_cooldown;
            false
        } else if new_direction.is_neutral() {
            false
        } else {
            // continue movement. Tick down cooldown.
            // If cooldown is due, move once and reset cooldown to Low.
            cursor.movement_cooldown -= time.delta_seconds();
            if cursor.movement_cooldown.is_sign_negative() {
                cursor.movement_cooldown = config.cursor_move_low_cooldown;
                true
            } else {
                false
            }
        };
        cursor.last_direction = new_direction;
        let old_cursor_pos = status.selection.end;
        let mut received_user_input_to_move_cursor = should_move;
        if keys.clear_just_pressed(KeyCode::Home) {
            status.selection.end.x = level_edit.bounds().x();
            received_user_input_to_move_cursor = true;
        }
        if keys.clear_just_pressed(KeyCode::End) {
            status.selection.end.x = level_edit.bounds().upper_x() - 1;
            received_user_input_to_move_cursor = true;
        }
        if keys.clear_just_pressed(KeyCode::PageDown) {
            status.selection.end.y = level_edit.bounds().y();
            received_user_input_to_move_cursor = true;
        }
        if keys.clear_just_pressed(KeyCode::PageUp) {
            status.selection.end.y = level_edit.bounds().upper_y() - 1;
            received_user_input_to_move_cursor = true;
        }
        if should_move {
            if adjust_bounds {
                level_edit
                    .bounds_mut()
                    .adjust_x(status.selection.end.x, new_direction.x.signum_i());
                level_edit
                    .bounds_mut()
                    .adjust_y(status.selection.end.y, new_direction.y.signum_i());
            }
            status.selection.end.x += new_direction.x.signum_i();
            status.selection.end.y += new_direction.y.signum_i();
        }
        status.selection.end = level_edit.bounds().clamp(&status.selection.end);
        if old_cursor_pos != status.selection.end {
            channel.send(RefreshPreviewsEvent);
        }
        if received_user_input_to_move_cursor {
            reset_blink(&mut cursor, &config);
            if !shift {
                status.selection.start = status.selection.end;
            }
        }
        transform.translation.x = status.selection.end.x as f32 + 0.5;
        transform.translation.y = status.selection.end.y as f32 + 0.5;
    }
}

/// Resets the blinking cooldown, which ensures that the cursor stays visible.
/// Use when the cursor moves, so it is never invisible while the user is actively using it.
fn reset_blink(cursor: &mut Cursor, config: &EditorConfig) {
    if cursor.is_visible {
        cursor.blink_cooldown = config.cursor_blink_on_time;
    } else {
        cursor.blink_cooldown = 0.0;
    }
}
