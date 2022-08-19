use bevy::prelude::*;
use bevy_kira_audio::InstanceHandle;

use crate::loading::assets::{MusicType, SoundType};

/// Elsewhere in the application, you can broadcast `SoundEvents`. The `PlaySfxSystem` below listens
/// for such events and actually plays the sound effect that was requested.
#[derive(Debug)]
pub enum SoundEvent {
    /// SoundType and whether the music should be interrupted during play.
    Sfx(SoundType, InterruptMusic),
    /// MusicType. If present, play this song. Otherwise, stop all music.
    Music(Option<(MusicType, Looped)>),
    KillAllSoundEffects,
}

pub type InterruptMusic = bool;
pub type Looped = bool;

#[derive(Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Component, Default, Clone)]
pub struct SfxChannel;

#[derive(Default)]
pub struct AudioResource {
    /// Sound effect that has interrupted the music. After this is done playing, the music should resume.
    /// Contains an instance handle and an optional position, to keep track of when it stops playing.
    pub interrupting_sound: Option<(InstanceHandle, f64)>,
}
