use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::camera::camera_systems::{camera_control, camera_follow_focal_point};
use crate::camera::create_camera::create_camera;
use crate::level_select::create_default_adventure::create_default_adventure;
use crate::level_select::setup_systems::on_start;
use crate::level_select::update_systems::{check_input, update_cursor, update_ui};
use crate::states::{back_on_escape, delete_all_entities, start_music, AppState};

pub struct LevelSelectState;

impl Plugin for LevelSelectState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::LevelSelect,
            ConditionSet::new()
                .label("create_default")
                .run_in_state(AppState::LevelSelect)
                .with_system(create_default_adventure)
                .with_system(start_music)
                .into(),
        )
        .add_enter_system_set(
            AppState::LevelSelect,
            ConditionSet::new()
                .run_in_state(AppState::LevelSelect)
                .after("create_default")
                .with_system(create_camera)
                .with_system(on_start)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::LevelSelect)
                .with_system(back_on_escape)
                .with_system(check_input)
                .with_system(camera_follow_focal_point)
                .with_system(camera_control)
                .with_system(update_cursor)
                .with_system(update_ui)
                .into(),
        )
        .add_exit_system_set(
            AppState::LevelSelect,
            ConditionSet::new()
                .run_in_state(AppState::LevelSelect)
                .with_system(delete_all_entities)
                .into(),
        );
    }
}
