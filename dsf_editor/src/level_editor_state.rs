use crate::systems::check_input::check_editor_input;
use crate::systems::cursor::animation::perform_blinking_animation;
use crate::systems::cursor::controls::cursor_controls;
use crate::systems::cursor::setup::init_cursor;
use crate::systems::place_tiles::place_tiles;
use crate::systems::preview_animation::animate_previews;
use crate::systems::refresh_previews::refresh_previews;
use crate::systems::selection::selection_system;
use crate::systems::setup::{init_instructions, init_misc};
use crate::systems::tile_paint::tile_paint_system;
use crate::systems::update_background::update_background;
use bevy::prelude::*;
use dsf_core::camera::camera_systems::{camera_control, camera_follow_focal_point};
use dsf_core::camera::create_camera::create_camera;
use dsf_core::states::{back_on_escape, delete_all_entities, start_music, AppState};
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

pub struct LevelEditorState;

impl Plugin for LevelEditorState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::LevelEditor,
            ConditionSet::new()
                .run_in_state(AppState::LevelEditor)
                .with_system(create_camera)
                .with_system(init_cursor)
                .with_system(init_misc)
                .with_system(init_instructions)
                .with_system(start_music)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::LevelEditor)
                .with_system(back_on_escape)
                .with_system(camera_follow_focal_point)
                .with_system(camera_control)
                .with_system(check_editor_input)
                .with_system(place_tiles)
                .with_system(refresh_previews)
                .with_system(perform_blinking_animation)
                .with_system(animate_previews)
                .with_system(cursor_controls)
                .with_system(update_background) //.after(cursor_controls)) TODO
                .with_system(selection_system) //.after(cursor_controls)) TODO
                .with_system(tile_paint_system) //.after(selection_system)) TODO
                .into(),
        )
        .add_exit_system_set(
            AppState::LevelEditor,
            ConditionSet::new()
                .run_in_state(AppState::LevelEditor)
                .with_system(delete_all_entities)
                .into(),
        );
    }
}
