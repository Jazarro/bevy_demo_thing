use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::systems::motion::structs::direction::Direction2D;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering_mode::SteeringMode;

/// Remembers the direction an entity is moving in. Also keeps a destination as a discrete position.
/// Steering is used to accomplish the snap-to-grid, tile-based movement.
///
/// Any non-particle entity that has movement should have steering.
/// Examples of entities with steering include the Player, enemies and projectiles.
#[derive(Component, Deserialize, Serialize, Default, Clone, Debug)]
pub struct Steering {
    /// Direction the entity is facing along the x-axis and y-axis.
    pub facing: Direction2D,
    pub destination: Pos,
    pub mode: SteeringMode,
}

impl Steering {
    pub fn new(pos: Pos) -> Steering {
        Steering {
            facing: Direction2D::new(1., 0.),
            destination: pos,
            mode: SteeringMode::Grounded,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.mode == SteeringMode::Grounded
    }

    pub fn is_mid_air(&self) -> bool {
        matches!(
            self.mode,
            SteeringMode::Falling { .. } | SteeringMode::Jumping { .. }
        )
    }

    pub fn is_jumping(&self) -> bool {
        matches!(self.mode, SteeringMode::Jumping { .. })
    }

    pub fn jump_has_peaked(&self) -> bool {
        if let SteeringMode::Jumping { duration, .. } = self.mode {
            duration > 0.209
        } else {
            false
        }
    }

    pub fn is_falling(&self) -> bool {
        matches!(self.mode, SteeringMode::Falling { .. })
    }

    pub fn is_climbing(&self) -> bool {
        self.mode == SteeringMode::Climbing
    }
}
