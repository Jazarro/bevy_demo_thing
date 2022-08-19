use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
pub struct AssetStorage {
    atlases: HashMap<SpriteType, Handle<TextureAtlas>>,
    sounds: HashMap<SoundType, Vec<Handle<AudioSource>>>,
    music: HashMap<MusicType, Vec<Handle<AudioSource>>>,
}

impl AssetStorage {
    pub fn put_atlas(&mut self, asset_type: SpriteType, asset: Handle<TextureAtlas>) {
        self.atlases.insert(asset_type, asset);
    }
    pub fn get_atlas(&self, asset_type: &SpriteType) -> Handle<TextureAtlas> {
        (*self
            .atlases
            .get(asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.atlases.get(&SpriteType::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_sound(&mut self, sound_type: SoundType, asset: Handle<AudioSource>) {
        self.sounds
            .entry(sound_type)
            .or_insert_with(Vec::new)
            .push(asset);
    }
    pub fn get_sound(&self, asset_type: &SoundType) -> Option<Handle<AudioSource>> {
        self
            .sounds
            .get(asset_type)
            .or_else(|| {
                error!("There are no sounds of type {:?}. Add them to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*(sounds_of_that_type.get(random_index).expect("Should not panic."))).clone()
            })
    }
    pub fn put_music(&mut self, music_type: MusicType, asset: Handle<AudioSource>) {
        self.music
            .entry(music_type)
            .or_insert_with(Vec::new)
            .push(asset);
    }
    pub fn get_music(&self, asset_type: &MusicType) -> Option<Handle<AudioSource>> {
        self
            .music
            .get(asset_type)
            .or_else(|| {
                error!("There is no music of type {:?}. Add it to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*(sounds_of_that_type.get(random_index).expect("Should not panic."))).clone()
            })
    }
}

/// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct AssetType(pub SpriteType, pub usize);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpriteType {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Ladder,
    Frame,
    Blocks,
    Tools,
    Door,
    LevelSelect,
    EnemyAnims,
    PlayerAnims,
    Background,
    BackgroundHeads,
    BackgroundEyes,
    Spawner,
    RevolvingDoor,
}

impl Default for SpriteType {
    fn default() -> Self {
        SpriteType::NotFound
    }
}

/// Identifies a type of sound effect. Each of these sound types could be represented by any number
/// of sound files that the game will randomly pick from.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SoundType {
    /// Sound will be played when the player initiates a jump.
    Jump,
    /// The player's footstep. Sound file must be a single footstep. Sound must not be too loud or
    /// noticeable.
    Step,
    /// One step while climbing on a ladder. Sound file must be just a single footstep. Sound must
    /// not be too loud or noticeable.
    LadderStep,
    /// One step when on the adventure and level selection screen.
    MapStep,
    /// This will be played when the player tries something that is not possible, such as trying to
    /// jump while underneath a 2-high ledge.
    CannotPerformAction,
    /// When the player starts to use a mining tool, ie: a tool that breaks blocks.
    Mining,
    /// Played when the player picks up any tool.
    PickupTool,
    /// Played when the player picks up a key.
    PickupKey,
    /// Played when the player collects the last key and unlocks the exit door.
    PickupLastKey,
    /// Played when the player completes a level by reaching the exit door after having picked up
    /// all keys.
    Win,
    /// Plays when the player resets the puzzle to the beginning
    /// (probably because they made a mistake).
    LvlReset,
    Death,
    GameOver,
    LevelSelect,
    SpawnerOpenClose,
    TrapWallCreated,
}

/// Identifies a music track.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum MusicType {
    Menu,
    InGame,
}
