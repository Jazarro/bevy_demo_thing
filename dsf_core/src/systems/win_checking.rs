use std::collections::HashSet;

use bevy::prelude::*;

use crate::audio::sound_event::SoundEvent;
use crate::levels::tiles::objects::{ExitDoor, Key, KeyDisplay};
use crate::loading::assets::SoundType::{PickupKey, PickupLastKey};
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::win_handling::WinResource;

/// Key width and height, hardcoded for now.
/// TODO: Get rid of these hardcoded constants.
const KEY_WIDTH: f32 = 2.;
const KEY_HEIGHT: f32 = 2.;

/// Maintains some information related to winning the level.
/// In any given level, the player must collect all keys. Once all keys are collected, the exit door
/// opens. When the player then reaches the door, they complete the level.
#[derive(Debug, Default)]
pub struct WinCondition {
    /// The set of positions of keys that are left in the level. If this collection is empty, then
    /// the player has collected all keys and is free to finish the level by reaching the exit door.
    pub keys: HashSet<Pos>,
}

impl WinCondition {
    /// Reset when (re)loading a level.
    pub fn reset(&mut self) {
        self.keys.clear();
    }
    /// Add a key. Only to be used when loading a level.
    pub fn add_key(&mut self, pos: Pos) {
        self.keys.insert(pos);
    }
    /// How many keys are left uncollected in the level.
    #[must_use]
    pub fn nr_keys_left(&self) -> usize {
        self.keys.len()
    }
    /// Sets the key at the given position as collected.
    pub fn set_key_collected(&mut self, pos: Pos) {
        self.keys.remove(&pos);
    }
    /// Whether or not the player has collected all keys.
    /// If this returns true, the door is open and once the player reaches it they win the level.
    #[must_use]
    pub fn all_keys_collected(&self) -> bool {
        self.keys.is_empty()
    }
}

/// Checks if the player intersects any keys.
/// If so, the key will collected by the player and will be removed from the game.
pub fn key_collect_system(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut win: ResMut<WinCondition>,
    query_player: Query<(&Player, &Coords, &Transform)>,
    query_keys: Query<(&Coords, &Transform, Entity), With<Key>>,
    query_key_displays: Query<(&KeyDisplay, Entity)>,
) {
    let player_collider = query_player
        .iter()
        .map(|(_, coords, transform)| {
            (
                Vec2::new(transform.translation.x, transform.translation.y),
                Vec2::new(coords.dimens.x as f32, coords.dimens.y as f32),
            )
        })
        .next();
    if let Some((pos, dimens)) = player_collider {
        let collected_key = query_keys
            .iter()
            .filter(|(_, transform, _)| {
                let key_x = transform.translation.x;
                let key_y = transform.translation.y;
                pos.x - dimens.x / 2. < key_x + KEY_WIDTH / 3.
                    && pos.x + dimens.x / 2. > key_x - KEY_WIDTH / 3.
                    && pos.y - dimens.y / 2. < key_y + KEY_HEIGHT / 3.
                    && pos.y + dimens.y / 2. > key_y - KEY_HEIGHT / 3.
            })
            .map(|(key, _, entity)| (key, entity))
            .next();
        if let Some((coords, key_entity)) = collected_key {
            win.set_key_collected(coords.pos);
            let sound_event = if win.all_keys_collected() {
                SoundEvent::Sfx(PickupLastKey, true)
            } else {
                SoundEvent::Sfx(PickupKey, false)
            };
            audio.send(sound_event);
            commands.entity(key_entity).despawn_recursive();
            for (key_display, display_entity) in query_key_displays.iter() {
                if key_display.pos == coords.pos {
                    commands.entity(display_entity).despawn_recursive();
                }
            }
        }
    }
}

/// Checks if the player has finished the level.
/// The player finishes the level when they collect all keys and then reach the exit door.
pub fn check_if_won(
    mut commands: Commands,
    win: Res<WinCondition>,
    query_player: Query<(&Steering, &Coords), With<Player>>,
    query_doors: Query<&Coords, With<ExitDoor>>,
) {
    if let Ok((player_steering, player_coords)) = query_player.get_single() {
        if !win.all_keys_collected() || !player_steering.is_grounded() {
            return;
        }
        for door_coords in query_doors.iter() {
            if player_coords.overlaps(door_coords) {
                commands.insert_resource(WinResource::default());
            }
        }
    }
}
