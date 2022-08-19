use std::fs;
use std::path::Path;

use bevy::prelude::*;

use crate::camera::camera_components::FocalPoint;
use crate::config::settings::user_cache::UserCache;
use crate::level_select::structs::{
    Adventure, LevelSelectionInstruction, MapCursor, MapElement, PositionOnMap,
};
use crate::levels::tiles::tile_defs::DepthLayer;
use crate::loading::assets::{AssetStorage, SpriteType};
use crate::systems::motion::structs::pos::Pos;

pub fn on_start(
    mut commands: Commands,
    user_cache: Res<UserCache>,
    storage: Res<AssetStorage>,
    instruction: Res<LevelSelectionInstruction>,
) {
    info!("LevelSelectState on_enter");
    let path = instruction.adventure.as_ref().expect("No adventure!");
    load_adventure(path, &mut commands, &user_cache, &storage);
    load_cursor(&mut commands, &storage, &Pos::new(0, 0));
}

pub fn load_adventure(
    path: &Path,
    commands: &mut Commands,
    user_cache: &Res<UserCache>,
    storage: &Res<AssetStorage>,
) {
    let data = fs::read_to_string(path).expect("Unable to read adventure file");
    let adventure = ron::de::from_str::<Adventure>(&data).expect("Unable to deserialise adventure");
    for (pos, map_element) in &adventure.nodes {
        match map_element {
            MapElement::Road => spawn_road(pos, commands, storage),
            MapElement::Node(_node) => spawn_node(pos, commands, storage),
        }
    }
    let initial_cursor_pos = {
        let last_known_pos = cursor_position(path, user_cache);
        if adventure.nodes.contains_key(&last_known_pos) {
            last_known_pos
        } else {
            Pos::default()
        }
    };
    // load_cursor(world, &initial_cursor_pos);
    commands.insert_resource(adventure);
    commands.insert_resource(PositionOnMap::new(initial_cursor_pos));
}

fn cursor_position(path: &Path, user_cache: &Res<UserCache>) -> Pos {
    user_cache.get_initial_cursor_pos(
        path.file_name()
            .expect("This should not happen.")
            .to_str()
            .expect("Adventure file name did not contain valid unicode."),
    )
}

fn load_cursor(commands: &mut Commands, storage: &Res<AssetStorage>, pos: &Pos) {
    let mut bundle = create_indexed(pos, storage, 3);
    bundle.transform.translation.z = DepthLayer::Player.z();
    bundle.sprite.color = Color::rgb(0.5, 0., 0.);
    commands
        .spawn_bundle(bundle)
        .insert(MapCursor::default())
        .insert(FocalPoint);
}

fn spawn_road(pos: &Pos, commands: &mut Commands, storage: &Res<AssetStorage>) {
    let bundle = create_indexed(pos, storage, 1);
    commands.spawn_bundle(bundle);
}

fn spawn_node(pos: &Pos, commands: &mut Commands, storage: &Res<AssetStorage>) {
    let bundle = create_indexed(pos, storage, 0);
    commands.spawn_bundle(bundle);
}

fn create_indexed(pos: &Pos, storage: &Res<AssetStorage>, index: usize) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas: storage.get_atlas(&SpriteType::LevelSelect),
        transform: Transform::from_xyz(
            pos.x as f32 + 0.5,
            pos.y as f32 + 0.5,
            DepthLayer::Blocks.z(),
        ),
        sprite: TextureAtlasSprite {
            index,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    }
}
