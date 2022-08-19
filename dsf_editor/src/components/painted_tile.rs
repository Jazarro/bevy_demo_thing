use bevy::prelude::*;
use dsf_core::systems::motion::structs::pos::Pos;
use serde::{Deserialize, Serialize};

/// The entity with this component is the graphical representation of a tile in the `LevelEdit`
/// resource. It has a position by which one can look up the corresponding Tile in the `LevelEdit`.
#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PaintedTile {
    pub pos: Pos,
}

impl PaintedTile {
    #[must_use]
    pub fn new(pos: Pos) -> Self {
        PaintedTile { pos }
    }
}