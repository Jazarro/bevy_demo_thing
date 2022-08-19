use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::files::{get_default_settings_dir, get_user_settings_dir, serialise_ron};

/// To change the default settings, check out the `assets/config/default_settings/audio.ron` file.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AudioSettings {
    /// What volume the music should be played at. If this value is None, the music will not be
    /// played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub music_volume: Option<f32>,
    /// What volume the sound effects should be played at. If this value is None, the music will
    /// not be played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub sfx_volume: Option<f32>,
}

impl AudioSettings {
    /// Add the given delta to the current music volume and write the `SoundConfig` to a user
    /// settings file.
    pub fn add_to_music_volume(&mut self, delta: f32) {
        self.music_volume = Self::add_volume(self.music_volume, delta);
        self.write_settings(get_user_settings_dir().join("audio.ron"));
    }

    /// Add the given delta to the current sound effects volume and write the `SoundConfig` to a user
    /// settings file.
    pub fn add_to_sfx_volume(&mut self, delta: f32) {
        self.sfx_volume = Self::add_volume(self.sfx_volume, delta);
        self.write_settings(get_user_settings_dir().join("audio.ron"));
    }

    /// Add the delta to the starting volume. Clamp to range [0, 1].
    /// A value of zero is interpreted as None (sound off).
    fn add_volume(starting_volume: Option<f32>, delta: f32) -> Option<f32> {
        let current_volume = starting_volume.unwrap_or(0.0);
        let new_volume = (current_volume + delta).max(0.0).min(1.0);
        Some(new_volume).and_then(|volume| {
            if volume.abs() < f32::EPSILON {
                None
            } else {
                Some(volume)
            }
        })
    }

    /// Return a pretty printed representation of the music volume.
    #[must_use]
    pub fn format_music_volume(&self) -> String {
        Self::format_volume(self.music_volume)
    }

    /// Return a pretty printed representation of the sound effects volume.
    #[must_use]
    pub fn format_sfx_volume(&self) -> String {
        Self::format_volume(self.sfx_volume)
    }

    /// Return a pretty printed representation of the given volume value.
    fn format_volume(volume: Option<f32>) -> String {
        match volume {
            Some(volume) => format!("{:.2}", volume),
            None => "Off".to_string(),
        }
    }

    fn write_settings(&self, path: PathBuf) {
        fs::write(
            path,
            serialise_ron(self).expect("Failed to serialise AudioConfig to ron."),
        )
        .expect("Failed to write AudioConfig to file.");
    }

    /// Loads the most relevant instance of `AudioSettings`.
    ///
    /// If the user `AudioSettings` file exists, tries to load from user settings first. If that fails,
    /// log an error and use the Default trait implementation (ie: `AudioSettings::default()`).
    ///
    /// If the user 'AudioSettings' file does not exist, tries to load the default settings file instead.
    #[must_use]
    pub fn load_from_file() -> AudioSettings {
        let user_settings_file = get_user_settings_dir().join("audio.ron");
        if user_settings_file.exists() {
            load_from_path(&user_settings_file)
        } else {
            load_from_path(&get_default_settings_dir().join("audio.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> AudioSettings {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<AudioSettings>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the audio settings file from {:?}! Falling back to AudioSettings::default(). Error: {:?}",
                    path, error
                );
            AudioSettings::default()
        })
}
