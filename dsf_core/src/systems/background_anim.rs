use crate::levels::tiles::background::{BackgroundEyes, BackgroundHeads};
use bevy::prelude::*;
use std::f32::consts;

pub fn anim_background_heads(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BackgroundHeads)>,
) {
    if let Ok((mut transform, mut heads)) = query.get_single_mut() {
        let amplitude = 30.;
        let frequency = 0.5;
        heads.anim = (time.seconds_since_startup() as f32 * consts::PI * frequency).sin();
        transform.translation.y = amplitude * heads.anim;
    }
}

pub fn anim_background_eyes(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut BackgroundEyes)>,
) {
    if let Ok((mut sprite, mut eyes)) = query.get_single_mut() {
        let amplitude = 0.6;
        let frequency = 0.1;
        let sine = ((time.seconds_since_startup() as f32 * consts::PI * frequency).sin() + 1.) / 2.;
        eyes.anim = (sine - 0.4).max(0.);
        sprite.color = Color::rgba(1., 1., 1., amplitude * eyes.anim);
    }
}
