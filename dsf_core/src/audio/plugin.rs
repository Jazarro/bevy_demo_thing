use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};

use crate::audio::play_sfx::play_sfx;
use crate::audio::sound_event::{AudioResource, MusicChannel, SfxChannel, SoundEvent};

pub struct DsfAudioPlugin;

impl Plugin for DsfAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(play_sfx)
            .add_event::<SoundEvent>()
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SfxChannel>()
            // .init_resource::<ChannelAudioState::<MusicChannel>>()
            // .init_resource::<ChannelAudioState::<SfxChannel>>()
            .init_resource::<AudioResource>();
    }
}
