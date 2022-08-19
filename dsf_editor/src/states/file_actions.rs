use crate::resources::level_edit::LevelEdit;
use bevy::log::info;
use dsf_core::levels::level_save::LevelSave;
use dsf_core::util::files::{auto_save_file, get_levels_dir, serialise_ron};
use std::fs;
use std::path::PathBuf;

/// Write the current state of the `LevelEdit` to the auto save file, overwriting what is already
/// there.
pub fn auto_save(level_edit: &LevelEdit) {
    write_level_file(auto_save_file(), level_edit);
    info!("Auto-saved the level!");
}

/// Store the current state of the `LevelEdit` to file. The given name will be used as a filename.
/// TODO: check if name is reserved (ie: `auto_save`)
/// TODO: check if level already exists, if so maybe ask to overwrite?
///     (or keep track of which one we loaded, so we know whether it's safe to overwrite)
#[allow(dead_code)] //Not used yet, but will be used in the future.
pub fn save(name: String, level_edit: &LevelEdit) {
    let level_file = get_levels_dir().join(name + ".ron");
    write_level_file(level_file, level_edit);
}

fn write_level_file(file: PathBuf, level_edit: &LevelEdit) {
    let level_save: LevelSave = (*level_edit).clone().into();
    fs::write(
        file,
        serialise_ron(level_save).expect("Failed to serialise LevelSave to ron."),
    )
    .expect("Failed to write LevelSave to file.");
}
