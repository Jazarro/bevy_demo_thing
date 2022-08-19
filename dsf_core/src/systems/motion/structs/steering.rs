use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::levels::world_bounds::WorldBounds;
use crate::systems::motion::structs::direction::{Direction1D, Direction2D};
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering_mode::SteeringMode;

/// Remembers the direction an entity is moving in. Also keeps a destination as a discrete position.
/// Steering is used to accomplish the snap-to-grid, tile-based movement.
///
/// Any non-particle entity that has movement should have steering.
/// Examples of entities with steering include the Player, enemies and projectiles.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Component)]
#[serde(deny_unknown_fields)]
pub struct Steering {
    /// The entity's discrete position. Not to be confused with its Transform, which is where the
    /// entity is actually at. Transform can be between squares, the discrete position is always at
    /// a square. The discrete position has it's coordinate in integral numbers, whereas the
    /// Transform's translation is in floats.
    ///
    /// If an entity is wider than 1 by 1, the pos is the bottom-left most tile in the entity's
    /// body.
    pub pos: Pos,
    /// Width and height of the entity.
    pub dimens: IVec2,
    /// Direction the entity is facing along the x-axis and y-axis.
    pub facing: Direction2D,
    pub destination: Pos,
    pub mode: SteeringMode,
}

impl Steering {
    pub fn new(pos: Pos, dimens: IVec2) -> Steering {
        Steering {
            pos,
            dimens,
            facing: Direction2D::new(1., 0.),
            destination: pos,
            mode: SteeringMode::Grounded,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.mode == SteeringMode::Grounded
    }

    pub fn is_mid_air(&self) -> bool {
        match self.mode {
            SteeringMode::Falling { .. } => true,
            SteeringMode::Jumping { .. } => true,
            _ => false,
        }
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

    /// Converts the given discrete position to a translation, taking into account the dimensions
    /// of the entity.
    ///
    /// The discrete position is the bottom-left corner of the entity, a translation is the
    /// center point of the entity.
    pub fn to_centered_coords(&self, pos: Pos) -> (f32, f32) {
        (
            pos.x as f32 + 0.5 * self.dimens.x as f32,
            pos.y as f32 + 0.5 * self.dimens.y as f32,
        )
    }

    /// Converts the given translation, which is the center-point of the entity, into a pair of
    /// anchored coordinates, describing the bottom-left corner of the entity.
    ///
    /// Note that this does NOT return a discrete position: output is not rounded or floored.
    pub fn to_anchor_coords(&self, transform: &Transform) -> (f32, f32) {
        (
            transform.translation.x - 0.5 * self.dimens.x as f32,
            transform.translation.y - 0.5 * self.dimens.y as f32,
        )
    }

    pub fn wrap(&mut self, bounds: &WorldBounds, transform: &mut Transform) {
        let delta = Pos::new(
            if self.pos.x < bounds.x() && self.facing.x == Direction1D::Negative {
                bounds.width()
            } else if (self.pos.x + self.dimens.x) > bounds.upper_x()
                && self.facing.x == Direction1D::Positive
            {
                -bounds.width()
            } else {
                0
            },
            if self.pos.y < bounds.y()
                && (self.mode != SteeringMode::Climbing || self.facing.y == Direction1D::Negative)
            {
                bounds.height()
            } else if (self.pos.y + self.dimens.y) > bounds.upper_y()
                && (self.mode != SteeringMode::Climbing || self.facing.y == Direction1D::Positive)
            {
                -bounds.height()
            } else {
                0
            },
        );
        self.pos = self.pos + delta;
        self.destination = self.destination + delta;
        self.mode = self.mode.wrap(delta.y as f32);
        transform.translation.x += delta.x as f32;
        transform.translation.y += delta.y as f32;
    }

    pub fn overlaps(&self, other: &Steering) -> bool {
        self.pos.x < other.pos.x + other.dimens.x
            && self.pos.x + self.dimens.x > other.pos.x
            && self.pos.y < other.pos.y + other.dimens.y
            && self.pos.y + self.dimens.y > other.pos.y
    }
    pub fn overlaps_pos(&self, other: &Pos) -> bool {
        let other = Steering::new(*other, IVec2::new(1, 1));
        self.overlaps(&other)
    }

    pub fn overlaps_rect(&self, other: &Pos, other_dimens: &IVec2) -> bool {
        let other = Steering::new(*other, *other_dimens);
        self.overlaps(&other)
    }
}
