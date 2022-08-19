use std::fs;

use bevy::prelude::*;

use crate::level_select::structs::{Adventure, AdventureNode, MapElement, NodeDetails};
use crate::levels::level_save::LevelSave;
use crate::systems::motion::structs::pos::Pos;
use crate::util::files::{get_adventures_dir, get_levels_dir, serialise_ron};

/// Creates a new adventure that gives access to every single level.
/// This is useful while there aren't too many levels yet.
/// This is a debug thing.
pub fn create_default_adventure() {
    let mut adventure = Adventure::default();
    level_files()
        .iter()
        .map(|level_name| {
            let level_file = get_levels_dir().join(level_name);
            let data = fs::read_to_string(level_file).expect("Unable to read level file");
            let level = ron::de::from_str::<LevelSave>(&data);
            (level_name, level)
        })
        .filter(|(level_name, result)| {
            result
                .as_ref()
                .map_err(|err| {
                    error!("Failed to load level {:?}: {:?}", level_name, err);
                    err
                })
                .is_ok()
        })
        .map(|(level_name, result)| (level_name, result.expect("Should never panic.")))
        .enumerate()
        .for_each(|(index, (level_name, _level))| {
            adventure.nodes.insert(
                Pos::new((index * 2) as i32, 0),
                MapElement::Node(AdventureNode {
                    name: level_name.clone(),
                    details: NodeDetails::Level(level_name.clone()),
                }),
            );
            if index > 0 {
                adventure
                    .nodes
                    .insert(Pos::new((index * 2 - 1) as i32, 0), MapElement::Road);
            }
        });

    fs::write(
        get_adventures_dir().join("default.ron"),
        serialise_ron(&adventure).expect("Failed to serialise adventure."),
    )
    .expect("Failed to create default adventure that contains all levels.");
}

fn level_files() -> Vec<String> {
    fs::read_dir(get_levels_dir())
        .expect("Failed to read contents of the levels directory.")
        .filter_map(|file| {
            if let Ok(file) = file {
                if file.path().is_file() {
                    Some(
                        file.path()
                            .file_name()
                            .expect("This should not happen.")
                            .to_str()
                            .expect("Music file name did not contain valid unicode.")
                            .to_string(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
