#![forbid(unsafe_code)]
// #![deny(
//     bad_style,
//     const_err,
//     dead_code,
//     improper_ctypes,
//     non_shorthand_field_patterns,
//     no_mangle_generic_items,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     private_in_public,
//     unconditional_recursion,
//     unused,
//     unused_allocation,
//     unused_comparisons,
//     unused_parens,
//     while_true
// )]

use bevy::prelude::*;
use bevy::window::WindowMode;
use iyes_loopless::prelude::AppLooplessStateExt;

use dsf_core::audio::plugin::DsfAudioPlugin;
use dsf_core::config::settings::user_cache::UserCache;
use dsf_core::level_select::structs::LevelSelectionInstruction;
use dsf_core::loading::assets::AssetStorage;
use dsf_core::states::{
    debug_current_state, AltMenuState, AppState, InGameState, LevelSelectState, LoadingState,
    MainMenuState, SettingsState,
};
use dsf_core::systems::rewind::structs::{CurrentState, Rewind};
use dsf_core::systems::win_checking::WinCondition;
use dsf_core::util::window_event_handler::handle_window;
use dsf_editor::level_editor_state::LevelEditorState;
use dsf_editor::systems::refresh_previews::RefreshPreviewsEvent;

fn main() {
    App::new()
        // Uncomment this to override the default log settings:
        // .insert_resource(bevy::log::LogSettings {
        //     level: bevy::log::Level::TRACE,
        //     filter: "wgpu=warn,bevy_ecs=info".to_string(),
        // })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Dwarf Seeks Fortune!".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DsfAudioPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_loopless_state(AppState::Loading)
        .add_plugin(LoadingState)
        .add_plugin(MainMenuState)
        .add_plugin(AltMenuState)
        .add_plugin(LevelSelectState)
        .add_plugin(InGameState)
        .add_plugin(SettingsState)
        .add_plugin(LevelEditorState)
        .add_system(debug_current_state)
        .add_system(handle_window)
        .init_resource::<AssetStorage>()
        .init_resource::<CurrentState>()
        .init_resource::<Rewind>()
        .init_resource::<LevelSelectionInstruction>()
        .init_resource::<WinCondition>()
        .init_resource::<UserCache>()
        .add_event::<RefreshPreviewsEvent>()
        .run();
}
