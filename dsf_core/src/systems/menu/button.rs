use bevy::prelude::*;

pub struct MenuButtons {
    pub selected: usize,
    pub buttons: Vec<String>,
    pub timer: Option<Timer>,
}

#[derive(Component)]
pub struct DsfButton(pub ButtonIndex);

pub type ButtonIndex = usize;

pub fn add_btn(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    buttons: &Vec<String>,
    index: usize,
    container: Entity,
) {
    let container_offset = (150. * buttons.len() as f32 / 2.) - (150. / 2.);
    let font = assets.load("fonts/square.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let btn = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(400.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                container_offset - (150. * index as f32),
                0.,
            )),
            ..default()
        })
        .insert(DsfButton(index))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(DsfButton(index))
                .insert_bundle(Text2dBundle {
                    text: Text::from_section(buttons.get(index).unwrap(), text_style)
                        .with_alignment(text_alignment),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                });
        })
        .id();
    commands.entity(container).push_children(&[btn]);
}
