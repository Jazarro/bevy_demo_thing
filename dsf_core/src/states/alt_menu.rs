use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::states::{delete_all_entities, start_music, AppState};
use crate::systems::menu::alt_menu::{animate_buttons, read_menu_input, setup_alt_menu};

pub struct AltMenuState;

impl Plugin for AltMenuState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::AltMenu,
            ConditionSet::new()
                .run_in_state(AppState::AltMenu)
                .with_system(setup_alt_menu)
                .with_system(start_music)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::AltMenu)
                .with_system(read_menu_input)
                .with_system(animate_buttons)
                .into(),
        )
        .add_exit_system_set(
            AppState::AltMenu,
            ConditionSet::new()
                .run_in_state(AppState::AltMenu)
                .with_system(delete_all_entities)
                .into(),
        );
    }
}
