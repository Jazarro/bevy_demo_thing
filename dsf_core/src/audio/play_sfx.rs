use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_kira_audio::PlaybackState::Stopped;

use crate::audio::sound_event::{AudioResource, MusicChannel, SfxChannel, SoundEvent};
use crate::config::settings::audio_settings::AudioSettings;
use crate::loading::assets::AssetStorage;

/// This system is responsible for playing non-location-dependent sound effects.
/// To play any sound effect, just broadcast a `SoundEvent` in the corresponding event channel.
/// This system will take care of the rest.
pub fn play_sfx(
    mut resource: ResMut<AudioResource>,
    mut events: EventReader<SoundEvent>,
    assets: Res<AssetStorage>,
    channel_music: Res<AudioChannel<MusicChannel>>,
    channel_sfx: Res<AudioChannel<SfxChannel>>,
    _config: Res<AudioSettings>,
) {
    // channel_music.set_volume(0.1);
    // channel_sfx.set_volume(0.3);
    for event in events.iter() {
        debug!("Received sound event: {:?}", event);
        match event {
            SoundEvent::KillAllSoundEffects => {
                channel_sfx.stop();
                resource.interrupting_sound = None;
            }
            SoundEvent::Sfx(sound_type, interrupt) => {
                if let Some(handle) = assets.get_sound(sound_type) {
                    let instance = channel_sfx.play(handle);
                    if *interrupt {
                        resource.interrupting_sound = Some((instance, -1.));
                        channel_music.pause();
                    }
                } else {
                    info!(
                        "Tried to play SoundType::{:?} but couldn't find that asset.",
                        sound_type
                    );
                }
            }
            SoundEvent::Music(Some((music_type, looped))) => {
                resource.interrupting_sound = None;
                if let Some(handle) = assets.get_music(music_type) {
                    channel_music.stop();
                    if *looped {
                        channel_music.play_looped(handle);
                    } else {
                        channel_music.play(handle);
                    }
                } else {
                    info!(
                        "Tried to play MusicType::{:?} but couldn't find that asset.",
                        music_type
                    );
                }
            }
            SoundEvent::Music(None) => {
                resource.interrupting_sound = None;
                channel_music.stop();
            }
        };
    }
    if let Some((instance, last_position)) = &mut resource.interrupting_sound {
        let state = channel_sfx.state(instance.clone());
        let position = state.position();
        if state == Stopped
            || (position.is_some() && (position.unwrap() - *last_position).abs() < f64::EPSILON)
        {
            channel_music.resume();
            resource.interrupting_sound = None;
        } else {
            *last_position = state.position().unwrap_or(-1.);
        }
    }
}
