use bevy::prelude::*;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::states::{back_on_escape, start_music, AppState};

pub struct SettingsState;

impl Plugin for SettingsState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Settings,
            ConditionSet::new()
                .run_in_state(AppState::Settings)
                .with_system(start_music)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Settings)
                .with_system(back_on_escape)
                .into(),
        )
        .add_exit_system_set(
            AppState::Settings,
            ConditionSet::new().run_in_state(AppState::Settings).into(),
        );
    }
}
