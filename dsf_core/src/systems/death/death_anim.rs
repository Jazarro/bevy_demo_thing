use std::f32::consts;

use crate::audio::sound_event::SoundEvent;
use crate::loading::assets::SoundType;
use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::states::AppState;
use crate::systems::motion::structs::player::Player;

#[derive(Clone, Default, Component)]
pub struct Dying {
    seconds_passed: f32,
    transform: Option<Transform>,
}

pub fn is_dying(query: Query<Option<&Dying>, With<Player>>) -> bool {
    if let Ok(Some(_)) = query.get_single() {
        true
    } else {
        false
    }
}

pub fn animate_death(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Dying)>,
) {
    if let Ok((mut transform, mut dying)) = query.get_single_mut() {
        if let None = dying.transform {
            audio.send(SoundEvent::Sfx(SoundType::Death, true));
            dying.transform = Some(transform.clone());
        }
        dying.seconds_passed += time.delta_seconds();
        if dying.seconds_passed > 3. {
            commands.insert_resource(NextState(AppState::InGame));
        } else if dying.seconds_passed > 2.5 {
            // No-op.
        } else if dying.seconds_passed < 2. {
            transform.rotation = Quat::from_rotation_y(dying.seconds_passed * consts::TAU * 4.);
        } else {
            transform.translation = dying.transform.unwrap().translation.clone();
            transform.rotation = dying.transform.unwrap().rotation.clone();
            let mut foot = transform.translation.clone();
            foot.y -= 1.;
            transform.rotate_around(
                foot,
                Quat::from_rotation_x(((dying.seconds_passed - 2.0) * 0.5) * consts::TAU),
            );
        }
    }
}
