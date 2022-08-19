use bevy::prelude::*;
use bevy::window::WindowMode;

/// Handle some general behaviour related to the window that should be executed in any State.
pub fn handle_window(mut keys: ResMut<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let primary = windows.primary_mut();
    // Toggle fullscreen:
    if keys.clear_just_pressed(KeyCode::F11) {
        primary.set_mode(if primary.mode() != WindowMode::Windowed {
            WindowMode::Windowed
        } else {
            WindowMode::BorderlessFullscreen
        });
    }
}

/////Responds to window resize events. Recreates the camera with the new dimensions.
// fn resize_camera(world: &mut World) {
//     world.exec(
//         |data: (
//             Entities<'_>,
//             WriteStorage<'_, Camera>,
//             WriteStorage<'_, Transform>,
//             WriteStorage<'_, Parent>,
//             ReadStorage<'_, CameraFrame>,
//             ReadExpect<'_, ScreenDimensions>,
//         )| {
//             let (entities, mut cameras, mut transforms, mut parents, camera_frames, screen_dimens) =
//                 data;
//             let frame = (&*entities, &camera_frames)
//                 .join()
//                 .map(|(entity, _)| entity)
//                 .next();
//             let cam = (&*entities, &cameras)
//                 .join()
//                 .map(|(entity, _)| entity)
//                 .next();
//             if let Some(frame) = frame {
//                 if let Some(cam) = cam {
//                     entities
//                         .delete(cam)
//                         .expect("Trying to resize, but failed to delete camera.");
//                 }
//                 if screen_dimens.x() > f32::EPSILON && screen_dimens.y() > f32::EPSILON {
//                     entities
//                         .build_entity()
//                         .with(Parent { entity: frame }, &mut parents)
//                         .with(
//                             Camera::standard_2d(screen_dimens.x(), screen_dimens.y()),
//                             &mut cameras,
//                         )
//                         .with(Transform::default(), &mut transforms)
//                         .build();
//                 }
//             }
//         },
//     );
// }
