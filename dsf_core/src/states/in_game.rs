use bevy::app::CoreStage::Update;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::camera::camera_systems::camera_follow_focal_point;
use crate::camera::create_camera::create_camera;
use crate::loading::levels::debug_frames::build_frames;
use crate::loading::levels::keys_on_door::add_key_displays_to_door;
use crate::loading::levels::load_level_system::load_level;
use crate::states::{back_on_escape, delete_all_entities, start_music, AppState};
use crate::systems::animations::walk_anim::animate_walking;
use crate::systems::background_anim::{anim_background_eyes, anim_background_heads};
use crate::systems::check_input::check_in_game_input;
use crate::systems::death::death_anim::{animate_death, is_dying};
use crate::systems::debug::debug_system;
use crate::systems::enemy::kill::enemy_kill;
use crate::systems::enemy::spawner::activate_spawners;
use crate::systems::menu::setup_hud::setup_hud;
use crate::systems::motion::move_enemy::set_enemy_steering_intent;
use crate::systems::motion::move_player::set_player_steering_intent;
use crate::systems::motion::movement::{movement_system, velocity_system};
use crate::systems::motion::steering::steering_system;
use crate::systems::revolving_door::{
    control_revolving_doors, control_revolving_sprites, set_revolving_controllers,
};
use crate::systems::tools::{pickup_system, use_tool_system};
use crate::systems::trap_wall::{trap_mechanism, trigger_trap_walls};
use crate::systems::win_checking::{check_if_won, key_collect_system};
use crate::systems::win_handling::{
    clean_resources, handle_win_door, handle_win_player, handle_win_queued, has_won,
};

pub struct LevelLoaded;

pub struct InGameState;

impl Plugin for InGameState {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoaded>()
            .add_enter_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(create_camera)
                    .with_system(load_level)
                    .with_system(start_music)
                    .into(),
            )
            .add_stage_before(Update, "finish_setup", SystemStage::parallel())
            .add_system_set_to_stage(
                "finish_setup",
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .run_on_event::<LevelLoaded>()
                    .with_system(set_revolving_controllers)
                    .with_system(setup_hud)
                    .with_system(add_key_displays_to_door)
                    .with_system(build_frames)
                    .into(),
            )
            .add_stage_before(Update, "set_intent", SystemStage::parallel())
            .add_system_set_to_stage(
                "set_intent",
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .run_if_not(has_won)
                    .with_system(set_player_steering_intent.run_if_not(is_dying))
                    .with_system(set_enemy_steering_intent)
                    .into(),
            )
            .add_stage_after("set_intent", "set_steering", SystemStage::parallel())
            .add_system_set_to_stage(
                "set_steering",
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .run_if_not(has_won)
                    .with_system(steering_system)
                    .into(),
            )
            .add_stage_after("set_steering", "set_movement", SystemStage::parallel())
            .add_system_set_to_stage(
                "set_movement",
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .run_if_not(has_won)
                    .with_system(movement_system)
                    .into(),
            )
            .add_stage_after("set_movement", "other", SystemStage::parallel())
            .add_system_set_to_stage(
                "other",
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(back_on_escape.run_if_not(has_won))
                    .with_system(camera_follow_focal_point)
                    // .with_system(camera_control)
                    .with_system(velocity_system.run_if_not(has_won))
                    .with_system(pickup_system)
                    .with_system(use_tool_system)
                    .with_system(key_collect_system)
                    .with_system(debug_system)
                    // .with_system(rewind_control_system)
                    // .with_system(rewind_system)
                    .with_system(anim_background_heads)
                    .with_system(anim_background_eyes)
                    .with_system(animate_walking.run_if_not(has_won))
                    .with_system(animate_death.run_if_not(has_won))
                    .with_system(check_in_game_input.run_if_not(is_dying))
                    .with_system(activate_spawners.run_if_not(has_won))
                    .with_system(enemy_kill.run_if_not(is_dying))
                    .with_system(trigger_trap_walls.run_if_not(has_won))
                    .with_system(trap_mechanism.run_if_not(has_won))
                    .with_system(control_revolving_sprites.run_if_not(has_won))
                    .with_system(control_revolving_doors.run_if_not(has_won))
                    .with_system(check_if_won.run_if_not(has_won))
                    .with_system(handle_win_queued.run_if(has_won))
                    .with_system(handle_win_door.run_if(has_won))
                    .with_system(handle_win_player.run_if(has_won))
                    .into(),
            )
            .add_exit_system_set(
                AppState::InGame,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(delete_all_entities)
                    .with_system(clean_resources)
                    .into(),
            );
    }
}
