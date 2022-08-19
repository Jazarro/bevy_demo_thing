use crate::components::selection::SelectionTag;
use crate::resources::status::editor_status::EditorStatus;
use bevy::prelude::*;
use dsf_core::levels::tiles::tile_defs::DepthLayer;
use std::cmp::min;

/// Responsible for managing the selection.
pub fn selection_system(
    status: Res<EditorStatus>,
    mut query: Query<&mut Transform, With<SelectionTag>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let width = (status.selection.start.x - status.selection.end.x).abs() + 1;
        let height = (status.selection.start.y - status.selection.end.y).abs() + 1;
        // TODO: set scale requires knowledge about dimensions of sprite.
        // Maybe solve with child entity.
        // Or accept hardcoded nature, because sprite unlikely to change?
        if width == 1 && height == 1 {
            transform.scale = Vec3::new(0., 0., 1.0);
        } else {
            transform.scale = Vec3::new(width as f32 / 128., height as f32 / 128., 1.0);
        }

        transform.translation = Vec3::new(
            (width as f32 * 0.5) + min(status.selection.start.x, status.selection.end.x) as f32,
            (height as f32 * 0.5) + min(status.selection.start.y, status.selection.end.y) as f32,
            (&DepthLayer::Selection).z(),
        );
    }
}
