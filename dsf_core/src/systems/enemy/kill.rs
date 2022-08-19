use bevy::prelude::*;

use crate::systems::death::death_anim::Dying;
use crate::systems::enemy::spawner::Enemy;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::Player;

pub fn enemy_kill(
    mut commands: Commands,
    query_player: Query<(Entity, &Coords), With<Player>>,
    query_enemy: Query<&Coords, With<Enemy>>,
) {
    if let Ok((player, player_coords)) = query_player.get_single() {
        for enemy_coords in query_enemy.iter() {
            if enemy_coords.overlaps(player_coords) {
                commands.entity(player).insert(Dying::default());
            }
        }
    }
}
