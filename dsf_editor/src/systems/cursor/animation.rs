use crate::components::cursor::Cursor;
use bevy::prelude::*;
use dsf_core::config::editor_config::EditorConfig;

/// Tick down the blinking cooldown and take care of the the blinking animation if the cooldown is
/// ready.
pub fn perform_blinking_animation(
    mut query: Query<(&mut Cursor, &mut Transform)>,
    time: Res<Time>,
    config: Res<EditorConfig>,
) {
    if let Ok((mut cursor, mut transform)) = query.get_single_mut() {
        if cursor.blink_cooldown.is_sign_negative() {
            cursor.is_visible ^= true;
            cursor.blink_cooldown = if cursor.is_visible {
                config.cursor_blink_on_time
            } else {
                config.cursor_blink_off_time
            };
            let scale_factor = if cursor.is_visible { 1.0 / 128. } else { 0.0 };
            transform.scale = Vec3::new(scale_factor, scale_factor, 1.0);
        }
        cursor.blink_cooldown -= time.delta_seconds();
    }
}
