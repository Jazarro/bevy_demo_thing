use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The camera will be a child entity of the camera frame.
///
/// The camera frame will maintain the rough position of the camera. Usually this will be the
/// player's position.
///
/// The camera itself will maintain an offset position. Usually this will be at the origin
/// (no offset). If there is camera shake, that will be done through this offset.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Component)]
pub struct CameraFrame {
    /// Player will be able to pan the camera around to a limited degree.
    /// This is the current offset from the camera's default position.
    pub pan: Vec2,
    /// The maximum distance in meters that the player can pan the camera.
    pub max_pan: f32,
    /// Speed at which the camera may pan, in meters per second.
    pub panning_speed: f32,
    /// Speed at which the camera may pan back to its default position after the player lets go
    /// of the panning controls. This will be faster than the speed at which the player can pan the
    /// camera around, resulting in a sort of rubber banding effect.
    pub panning_recovery_speed: f32,
}

impl Default for CameraFrame {
    fn default() -> Self {
        CameraFrame {
            pan: Vec2::new(0., 0.),
            max_pan: 5.,
            panning_speed: 10.,
            panning_recovery_speed: 40.,
        }
    }
}

/// Should only be given to one entity. Typically given to the player.
/// The camera will follow the entity with this component.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct FocalPoint;
