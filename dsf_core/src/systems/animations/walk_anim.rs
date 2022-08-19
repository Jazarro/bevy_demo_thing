use crate::systems::animations::structs::AnimationTimer;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;
use crate::systems::motion::structs::steering_mode::SteeringMode;
use bevy::prelude::*;

pub fn animate_walking(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationTimer,
        &SteeringIntent,
        &Steering,
    )>,
) {
    for (mut sprite, mut anim, intent, steering) in query.iter_mut() {
        let is_walking =
            intent.walk != Direction1D::Neutral && steering.mode == SteeringMode::Grounded;
        let is_climbing =
            intent.climb != Direction1D::Neutral && steering.mode == SteeringMode::Climbing;
        if is_walking || is_climbing {
            sprite.index = anim.tick(time.delta());
        } else {
            sprite.index = 2;
        }
    }
}
