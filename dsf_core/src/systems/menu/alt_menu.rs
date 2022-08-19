use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::audio::sound_event::SoundEvent;
use crate::config::settings::progression::Progression;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::loading::assets::SoundType;
use crate::states::AppState;
use crate::systems::menu::button::{add_btn, DsfButton, MenuButtons};
use crate::util::files::get_levels_dir;

const BUTTON_CONT: &str = "Continue";
const BUTTON_START: &str = "New Game";
const BUTTON_EXIT: &str = "Exit";
const COOLDOWN: f32 = 2.;

pub fn setup_alt_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    progression: Res<Progression>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut buttons = vec![BUTTON_START.to_string(), BUTTON_EXIT.to_string()];
    if progression.current_level > 0 {
        buttons.insert(0, BUTTON_CONT.to_string());
    }
    let container = commands.spawn_bundle(TransformBundle::default()).id();
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
    time: Res<Time>,
    mut audio: EventWriter<SoundEvent>,
    mut progression: ResMut<Progression>,
    mut instruction: ResMut<LevelSelectionInstruction>,
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut buttons: ResMut<MenuButtons>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(timer) = &mut buttons.timer {
        timer.tick(time.delta());
        if timer.finished() {
            instruction.level = Some(
                get_levels_dir().join(progression.levels.get(progression.current_level).unwrap()),
            );
            commands.insert_resource(NextState(AppState::InGame));
        }
    } else {
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
        if keys.any_just_pressed([KeyCode::Return, KeyCode::NumpadEnter, KeyCode::Space]) {
            keys.clear_just_pressed(KeyCode::Return);
            keys.clear_just_pressed(KeyCode::NumpadEnter);
            keys.clear_just_pressed(KeyCode::Space);
            match buttons.buttons.get(buttons.selected).unwrap().as_str() {
                BUTTON_CONT => {
                    audio.send(SoundEvent::Sfx(SoundType::LevelSelect, true));
                    buttons.timer = Some(Timer::from_seconds(COOLDOWN, false));
                }
                BUTTON_START => {
                    audio.send(SoundEvent::Sfx(SoundType::LevelSelect, true));
                    progression.reset();
                    buttons.timer = Some(Timer::from_seconds(COOLDOWN, false));
                }
                BUTTON_EXIT => {
                    exit.send(AppExit);
                }
                _ => (),
            }
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
