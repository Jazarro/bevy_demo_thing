use bevy::prelude::*;
use iyes_loopless::prelude::CurrentState;
use iyes_loopless::state::NextState;

use crate::audio::sound_event::SoundEvent;
use crate::config::settings::debug_settings::DebugSettings;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::loading::assets::MusicType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    AltMenu,
    LevelSelect,
    InGame,
    LevelEditor,
    Settings,
}

pub fn delete_all_entities(mut commands: Commands, query: Query<Entity>) {
    info!("Deleting all entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn back_on_escape(
    mut instruction: ResMut<LevelSelectionInstruction>,
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    config: Res<DebugSettings>,
    current_state: Res<CurrentState<AppState>>,
) {
    if keys.clear_just_pressed(KeyCode::Escape) {
        let next = if current_state.0 == AppState::InGame {
            if instruction.editor_open {
                AppState::LevelEditor
            } else if config.use_alternate_menu {
                AppState::AltMenu
            } else {
                AppState::LevelSelect
            }
        } else if current_state.0 == AppState::LevelEditor {
            instruction.editor_open = false;
            AppState::MainMenu
        } else {
            AppState::MainMenu
        };
        commands.insert_resource(NextState(next));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
pub fn debug_current_state(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        info!("Switching to game state {:?}!", state.0);
    }
}

pub fn start_music(state: Res<CurrentState<AppState>>, mut audio: EventWriter<SoundEvent>) {
    let music_type = match state.0 {
        AppState::AltMenu => Some((MusicType::Menu, true)),
        AppState::MainMenu => Some((MusicType::Menu, true)),
        AppState::LevelSelect => Some((MusicType::Menu, true)),
        AppState::InGame => Some((MusicType::InGame, true)),
        _ => None,
    };
    audio.send(SoundEvent::Music(music_type));
    audio.send(SoundEvent::KillAllSoundEffects)
}
