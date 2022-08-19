use crate::resources::level_edit::LevelEdit;
use crate::resources::status::editor_status::EditorStatus;
use crate::states::file_actions::auto_save;
use crate::systems::refresh_previews::RefreshPreviewsEvent;
use bevy::prelude::*;
use dsf_core::states::AppState;
use iyes_loopless::prelude::NextState;

/// Responsible for changing transient configurations for the editor. These settings stay alive
/// as long as the `EditorState` lives.
///
/// Currently, this system is responsible for:
///
/// - Changing what tile is on the brush.
/// - Toggling the copy-air flag.
/// - Toggling the force-place flag.
///
pub fn check_editor_input(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut writer: EventWriter<RefreshPreviewsEvent>,
    mut status: ResMut<EditorStatus>,
    level_edit: Res<LevelEdit>,
) {
    if keys.clear_just_pressed(KeyCode::LBracket) {
        let _new_key = status.brush.select_previous();
        writer.send(RefreshPreviewsEvent);
    }
    if keys.clear_just_pressed(KeyCode::RBracket) {
        let _new_key = status.brush.select_next();
        writer.send(RefreshPreviewsEvent);
    }
    if keys.clear_just_pressed(KeyCode::F) {
        status.force_place ^= true;
        writer.send(RefreshPreviewsEvent);
    }
    if keys.clear_just_pressed(KeyCode::G) {
        status.copy_air ^= true;
        writer.send(RefreshPreviewsEvent);
    }
    if keys.pressed(KeyCode::LControl) && keys.clear_just_pressed(KeyCode::S) {
        auto_save(&level_edit);
    }
    if keys.clear_just_pressed(KeyCode::F5) {
        auto_save(&level_edit);
        commands.insert_resource(NextState(AppState::InGame));
    }
}
