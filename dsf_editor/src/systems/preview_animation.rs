use crate::components::cursor::PreviewGhostTag;
use bevy::prelude::*;
use std::f32::consts;

/// Responsible for animating the cursor previews (IE the ghostly outlines of the blocks that
/// would get placed if the user would press 'place' at that time).
pub fn animate_previews(time: Res<Time>, mut query: Query<&mut Transform, With<PreviewGhostTag>>) {
    for mut transform in query.iter_mut() {
        let scale_factor = 1.
            - 0.1
                * (time.seconds_since_startup() as f32 * consts::PI)
                    .sin()
                    .abs();
        transform.scale = Vec3::new(scale_factor, scale_factor, 1.0);
    }
}
