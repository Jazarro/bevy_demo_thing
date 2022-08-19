use crate::components::cursor::PreviewGhostTag;
use crate::components::painted_tile::PaintedTile;
use crate::resources::blueprint::Blueprint;
use crate::resources::level_edit::{LevelEdit, PlaceTileDryRun};
use crate::resources::status::editor_status::EditorStatus;
use bevy::prelude::*;
use dsf_core::levels::load_level_system::load_transform;
use dsf_core::levels::tiles::tile_defs::Archetype;
use dsf_core::loading::assets::AssetStorage;
use dsf_core::systems::motion::structs::direction::Direction1D;

/// Send this through the event bus in order to trigger a complete refresh of the previews.
#[derive(Debug, Copy, Clone)]
pub struct RefreshPreviewsEvent;

/// Responsible for refreshing the preview when it receives the signal to do so through its event
/// bus. This will add a red tint to all existing tiles that are due to be removed. It will also
/// add ghost images for all the tiles that are due to be added.
pub fn refresh_previews(
    mut channel: EventReader<RefreshPreviewsEvent>,
    status: Res<EditorStatus>,
    level_edit: Res<LevelEdit>,
    mut commands: Commands,
    storage: Res<AssetStorage>,
    query_ghost: Query<Entity, With<PreviewGhostTag>>,
    mut query_tile: Query<(&mut TextureAtlasSprite, &PaintedTile)>,
) {
    // We don't care how many events we received, refreshing more than once doesn't do anything.
    // Check if at least one event was received, while still making sure to empty the iterator
    // (very important, otherwise the surplus events stay in the channel until next frame).
    let at_least_one_event = channel.iter().fold(false, |_, _| true);
    if !at_least_one_event {
        return;
    }
    let blueprint = Blueprint::from_placing_tiles(&status, &level_edit);
    let lower_bounds = status.selection.lower_bounds();
    let blueprint_dry_run =
        blueprint
            .tiles
            .iter()
            .fold(PlaceTileDryRun::default(), |accumulator, (pos, tile)| {
                let place_tile_dry_run = level_edit.check_place_tile(
                    status.force_place,
                    lower_bounds + *pos,
                    Some(tile.clone()),
                );
                accumulator.extend(place_tile_dry_run)
            });

    // Tint existing tiles that are due to be removed red.
    for (mut sprite, painted_tile) in query_tile.iter_mut() {
        sprite.color = if blueprint_dry_run.to_be_removed.contains(&painted_tile.pos) {
            Color::rgba(1., 0., 0., 1.0)
        } else {
            Color::rgba(1., 1., 1., 1.0)
        };
    }
    // First delete all existing previews:
    for entity in query_ghost.iter() {
        commands.entity(entity).despawn();
    }
    // Then create new previews based on the current Blueprint:
    blueprint_dry_run
        .to_be_added
        .iter()
        .for_each(|(pos, key, dimens)| {
            let tile_def = level_edit.tile_map.tile_defs.get(&key);
            let atlas = storage.get_atlas(&tile_def.get_preview().0);
            let flip = tile_def.archetype == Some(Archetype::RevolvingDoor(Direction1D::Negative));
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas,
                    transform: load_transform(pos, &dimens, &tile_def.depth),
                    sprite: TextureAtlasSprite {
                        index: tile_def.get_preview().1,
                        flip_x: flip,
                        color: Color::rgba(0.5, 0.5, 0.5, 0.7),
                        ..default()
                    },
                    ..default()
                })
                .insert(PreviewGhostTag);
        });
}
