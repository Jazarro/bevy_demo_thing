use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::states::AppState;
use crate::systems::death::death_anim::Dying;
use crate::systems::enemy::spawner::Enemy;
use crate::systems::motion::structs::player::Player;

pub fn check_in_game_input(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    query_enemy: Query<Entity, With<Enemy>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    // Reset the level
    if keys.clear_just_pressed(KeyCode::F5) {
        if let Ok(entity) = query.get_single() {
            commands.entity(entity).insert(Dying::default());
        }
    }
    if keys.clear_just_pressed(KeyCode::F4) {
        // Sneaky debug way to reset the level faster.
        commands.insert_resource(NextState(AppState::InGame));
    }
    if keys.clear_just_pressed(KeyCode::F3) {
        // Sneaky debug way to kill all enemies.
        for entity in query_enemy.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
