use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The blue background sprite.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct BackgroundTag;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct BackgroundHeads {
    pub anim: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct BackgroundEyes {
    pub anim: f32,
}
