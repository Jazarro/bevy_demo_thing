use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::audio::sound_event::SoundEvent;
use crate::config::settings::progression::Progression;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::levels::tiles::objects::ExitDoor;
use crate::loading::assets::SoundType;
use crate::states::AppState;
use crate::systems::animations::structs::AnimationTimer;
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::Player;
use crate::util::files::get_levels_dir;

const TIME_DOOR_ANIM: f32 = 1.;
const TIME_PLAYER_ANIM: f32 = 10.;

/// The presence of this resource indicates that the player has won the level.
pub struct WinResource {
    pub state: WinState,
}

impl Default for WinResource {
    fn default() -> Self {
        WinResource {
            state: WinState::Queued,
        }
    }
}

pub enum WinState {
    Queued,
    AnimatingDoor(Timer),
    AnimatingPlayer(Timer),
}

pub fn has_won(win: Option<Res<WinResource>>) -> bool {
    win.is_some()
}

pub fn handle_win_queued(
    mut instructions: ResMut<LevelSelectionInstruction>,
    mut progression: ResMut<Progression>,
    mut audio: EventWriter<SoundEvent>,
    mut win: ResMut<WinResource>,
    mut query_player: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut AnimationTimer),
        With<Player>,
    >,
    query_doors: Query<&Coords, With<ExitDoor>>,
) {
    if let WinState::Queued = win.state {
        audio.send(SoundEvent::Sfx(SoundType::Win, true));
        progression.increment();
        instructions.level =
            Some(get_levels_dir().join(progression.levels.get(progression.current_level).unwrap()));
        win.state = WinState::AnimatingDoor(Timer::from_seconds(TIME_DOOR_ANIM, true));
        if let Ok((mut sprite, mut transform, mut anim)) = query_player.get_single_mut() {
            sprite.index = 2;
            anim.timer = Timer::new(Duration::from_millis(100), true);
            if let Some(door) = query_doors.iter().next() {
                transform.translation.x = door.pos.x as f32 + door.dimens.x as f32 / 2.;
                transform.translation.y = door.pos.y as f32 + 1.;
            }
        }
    }
}

pub fn handle_win_door(
    time: Res<Time>,
    mut win: ResMut<WinResource>,
    mut query_door: Query<&mut TextureAtlasSprite, With<ExitDoor>>,
) {
    let mut finished = false;
    if let WinState::AnimatingDoor(timer) = &mut win.state {
        timer.tick(time.delta());
        for mut sprite in query_door.iter_mut() {
            if timer.finished() {
                sprite.index = 2;
                finished = true;
            } else if timer.percent() > 0.5 {
                sprite.index = 1;
            } else {
                sprite.index = 0;
            }
        }
    }
    if finished {
        win.state = WinState::AnimatingPlayer(Timer::from_seconds(TIME_PLAYER_ANIM, false));
    }
}

pub fn handle_win_player(
    mut commands: Commands,
    time: Res<Time>,
    mut win: ResMut<WinResource>,
    mut query_player: Query<
        (&mut TextureAtlasSprite, &mut Transform, &mut AnimationTimer),
        With<Player>,
    >,
) {
    if let WinState::AnimatingPlayer(timer) = &mut win.state {
        timer.tick(time.delta());
        if timer.finished() {
            commands.insert_resource(NextState(AppState::InGame));
        } else if let Ok((mut sprite, mut transform, mut anim)) = query_player.get_single_mut() {
            sprite.index = anim.tick(time.delta());
            sprite.color = Color::rgba(1., 1., 1., 1. - timer.percent());
            let scale = 1. - (timer.percent() / 2.);
            transform.scale = Vec3::new(scale, scale, 1.);
        }
    }
}

pub fn clean_resources(mut commands: Commands) {
    commands.remove_resource::<WinResource>();
}
