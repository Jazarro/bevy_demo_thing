use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::loading::assets::{AssetStorage, SoundType};
use crate::loading::entities::inflate::spawn_enemy;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::dimens::Dimens;

const SPAWN_COOLDOWN: f32 = 2.;
const SPAWN_DURATION: f32 = 2.;

#[derive(Component)]
pub struct Spawner {
    pub state: SpawnerState,
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            state: SpawnerState::SpawnCooldown(Timer::from_seconds(SPAWN_COOLDOWN, false)),
        }
    }
}

pub enum SpawnerState {
    SpawnCooldown(Timer),
    Spawning(Timer),
    Spawned(Entity),
}

#[derive(Component, Clone, Default)]
pub struct Enemy;

/// Check if any spawners should start spawning.
pub fn activate_spawners(
    mut commands: Commands,
    // entities: Entities, // TODO: Impossible to include due to bug in iyes_loopless?
    storage: Res<AssetStorage>,
    mut audio: EventWriter<SoundEvent>,
    time: Res<Time>,
    mut query_spawner: Query<(&Coords, &mut Spawner, &mut TextureAtlasSprite)>,
    query_enemy: Query<Entity, With<Enemy>>,
) {
    for (spawner_coords, mut spawner, mut sprite) in query_spawner.iter_mut() {
        match &mut spawner.state {
            SpawnerState::SpawnCooldown(timer) => {
                timer.tick(time.delta());
                if timer.finished() {
                    audio.send(SoundEvent::Sfx(SoundType::SpawnerOpenClose, false));
                    sprite.index = 1;
                    spawner.state =
                        SpawnerState::Spawning(Timer::from_seconds(SPAWN_DURATION, false));
                }
            }
            SpawnerState::Spawning(timer) => {
                timer.tick(time.delta());
                if timer.finished() {
                    audio.send(SoundEvent::Sfx(SoundType::SpawnerOpenClose, false));
                    sprite.index = 0;
                    let entity = spawn_enemy(
                        &mut commands,
                        &storage,
                        Coords::new(spawner_coords.pos, Dimens::new(2, 2)),
                    );
                    spawner.state = SpawnerState::Spawned(entity);
                }
            }
            SpawnerState::Spawned(entity) => {
                if !query_enemy.iter().any(|enemy| &enemy == entity) {
                    spawner.state =
                        SpawnerState::SpawnCooldown(Timer::from_seconds(SPAWN_COOLDOWN, false));
                }
            }
        }
    }
}
