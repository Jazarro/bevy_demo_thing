use bevy::prelude::*;

use crate::camera::camera_components::CameraFrame;

/// Initialise the camera.
pub fn create_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut bundle = Camera2dBundle::default();
    let window = get_primary_window_size(&windows);
    // info!("Window size is {:?}", window);
    let scale = 24. / window.y;
    bundle.transform.scale = Vec3::new(scale, scale, 1.);
    commands.spawn_bundle(bundle).insert(CameraFrame::default());
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}
