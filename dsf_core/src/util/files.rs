use ron::Options;
use std::fs;
use std::path::{Path, PathBuf};

use crate::levels::level_save::LevelSave;
use serde::Serialize;

/// Returns a `PathBuf` to the file that is used to store auto saves.
pub fn auto_save_file() -> PathBuf {
    get_levels_dir().join("auto_save.ron")
}

pub fn get_default_settings_dir() -> PathBuf {
    create_if_missing(get_config_dir().join("default_settings/"))
}

pub fn get_config_dir() -> PathBuf {
    create_if_missing(get_assets_dir().join("config/"))
}

pub fn get_adventures_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("adventures/"))
}

pub fn get_levels_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("levels/"))
}

pub fn get_world_dir() -> PathBuf {
    get_assets_dir().join("world/")
}

pub fn get_atlases_dir() -> PathBuf {
    create_if_missing(get_assets_dir().join("atlases/"))
}

pub fn get_assets_dir() -> PathBuf {
    get_root_dir().join("assets/")
}

fn get_root_dir() -> PathBuf {
    PathBuf::new()
    // application_root_dir().expect("Root directory not found!")
}

fn create_if_missing(path: PathBuf) -> PathBuf {
    fs::create_dir_all(&path).unwrap_or_else(|err| {
        panic!(
            "Failed to create directory {:?} because error {:?}",
            &path, err
        )
    });
    path
}

/// This directory contains transient user data. That includes player settings, key bindings,
/// cache files, save files, etc.
/// This directory will not be in git. It will be empty (or not even exist) the first time any
/// player starts up the game.
fn get_user_data_dir() -> PathBuf {
    create_if_missing(get_root_dir().join(".userdata/"))
}

pub fn get_user_cache_file() -> PathBuf {
    get_user_data_dir().join("cache.ron")
}

pub fn get_user_settings_dir() -> PathBuf {
    create_if_missing(get_user_data_dir().join("settings/"))
}

pub fn serialise_ron<S>(serialize: S) -> Result<String, ron::Error>
where
    S: Serialize,
{
    let pretty_config = ron::ser::PrettyConfig::default()
        // .indentor("\t".to_string())
        .new_line("\n".to_string());
    let mut buf = Vec::new();
    let mut ron_serializer =
        ron::ser::Serializer::with_options(&mut buf, Some(pretty_config), Options::default())?;
    serialize.serialize(&mut ron_serializer)?;
    Ok(String::from_utf8(buf).unwrap())
}

// pub fn deserialise_ron<S>(deserialise: S) -> Result<String, ron::Error> where S:Serialize,{
//     = fs::read_to_string("address.txt")?.parse()?;
// }

/// Loads the level from file.
#[must_use]
pub fn load_level_file(path: &Path) -> LevelSave {
    let data = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to read level file at path: {:?}", path));
    ron::de::from_str::<LevelSave>(&data).expect("Unable to deserialise LevelSave")
}
