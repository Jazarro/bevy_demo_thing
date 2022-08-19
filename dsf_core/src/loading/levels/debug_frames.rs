use crate::config::settings::debug_settings::DebugSettings;
use crate::levels::tiles::tile_defs::DepthLayer;
use crate::loading::assets::{AssetStorage, SpriteType};
use crate::systems::motion::structs::coords::Coords;
use crate::systems::motion::structs::player::{DebugPosGhostTag, DebugSteeringGhostTag, Player};
use bevy::prelude::*;

pub fn build_frames(
    mut commands: Commands,
    debug_settings: Res<DebugSettings>,
    storage: Res<AssetStorage>,
    query_player: Query<&Coords, With<Player>>,
) {
    if !debug_settings.display_debug_frames {
        return;
    }
    if let Ok(player_coords) = query_player.get_single() {
        let texture_atlas = storage.get_atlas(&SpriteType::Frame);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                transform: Transform::default()
                    .with_scale(Vec3::new(
                        player_coords.dimens.x as f32 / 50.,
                        player_coords.dimens.y as f32 / 50.,
                        1.,
                    ))
                    .with_translation(Vec3::new(
                        player_coords.dimens.x as f32 * 0.5,
                        player_coords.dimens.y as f32 * 0.5,
                        DepthLayer::UiElements.z(),
                    )),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(DebugSteeringGhostTag);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas,
                transform: Transform::default()
                    .with_scale(Vec3::new(1. / 50., 1. / 50., 1.))
                    .with_translation(Vec3::new(0.5, 0.5, DepthLayer::UiElements.z())),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(DebugPosGhostTag);
    }
}
