use crate::systems::animations::structs::AnimationTimer;
use crate::systems::enemy::spawner::Enemy;
use crate::systems::motion::move_enemy::EnemyAi;
use bevy::prelude::*;

use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;
use crate::systems::motion::structs::velocity::Velocity;

#[derive(Bundle, Clone, Default)]
pub struct PlayerBundle {
    pub velocity: Velocity,
    pub steering_intent: SteeringIntent,
    pub steering: Steering,
    pub player: Player,
    pub anim: AnimationTimer,
    // pub focal_point: FocalPoint,
}

#[derive(Bundle, Clone, Default)]
pub struct EnemyBundle {
    pub velocity: Velocity,
    pub steering_intent: SteeringIntent,
    pub steering: Steering,
    pub enemy: Enemy,
    pub ai: EnemyAi,
    pub anim: AnimationTimer,
}
