use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};

use crate::audio::play_sfx::{change_audio_settings, play_sfx};
use crate::audio::sound_event::{AudioResource, MusicChannel, SfxChannel, SoundEvent};

pub struct DsfAudioPlugin;

impl Plugin for DsfAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(play_sfx)
            .add_system(change_audio_settings)
            .add_event::<SoundEvent>()
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SfxChannel>()
            .init_resource::<AudioResource>();
    }
}
