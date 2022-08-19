use bevy::prelude::*;

use crate::levels::tiles::tile_defs::TileDefinition;
use crate::levels::tiles::tilemap::TileMap;
use crate::levels::world_bounds::WorldBounds;
use crate::systems::enemy::spawner::Enemy;
use crate::systems::motion::structs::direction::Direction1D;
use crate::systems::motion::structs::player::Player;
use crate::systems::motion::structs::pos::Pos;
use crate::systems::motion::structs::steering::Steering;
use crate::systems::motion::structs::steering_intent::SteeringIntent;

const COOLDOWN: f32 = 1.;

#[derive(Clone, Component)]
pub struct EnemyAi {
    pub state: AiState,
}

impl Default for EnemyAi {
    fn default() -> Self {
        EnemyAi {
            state: AiState::MakeNewPlan(Timer::from_seconds(COOLDOWN, false)),
        }
    }
}

#[derive(Clone)]
pub enum AiState {
    MakeNewPlan(Timer),
    Walking,
}

pub fn set_enemy_steering_intent(
    tile_map: Res<TileMap>,
    time: Res<Time>,
    mut query_enemy: Query<(&mut EnemyAi, &mut SteeringIntent, &Steering), With<Enemy>>,
    query_player: Query<&Steering, With<Player>>,
) {
    for (mut ai, mut intent, steering) in query_enemy.iter_mut() {
        match &mut ai.state {
            AiState::MakeNewPlan(timer) => {
                timer.tick(time.delta());
                if timer.finished() {
                    let right_is_blocked = any_collide(
                        &tiles_to_side(&Direction1D::Positive, &steering, &tile_map.world_bounds),
                        &tile_map,
                    );
                    let left_is_blocked = any_collide(
                        &tiles_to_side(&Direction1D::Negative, &steering, &tile_map.world_bounds),
                        &tile_map,
                    );
                    let player_pos = query_player
                        .get_single()
                        .map(|player_steering| player_steering.pos)
                        .unwrap_or(Pos::default());
                    let preferred_direction =
                        Direction1D::new((player_pos - steering.pos).x.signum() as f32);
                    let direction = if !left_is_blocked
                        && (preferred_direction != Direction1D::Positive || right_is_blocked)
                    {
                        Direction1D::Negative
                    } else if !right_is_blocked
                        && (preferred_direction != Direction1D::Negative || left_is_blocked)
                    {
                        Direction1D::Positive
                    } else {
                        Direction1D::Neutral
                    };

                    if direction == Direction1D::Neutral {
                        // TODO: Suicide.
                    } else {
                        ai.state = AiState::Walking;
                        intent.walk = direction;
                    }
                }
            }
            AiState::Walking => {
                let tiles = tiles_to_side(&steering.facing.x, &steering, &tile_map.world_bounds);
                if any_collide(&tiles, &tile_map) {
                    ai.state = AiState::MakeNewPlan(Timer::from_seconds(COOLDOWN, false));
                    intent.walk = Direction1D::Neutral;
                }
            }
        }
    }
}

// TODO: This was copied almost verbatim from tools. Get rid of duplicate code.
//          - It's also very similar to code in steering systems.
fn tiles_to_side(facing: &Direction1D, steering: &Steering, bounds: &WorldBounds) -> Vec<Pos> {
    let facing_offset = if facing.is_positive() {
        steering.dimens.x
    } else {
        -1
    };
    let depth = 1;
    (0..(i32::from(depth)))
        .flat_map(|x| {
            (0..steering.dimens.y).map(move |y| (x, y)) //???
        })
        .map(|(x_offset, y_offset)| {
            Pos::new(
                steering.pos.x + facing_offset + x_offset * facing.signum_i(),
                steering.pos.y + y_offset,
            )
        })
        .map(|pos| bounds.wrapped(&pos))
        .collect()
}

fn any_collide(blocks: &[Pos], tile_map: &TileMap) -> bool {
    blocks.iter().any(|pos| {
        tile_map
            .get_tile(pos)
            .map_or(false, TileDefinition::collides_horizontally)
    })
}
