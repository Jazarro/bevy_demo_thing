use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::loading::assets::{MusicType, SoundType, SpriteType};
use crate::util::files::get_config_dir;

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoadingConfig {
    pub atlases: HashMap<SpriteType, String>,
    pub sound_effects: HashMap<SoundType, String>,
    pub music: HashMap<MusicType, String>,
}

impl LoadingConfig {
    /// Loads the LoadingConfig from file.
    #[must_use]
    pub fn load_from_file() -> LoadingConfig {
        let file = get_config_dir().join("loading.ron");
        let data = fs::read_to_string(file).expect("Unable to read loading config file");
        ron::de::from_str::<LoadingConfig>(&data).expect("Unable to deserialise LoadingConfig")
    }
}
