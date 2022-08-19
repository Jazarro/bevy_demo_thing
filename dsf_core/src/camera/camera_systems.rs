use bevy::prelude::*;

use crate::camera::camera_components::{CameraFrame, FocalPoint};
use crate::systems::motion::structs::direction::Direction2D;

/// This system handles player input to control certain aspects of the camera.
/// Specifically: camera panning, camera zoom.
pub fn camera_control(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut frame_query: Query<&mut CameraFrame>,
) {
    // let _zoom = input.axis_value("zoom").unwrap_or(0.0);
    let left = keys.any_pressed([KeyCode::J]);
    let right = keys.any_pressed([KeyCode::L]);
    let up = keys.any_pressed([KeyCode::I]);
    let down = keys.any_pressed([KeyCode::K]);
    let new_direction = Direction2D::from_input(left, right, down, up);
    let mut frame = frame_query.single_mut();
    if new_direction.is_neutral() {
        if frame.pan.length_squared() > f32::EPSILON {
            // Recovery (jump back to zero pan)
            let pan_add =
                frame.pan.normalize() * frame.panning_recovery_speed * time.delta_seconds() * -1.;
            if pan_add.length() >= frame.pan.length() {
                frame.pan = Vec2::new(0., 0.);
            } else {
                frame.pan += pan_add;
            }
        }
    } else {
        let pan_add = new_direction.signum() * frame.panning_speed * time.delta_seconds();
        frame.pan += pan_add;
        frame.pan = Vec2::new(
            frame.pan.x.clamp(-frame.max_pan, frame.max_pan),
            frame.pan.y.clamp(-frame.max_pan, frame.max_pan),
        );
    }
}

/// This system updates the camera frame position to center on the player's position.
pub fn camera_follow_focal_point(
    mut set: ParamSet<(
        Query<&Transform, With<FocalPoint>>,
        Query<(&mut Transform, &CameraFrame)>,
    )>,
) {
    let target_pos = if let Ok(transform) = set.p0().get_single() {
        transform.translation
    } else {
        error!("There is not exactly one focal point for the camera to focus on.");
        Vec3::default()
    };
    let mut cam_query = set.p1();
    let (mut transform, frame) = cam_query.single_mut();
    transform.translation.x = target_pos.x + frame.pan.x;
    transform.translation.y = target_pos.y + frame.pan.y;
}
