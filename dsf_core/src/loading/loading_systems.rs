use std::fs;
use std::path::PathBuf;

use bevy::asset::LoadState;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::config::editor_config::EditorConfig;
use crate::config::loading_config::LoadingConfig;
use crate::config::movement_config::MovementConfig;
use crate::config::settings::audio_settings::AudioSettings;
use crate::config::settings::debug_settings::DebugSettings;
use crate::config::settings::progression::Progression;
use crate::level_select::structs::LevelSelectionInstruction;
use crate::loading::assets::AssetStorage;
use crate::loading::atlas_prefab::AtlasPrefab;
use crate::states::AppState;
use crate::util::files::{get_assets_dir, get_atlases_dir, get_levels_dir};

#[derive(Default, Debug)]
pub struct LoadingAssets {
    handles: Vec<HandleUntyped>,
}

pub fn load_assets(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut storage: ResMut<AssetStorage>,
) {
    let config = LoadingConfig::load_from_file();
    let mut loading_assets = LoadingAssets::default();
    for (sprite_type, path) in config.atlases {
        let file = get_atlases_dir().join(path);
        let data = fs::read_to_string(&file).expect("Unable to read file");
        let from_grid = ron::de::from_str::<AtlasPrefab>(&data)
            .unwrap_or_else(|_| panic!("Unable to deserialise AtlasPrefab at path {:?}", &file));
        let texture_handle = assets.load(PathBuf::new().join("textures/").join(&from_grid.texture));
        loading_assets.handles.push(texture_handle.clone_untyped());
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            from_grid.tile_size,
            from_grid.columns,
            from_grid.rows,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        storage.put_atlas(sprite_type, texture_atlas_handle);
    }

    for (sound_type, path) in config.sound_effects {
        let asset_path = PathBuf::new().join("audio/sfx/").join(path);
        let file = get_assets_dir().join(&asset_path);
        if file.is_file() {
            let handle = assets.load(asset_path);
            loading_assets.handles.push(handle.clone_untyped());
            storage.put_sound(sound_type, handle);
        } else if file.is_dir() {
            for handle in assets.load_folder(asset_path).unwrap() {
                loading_assets.handles.push(handle.clone());
                storage.put_sound(sound_type, handle.typed());
            }
        } else {
            warn!("Did not recognise path {:?}", asset_path);
        }
    }

    for (music_type, path) in config.music {
        let asset_path = PathBuf::new().join("audio/music/").join(path);
        let file = get_assets_dir().join(&asset_path);
        if file.is_file() {
            let handle = assets.load(asset_path);
            loading_assets.handles.push(handle.clone_untyped());
            storage.put_music(music_type, handle);
        } else if file.is_dir() {
            for handle in assets.load_folder(asset_path).unwrap() {
                loading_assets.handles.push(handle.clone());
                storage.put_music(music_type, handle.typed());
            }
        } else {
            warn!("Did not recognise path {:?}", asset_path);
        }
    }

    commands.insert_resource(loading_assets);
}

pub fn load_configs(mut commands: Commands) {
    commands.insert_resource(AudioSettings::load_from_file());
    commands.insert_resource(DebugSettings::load_from_file());
    commands.insert_resource(Progression::load_from_file());

    commands.insert_resource(MovementConfig::load_from_file());
    commands.insert_resource(EditorConfig::load_from_file());
}

/// TODO: In case of failure: print on screen what assets loaded successfully and which didn't.
///     Allow user to skip to main menu by pressing the any key.
pub fn check_load_state(
    mut commands: Commands,
    mut instruction: ResMut<LevelSelectionInstruction>,
    asset_server: Res<AssetServer>,
    loading_assets: Res<LoadingAssets>,
    config: Res<DebugSettings>,
) {
    match asset_server.get_group_load_state(loading_assets.handles.iter().map(|h| h.id)) {
        LoadState::Failed => {
            error!("Failed loading assets");
            // one of our assets had an error
        }
        LoadState::Loaded => {
            if config.use_alternate_menu {
                info!("Done loading, switching to alt menu!");
                commands.insert_resource(NextState(AppState::AltMenu));
            } else if config.skip_straight_to_editor {
                info!("Done loading, switching to level editor!");
                instruction.level = Some(get_levels_dir().join("auto_save.ron"));
                commands.insert_resource(NextState(AppState::LevelEditor));
            } else {
                info!("Done loading, switching to main menu!");
                commands.insert_resource(NextState(AppState::MainMenu));
            }
        }
        _ => {
            info!("Busy loading {} things...", loading_assets.handles.len());
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
