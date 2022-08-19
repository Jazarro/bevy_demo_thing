use bevy::prelude::*;
use dsf_core::systems::motion::structs::pos::Pos;
use std::cmp::min;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection {
    /// Inclusive bound.
    pub start: Pos,
    /// Inclusive bound. The end point of the selection is always set to the current location of the cursor.
    pub end: Pos,
}

impl Selection {
    #[must_use]
    pub fn lower_bounds(&self) -> Pos {
        Pos::new(min(self.start.x, self.end.x), min(self.start.y, self.end.y))
    }
    #[must_use]
    pub fn dimens(&self) -> IVec2 {
        IVec2::new(
            (self.start.x - self.end.x).abs() + 1,
            (self.start.y - self.end.y).abs() + 1,
        )
    }
}
