use crate::level_select::structs::LevelSelectionInstruction;
use crate::states::AppState;
use crate::systems::menu::button::{add_btn, DsfButton, MenuButtons};
use crate::util::files::{get_adventures_dir, get_levels_dir};
use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::state::NextState;

pub fn setup_main_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut instruction: ResMut<LevelSelectionInstruction>,
) {
    instruction.adventure = Some(get_adventures_dir().join("default.ron"));

    commands.spawn_bundle(Camera2dBundle::default());
    let buttons = vec![
        "Play".to_string(),
        "Level Editor".to_string(),
        "Settings".to_string(),
        "Exit".to_string(),
    ];
    let container = commands.spawn_bundle(SpatialBundle::default()).id();
    for i in 0..buttons.len() {
        add_btn(&mut commands, &assets, &buttons, i, container);
    }
    commands.insert_resource(MenuButtons {
        selected: 0,
        buttons,
        timer: None,
    });
}

pub fn read_menu_input(
    mut instruction: ResMut<LevelSelectionInstruction>,
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut buttons: ResMut<MenuButtons>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.clear_just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
    if keys.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        buttons.selected =
            (buttons.selected as i32 + 1).rem_euclid(buttons.buttons.len() as i32) as usize;
    }
    if keys.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        buttons.selected =
            (buttons.selected as i32 - 1).rem_euclid(buttons.buttons.len() as i32) as usize;
    }
    if keys.any_just_pressed([KeyCode::Return, KeyCode::NumpadEnter]) {
        keys.clear_just_pressed(KeyCode::Return);
        keys.clear_just_pressed(KeyCode::NumpadEnter);
        match buttons.selected {
            0 => commands.insert_resource(NextState(AppState::LevelSelect)),
            1 => {
                instruction.level = Some(get_levels_dir().join("auto_save.ron"));
                commands.insert_resource(NextState(AppState::LevelEditor));
            }
            2 => commands.insert_resource(NextState(AppState::Settings)),
            3 => exit.send(AppExit),
            _ => (),
        }
    }
}

pub fn animate_buttons(buttons: Res<MenuButtons>, mut query: Query<(&mut Transform, &DsfButton)>) {
    for (mut transform, button) in query.iter_mut() {
        transform.scale = if buttons.selected == button.0 {
            Vec3::splat(1.1)
        } else {
            Vec3::splat(1.)
        };
    }
}
