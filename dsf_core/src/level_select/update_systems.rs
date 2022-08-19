use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::audio::sound_event::SoundEvent;
use crate::config::movement_config::MovementConfig;
use crate::level_select::structs::{
    Adventure, AdventureNode, LevelSelectionInstruction, MapCursor, MapElement, NodeDetails,
    PositionOnMap,
};
use crate::loading::assets::SoundType;
use crate::states::AppState;
use crate::systems::motion::structs::direction::Direction2D;
use crate::util::files::get_levels_dir;

pub fn check_input(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    pos_on_map: Res<PositionOnMap>,
    adventure: ResMut<Adventure>,
    mut instruction: ResMut<LevelSelectionInstruction>,
) {
    if keys.any_just_pressed([KeyCode::Return, KeyCode::NumpadEnter]) {
        keys.clear_just_pressed(KeyCode::Return);
        keys.clear_just_pressed(KeyCode::NumpadEnter);

        // - If the user selected a road, nothing will happen.
        // - If the user selected a level, that level will be opened in the Play state.
        // - If the user selected an adventure, that adventure will be opened in a nested `LevelSelect` state.
        if let Some(MapElement::Node(AdventureNode {
            details: NodeDetails::Level(level_name),
            ..
        })) = adventure.nodes.get(&pos_on_map.pos)
        {
            instruction.level = Some(get_levels_dir().join(level_name));
            commands.insert_resource(NextState(AppState::InGame));
        }
    }
}

/// Responsible for moving the map cursor in the adventure and level selection.
pub fn update_cursor(
    time: Res<Time>,
    keys: ResMut<Input<KeyCode>>,
    mut pos_on_map: ResMut<PositionOnMap>,
    adventure: Res<Adventure>,
    config: Res<MovementConfig>,
    mut query: Query<(&mut MapCursor, &mut Transform)>,
    mut audio: EventWriter<SoundEvent>,
) {
    for (mut cursor, mut transform) in query.iter_mut() {
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
        let down = keys.any_pressed([KeyCode::S, KeyCode::Down]);
        let up = keys.any_pressed([KeyCode::W, KeyCode::Up]);
        let new_direction = Direction2D::from_input(left, right, down, up);
        if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
            // Start movement now. Move once, then set cooldown to High.
            move_cursor(
                new_direction,
                &mut pos_on_map,
                &mut transform,
                &adventure,
                &mut audio,
            );
            cursor.cooldown = config.map_cursor_move_high_cooldown;
        } else if cursor.last_direction.is_opposite(&new_direction) {
            // Reset movement. Set cooldown to high.
            cursor.cooldown = config.map_cursor_move_high_cooldown;
        } else if !new_direction.is_neutral() {
            // continue movement. Tick down cooldown.
            // If cooldown is due, move once and reset cooldown to Low.
            cursor.cooldown -= time.delta_seconds();
            if cursor.cooldown.is_sign_negative() {
                cursor.cooldown = config.map_cursor_move_low_cooldown;
                move_cursor(
                    new_direction,
                    &mut pos_on_map,
                    &mut transform,
                    &adventure,
                    &mut audio,
                );
            }
        }
        cursor.last_direction = new_direction;
    }
}

/// Move on both x and y directions if possible. If the target position is not available, move
/// on just the x-axis. If that position is not available either, move on just the y-axis.
fn move_cursor(
    direction: Direction2D,
    pos_on_map: &mut PositionOnMap,
    transform: &mut Transform,
    adventure: &Adventure,
    audio: &mut EventWriter<SoundEvent>,
) {
    let target_pos = if direction.x.is_neutral() {
        pos_on_map.pos.append_y(direction.y.signum_i())
    } else {
        pos_on_map.pos.append_x(direction.x.signum_i())
    };

    if adventure.nodes.contains_key(&target_pos) {
        pos_on_map.pos = target_pos;
        transform.translation.x = pos_on_map.pos.x as f32 + 0.5;
        transform.translation.y = pos_on_map.pos.y as f32 + 0.5;
        audio.send(SoundEvent::Sfx(SoundType::MapStep, false))
    }
}

/// Updates the UI label on the adventure and level select screen. The label must always display the
/// name of the currently selected node.
pub fn update_ui() {}

// #[derive(Copy, Clone, Debug)]
// pub struct LevelSelectUiUpdateSystem;
//
// impl<'s> System<'s> for LevelSelectUiUpdateSystem {
//     type SystemData = (
//         WriteStorage<'s, UiText>,
//         UiFinder<'s>,
//         Read<'s, Adventure>,
//         Read<'s, PositionOnMap>,
//     );
//
//     fn run(&mut self, (mut ui_text, finder, adventure, pos_on_map): Self::SystemData) {
//         let label_title = {
//             let label_title_entity = finder.find("label_node_title");
//             label_title_entity.and_then(|fps_entity| ui_text.get_mut(fps_entity))
//         };
//         if let Some(mut label_title) = label_title {
//             let selected = adventure.nodes.get(&pos_on_map.pos);
//             let selected_title = match selected {
//                 Some(MapElement::Node(AdventureNode {
//                     details: NodeDetails::Level(file_name),
//                     ..
//                 })) => file_name,
//                 _ => "Nothing",
//             };
//             label_title.text = format!("Selected: {:?}", selected_title);
//         }
//     }
// }
