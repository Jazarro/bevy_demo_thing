use bevy::prelude::*;
use bevy::text::Text2dSize;

use crate::levels::tiles::tile_defs::DepthLayer;
use crate::levels::tiles::tilemap::TileMap;
use crate::systems::motion::structs::pos::Pos;

pub fn setup_hud(mut commands: Commands, tile_map: Res<TileMap>, assets: Res<AssetServer>) {
    info!("setup_hud");
    let font = assets.load("fonts/square.ttf");

    let pos = Pos::new(32, 23) + tile_map.world_bounds.pos;
    spawn("MOVE:", pos, &mut commands, font.clone());
    spawn("ARROWS/WASD", pos.append_y(-1), &mut commands, font.clone());

    let pos = pos.append_y(-3);
    spawn("JUMP:", pos, &mut commands, font.clone());
    spawn("SPACE", pos.append_y(-1), &mut commands, font.clone());

    let pos = pos.append_y(-3);
    spawn("USE TOOLS:", pos, &mut commands, font.clone());
    spawn("SPACE", pos.append_y(-1), &mut commands, font.clone());

    let pos = pos.append_y(-3);
    spawn("RESET:", pos, &mut commands, font.clone());
    spawn("F5", pos.append_y(-1), &mut commands, font.clone());

    let pos = pos.append_y(-3);
    spawn("BACK:", pos, &mut commands, font.clone());
    spawn("ESC", pos.append_y(-1), &mut commands, font.clone());
}

fn spawn(phrase: &str, pos: Pos, commands: &mut Commands, font: Handle<Font>) {
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Left,
    };
    let transform =
        Transform::from_xyz(pos.x as f32 + 0.5, pos.y as f32, DepthLayer::UiElements.z())
            .with_scale(Vec3::new(1. / 128., 1. / 128., 1.));
    commands.spawn().insert_bundle(Text2dBundle {
        text: Text::with_section(phrase, text_style, text_alignment),
        text_2d_size: Text2dSize {
            size: Size::new(5., 1.),
        },
        transform,
        ..default()
    });
}
