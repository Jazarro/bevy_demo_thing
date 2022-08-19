use serde::{Deserialize, Serialize};

use crate::systems::motion::structs::direction::Direction1D;

/// SteeringMode influences max speeds, ability to jump, ability to move, etc.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SteeringMode {
    /// Standard mode. There is flat ground beneath the entity and the entity can either move
    /// horizontally or initiate a jump.
    Grounded,
    /// Climbing on a ladder. The entity can either climb up or down.
    Climbing,
    /// The entity is falling straight down.
    Falling {
        /// The x-movement that the entity has while falling. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: Direction1D,
        /// The y-coordinate that the entity had when it  first started falling.
        starting_y_pos: f32,
        /// The time in seconds since the entity started falling.
        duration: f32,
    },
    /// The entity is jumping. The character may have an x-velocity.
    /// While jumping, the character's y-coordinate describes a certain curve.
    /// This also takes the original y-coordinate and the start time.
    /// These are necessary to be able to interpolate the y-coordinate.
    Jumping {
        /// The x-movement that the entity has while jumping. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: Direction1D,
        /// The y-coordinate that the entity had when it started the jump.
        starting_y_pos: f32,
        /// The time in seconds since the character started their jump.
        duration: f32,
    },
}

impl Default for SteeringMode {
    fn default() -> Self {
        SteeringMode::Grounded
    }
}

impl SteeringMode {
    /// Calculate the y offset from the initial y-position at the time this movement began.
    /// This method is only valid for SteeringMode::Falling and SteeringMode::Jumping. It will
    /// return 0. otherwise.
    pub fn calc_delta_y(&self, duration: f32) -> f32 {
        match self {
            SteeringMode::Jumping { .. } => -50. * (duration - 0.209).powf(2.) + 2.2,
            SteeringMode::Falling { .. } => duration * -15.,
            _ => 0.,
        }
    }

    pub fn jump_to_fall(&self) -> Self {
        if let SteeringMode::Jumping {
            x_movement,
            starting_y_pos,
            duration,
        } = *self
        {
            SteeringMode::Falling {
                x_movement,
                starting_y_pos: starting_y_pos + self.calc_delta_y(0.209),
                duration: duration - 0.209,
            }
        } else {
            panic!("Not allowed.");
        }
    }

    pub fn add_to_duration(&self, delta_time: f32) -> Self {
        match *self {
            SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration: duration + delta_time,
            },
            SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration: duration + delta_time,
            },
            _ => panic!("Not allowed to call this on SteeringMode that is not Falling or Jumping."),
        }
    }

    pub fn wrap(&self, delta_y: f32) -> Self {
        match *self {
            SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Jumping {
                x_movement,
                starting_y_pos: starting_y_pos + delta_y,
                duration,
            },
            SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Falling {
                x_movement,
                starting_y_pos: starting_y_pos + delta_y,
                duration,
            },
            _ => self.clone(),
        }
    }
}
