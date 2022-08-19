use crate::resources::level_edit::LevelEdit;
use bevy::prelude::*;
use dsf_core::levels::tiles::background::BackgroundTag;
use dsf_core::levels::tiles::tile_defs::DepthLayer;

/// Responsible for updating the size and location of the background sprite whenever the
/// world bounds change.
pub fn update_background(
    level_edit: Res<LevelEdit>,
    mut query: Query<(&mut Transform, &mut Sprite), With<BackgroundTag>>,
) {
    for (mut transform, mut sprite) in query.iter_mut() {
        // TODO: set scale requires knowledge about dimensions of sprite.
        sprite.custom_size = Some(Vec2::new(
            level_edit.bounds().width() as f32,
            level_edit.bounds().height() as f32,
        ));
        transform.translation = Vec3::new(
            level_edit.bounds().x() as f32 + (level_edit.bounds().width() as f32 * 0.5),
            level_edit.bounds().y() as f32 + (level_edit.bounds().height() as f32 * 0.5),
            (&DepthLayer::Background).z(),
        );
    }
}
