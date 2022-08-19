use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::levels::tiles::tile_defs::ToolType;
use crate::loading::assets::SpriteType;
use crate::systems::motion::structs::pos::Pos;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct Key;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct Tool {
    pub tool_type: ToolType,
    pub sprite: SpriteType,
    pub sprite_nr: usize,
}

impl Tool {
    #[must_use]
    pub fn new(tool_type: ToolType, sprite: SpriteType, sprite_nr: usize) -> Self {
        Tool {
            tool_type,
            sprite,
            sprite_nr,
        }
    }
}

/// A miniature version of every key is found on the exit door.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct KeyDisplay {
    /// The position of the corresponding key in the world. NOT the actual position of this display.
    /// The display is a miniature version of the key located somewhere on top of the door.
    pub pos: Pos,
}

impl KeyDisplay {
    #[must_use]
    pub fn new(pos: Pos) -> Self {
        KeyDisplay { pos }
    }
}

/// The exit door.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct ExitDoor;
