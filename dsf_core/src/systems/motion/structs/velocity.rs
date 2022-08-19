use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Velocity in meters per second.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
#[serde(deny_unknown_fields)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
