use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::loading::loading_systems::{check_load_state, load_assets, load_configs};
use crate::states::AppState;

pub struct LoadingState;

impl Plugin for LoadingState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Loading,
            ConditionSet::new()
                .run_in_state(AppState::Loading)
                .with_system(load_assets)
                .with_system(load_configs)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Loading)
                .with_system(check_load_state)
                .into(),
        )
        .add_exit_system_set(
            AppState::Loading,
            ConditionSet::new().run_in_state(AppState::Loading).into(),
        );
    }
}
