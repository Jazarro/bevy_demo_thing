use crate::components::cursor::Cursor;
use crate::components::selection::SelectionTag;
use bevy::prelude::*;
use dsf_core::camera::camera_components::FocalPoint;
use dsf_core::levels::tiles::tile_defs::DepthLayer;

/// Adds a selection and a cursor entity.
pub fn init_cursor(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 1., 0.25, 0.1),
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., DepthLayer::Cursor.z()).with_scale(Vec3::new(
                1. / 128.,
                1. / 128.,
                1.,
            )),
            ..default()
        })
        .insert(FocalPoint)
        .insert(Cursor::default());

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0.25, 1., 0.3),
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., DepthLayer::Selection.z()),
            ..default()
        })
        .insert(SelectionTag);
}
