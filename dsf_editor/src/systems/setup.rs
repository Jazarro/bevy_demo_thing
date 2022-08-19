use crate::resources::level_edit::LevelEdit;
use crate::resources::status::editor_status::EditorStatus;
use bevy::prelude::*;
use dsf_core::level_select::structs::LevelSelectionInstruction;
use dsf_core::levels::level_save::LevelSave;
use dsf_core::loading::levels::load_level_system::{add_plain_background, load_tile_definitions};
use dsf_core::util::files::load_level_file;

/// Perform setup that should be executed both upon starting and upon resuming the State.
pub fn init_misc(mut commands: Commands, instruction: Res<LevelSelectionInstruction>) {
    let tile_defs = load_tile_definitions();
    let mut status = EditorStatus::default();
    status.brush.set_palette(&tile_defs);
    commands.insert_resource(status);

    let path = instruction
        .level
        .as_ref()
        .expect("Tried to start LevelEditor without level to edit!!");

    let level_save = if path.is_file() {
        load_level_file(path)
    } else {
        LevelSave::default()
    };
    let level_edit = LevelEdit::new(level_save, tile_defs);
    add_plain_background(&level_edit.tile_map.world_bounds, &mut commands);
    commands.insert_resource(level_edit);
}

pub fn init_instructions(mut instruction: ResMut<LevelSelectionInstruction>) {
    instruction.editor_open = true;
}
