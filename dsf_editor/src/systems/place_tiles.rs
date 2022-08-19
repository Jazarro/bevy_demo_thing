use crate::resources::blueprint::Blueprint;
use crate::resources::level_edit::LevelEdit;
use crate::resources::status::editor_status::EditorStatus;
use crate::systems::refresh_previews::RefreshPreviewsEvent;
use bevy::prelude::*;

/// Responsible for placing and removing tiles based on player input.
pub fn place_tiles(
    mut channel: EventWriter<RefreshPreviewsEvent>,
    mut keys: ResMut<Input<KeyCode>>,
    status: Res<EditorStatus>,
    mut level_edit: ResMut<LevelEdit>,
) {
    if keys.any_just_pressed([KeyCode::Return, KeyCode::NumpadEnter]) {
        keys.reset(KeyCode::Return);
        keys.reset(KeyCode::NumpadEnter);
        let blueprint = Blueprint::from_placing_tiles(&status, &level_edit);
        let lower_bounds = status.selection.lower_bounds();
        blueprint.tiles.iter().for_each(|(relative_pos, tile)| {
            level_edit.place_tile(
                status.force_place,
                lower_bounds + *relative_pos,
                Some(tile.clone()),
            );
        });
        channel.send(RefreshPreviewsEvent);
    }
    if keys.any_just_pressed([KeyCode::Delete]) {
        keys.reset(KeyCode::Delete);
        let lower_bounds = status.selection.lower_bounds();
        let selection_dimens = status.selection.dimens();
        (0..selection_dimens.x).for_each(|x| {
            (0..selection_dimens.y).for_each(|y| {
                level_edit.place_tile(true, lower_bounds.append_xy(x, y), None);
            });
        });
        channel.send(RefreshPreviewsEvent);
    }
}
