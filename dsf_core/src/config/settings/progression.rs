use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::files::{get_default_settings_dir, get_user_settings_dir, serialise_ron};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Progression {
    pub current_level: usize,
    pub levels: Vec<String>,
}

impl Progression {
    pub fn increment(&mut self) {
        self.current_level = (self.levels.len() - 1).min(self.current_level + 1);
        self.write_settings(get_user_settings_dir().join("progression.ron"));
    }
    pub fn reset(&mut self) {
        let defaults = load_from_path(&get_default_settings_dir().join("progression.ron"));
        self.current_level = defaults.current_level;
        self.levels = defaults.levels;
        self.write_settings(get_user_settings_dir().join("progression.ron"));
    }
    fn write_settings(&self, path: PathBuf) {
        fs::write(
            path,
            serialise_ron(self).expect("Failed to serialise Progression to ron."),
        )
        .expect("Failed to write Progression to file.");
    }

    /// Loads the most relevant instance of `Progression`.
    ///
    /// If the user `Progression` file exists, tries to load from user settings first. If that fails,
    /// log an error and use the Default trait implementation (ie: `Progression::default()`).
    ///
    /// If the user 'Progression' file does not exist, tries to load the default settings file instead.
    #[must_use]
    pub fn load_from_file() -> Progression {
        let file = get_user_settings_dir().join("progression.ron");
        if file.exists() {
            load_from_path(&file)
        } else {
            load_from_path(&get_default_settings_dir().join("progression.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> Progression {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<Progression>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the Progression file from {:?}! Falling back to Progression::default(). Error: {:?}",
                    path, error
                );
            Progression::default()
        })
}
