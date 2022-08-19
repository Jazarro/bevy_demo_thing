use bevy::prelude::*;

use dsf_core::levels::load_level_system::load_transform;
use dsf_core::levels::tiles::tile_defs::Archetype;
use dsf_core::loading::assets::AssetStorage;
use dsf_core::systems::motion::structs::direction::Direction1D;

use crate::components::painted_tile::PaintedTile;
use crate::resources::level_edit::LevelEdit;

/// Clears all dirty tiles, then adds all dirty tiles back in.
/// At the end of this system's execution, no tiles should be left dirty.
pub fn tile_paint_system(
    mut commands: Commands,
    storage: Res<AssetStorage>,
    mut level_edit: ResMut<LevelEdit>,
    query: Query<(Entity, &PaintedTile)>,
) {
    // First delete all dirty entities:
    for (entity, _) in query
        .iter()
        .filter(|(_, &painted_tile)| level_edit.dirty.contains(&painted_tile.pos))
    {
        commands.entity(entity).despawn_recursive();
    }

    // Then create new entities for all dirty positions.
    // These are the entities that were changed or newly added.
    level_edit
        .drain_dirty()
        .drain(..)
        // Do not create entities for dummy tiles:
        .filter(|pos| level_edit.tile_map.is_tile_def_key(pos))
        .map(|dirty_pos| {
            let tile_def = level_edit.tile_map.get_tile(&dirty_pos).expect(
                "Cannot panic, we previously checked that there is a proper tile in this location.",
            );
            (dirty_pos, tile_def)
        })
        .for_each(|(pos, tile_def)| {
            let atlas = storage.get_atlas(&tile_def.get_preview().0);
            let flip = tile_def.archetype == Some(Archetype::RevolvingDoor(Direction1D::Negative));
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas,
                    transform: load_transform(&pos, &tile_def.dimens, &tile_def.depth),
                    sprite: TextureAtlasSprite {
                        index: tile_def.get_preview().1,
                        flip_x: flip,
                        ..default()
                    },
                    ..default()
                })
                .insert(PaintedTile::new(pos));
        });
}
