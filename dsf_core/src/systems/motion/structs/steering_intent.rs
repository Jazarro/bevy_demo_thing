use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::pos::Pos;

/// Specifies how the entity intents to move. For the player, this is mostly informed by the
/// keyboard input. For enemies, this will be set by the AI. For all entities with Steering,
/// the SteeringSystem then actually moves the entity based on this intent.
#[derive(Component, Deserialize, Serialize, Default, Clone, Debug)]
pub struct SteeringIntent {
    /// If a player is still holding a horizontal movement key (for instance; RIGHT) when they
    /// start climbing, they might move off the ladder after climbing 1 tile. To fix this,
    /// this flag will be set to true when the player starts climbing. To start moving horizontally
    /// at this point, they must let go of the movement key and press RIGHT or LEFT again.
    ///
    /// If they reach the end of the ladder and can climb no further, if they're still holding down
    /// the (invalidated) RIGHT or LEFT button they will start moving horizontally regardless of
    /// this flag.
    ///
    /// This feature exists solely for players, to make movement feel better.
    pub walk_invalidated: bool,
    /// The entity wishes to face this direction. Only for player characters. Note that entities
    /// will still face the direction they are walking in, this value is just for when you
    /// want to face a certain direction without walking.
    pub face: Direction1D,
    /// The entity wishes to walk along the floor in this direction.
    pub walk: Direction1D,
    /// The entity wishes to climb on a ladder in this direction.
    pub climb: Direction1D,
    /// If true; the entity wishes to jump.
    pub jump: bool,
    /// The entity wishes to jump in this direction. This is separate from walk because it is
    /// possible to specify a direction for a limited time after the jump has already started.
    /// That feature exists solely for players, to make movement feel better.
    pub jump_direction: Direction1D,
    /// If the player is forced to walk into a certain spot. Used for revolving and one-way doors.
    pub forced_walk: Option<Pos>,
}
