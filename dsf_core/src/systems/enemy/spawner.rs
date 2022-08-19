use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::bundles::EnemyBundle;
use crate::levels::load_level_system::load_transform;
use crate::levels::tiles::tile_defs::DepthLayer;
use crate::loading::assets::{AssetStorage, SoundType, SpriteType};
use crate::systems::animations::structs::AnimationTimer;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;

const SPAWN_COOLDOWN: f32 = 2.;
const SPAWN_DURATION: f32 = 2.;

#[derive(Component)]
pub struct Spawner {
    pub pos: Pos,
    pub state: SpawnerState,
}

impl Spawner {
    pub fn new(pos: Pos) -> Self {
        Spawner {
            state: SpawnerState::SpawnCooldown(Timer::from_seconds(SPAWN_COOLDOWN, false)),
            pos,
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
    mut query_spawner: Query<(&mut Spawner, &mut TextureAtlasSprite)>,
    query_enemy: Query<Entity, With<Enemy>>,
) {
    for (mut spawner, mut sprite) in query_spawner.iter_mut() {
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
                    let entity = commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: storage.get_atlas(&SpriteType::EnemyAnims),
                            transform: load_transform(
                                &spawner.pos,
                                &IVec2::new(2, 2),
                                &DepthLayer::Enemies,
                            ),
                            sprite: TextureAtlasSprite {
                                index: 0,
                                ..default()
                            },
                            ..default()
                        })
                        .insert_bundle(EnemyBundle {
                            steering: Steering::new(spawner.pos, IVec2::new(2, 2)),
                            anim: AnimationTimer::for_player(),
                            ..default()
                        })
                        .id();
                    spawner.state = SpawnerState::Spawned(entity);
                }
            }
            SpawnerState::Spawned(entity) => {
                if query_enemy.iter().find(|enemy| enemy == entity).is_none() {
                    spawner.state =
                        SpawnerState::SpawnCooldown(Timer::from_seconds(SPAWN_COOLDOWN, false));
                }
                // if !entities.contains(*entity) {
                //     spawner.state = SpawnerState::SpawnCooldown(Timer::from_seconds(2., false));
                // }
            }
        }
    }
}
