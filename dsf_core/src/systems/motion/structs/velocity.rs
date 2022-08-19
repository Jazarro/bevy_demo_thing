use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Velocity in meters per second.
#[derive(Component, Deserialize, Serialize, Default, Clone, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
