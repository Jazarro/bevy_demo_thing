use bevy::prelude::*;

use crate::systems::death::death_anim::Dying;
use crate::systems::enemy::spawner::Enemy;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::steering::Steering;

pub fn enemy_kill(
    mut commands: Commands,
    query_player: Query<(Entity, &Steering), With<Player>>,
    query_enemy: Query<&Steering, With<Enemy>>,
) {
    if let Ok((player, player_steering)) = query_player.get_single() {
        for steering in query_enemy.iter() {
            if steering.overlaps(player_steering) {
                commands.entity(player).insert(Dying::default());
            }
        }
    }
}
