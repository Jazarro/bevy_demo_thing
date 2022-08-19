use bevy::prelude::*;

use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::{DebugPosGhostTag, DebugSteeringGhostTag, Player};
use crate::systems::motion::structs::steering::Steering;

pub fn debug_system(
    query_player: Query<(&Coords, &Steering), With<Player>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<DebugSteeringGhostTag>>,
        Query<&mut Transform, With<DebugPosGhostTag>>,
    )>,
) {
    // Sets the transform on the ghost tags.
    // This is a debug thing to show us where the player is going.
    if let Ok((coords, steering)) = query_player.get_single() {
        for mut transform in set.p0().iter_mut() {
            let (centered_x, centered_y) = coords.to_centered_coords(steering.destination);
            transform.translation.x = centered_x;
            transform.translation.y = centered_y;
        }
        for mut transform in set.p1().iter_mut() {
            transform.translation.x = coords.pos.x as f32 + 0.5;
            transform.translation.y = coords.pos.y as f32 + 0.5;
        }
    }
}
