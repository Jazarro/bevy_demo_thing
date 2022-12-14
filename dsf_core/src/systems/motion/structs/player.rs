use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::levels::tiles::tile_defs::ToolType;

/// The entity with this component is the player.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct Player {
    /// The tool currently equipped by the player.
    pub equipped: Option<ToolType>,
    /// Whether the jump key is currently down. Needed to figure out if the player wants to jump
    /// this frame. (Jump is only executed if this value changes from false to true.)
    pub pressing_jump: bool,
    /// How many seconds have passed since the character started jumping?
    ///
    /// This value is usually None. When the character starts jumping, it is assigned Some(0.0).
    /// The delta_seconds is added to this value every tick. Once it surpasses a threshold, it is
    /// set back to None.
    ///
    /// As long as the grace timer hasn't run out yet, the player can give their jump horizontal
    /// speed. This fixes the problem that if the player presses jump and move at the same time,
    /// jump is sometimes registered before move and the character only jumps up, not sideways.
    pub jump_grace_timer: Option<f32>,
    /// How many seconds have passed since the character turned around while standing still?
    ///
    /// This value is usually None. When the player is standing still and presses move in the
    /// opposite direction that they're facing, they will turn around and this value will be
    /// initialised at Some(0.0). The delta_seconds is added to this value every tick. Once it
    /// surpasses a certain threshold or the player stops pressing move, this value is reset to None.
    ///
    /// As long as the timer hasn't run out yet, the player will not start moving. This fixes the
    /// problem that tapping RIGHT while facing left will not only turn around, but will also move
    /// 1 tile to the right.
    pub turn_around_timer: Option<f32>,
}

/// The entity with this component is a tool equipped by the player.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct EquippedTag;

/// A debug entity that shows the player's current discrete position.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct DebugPosGhostTag;

/// A debug entity that shows the player's destination.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct DebugSteeringGhostTag;
