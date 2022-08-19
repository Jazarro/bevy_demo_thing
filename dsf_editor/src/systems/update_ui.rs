
// /// Updates the UI images for the copy-air and force-place flags and for the active brush.
// pub fn editor_ui_update() {
//
// }
//
// #[derive(Copy, Clone, Debug)]
// pub struct EditorUiUpdateSystem;
//
// impl<'s> System<'s> for EditorUiUpdateSystem {
//     type SystemData = (
//         WriteStorage<'s, UiImage>,
//         UiFinder<'s>,
//         Read<'s, EditorStatus>,
//         Read<'s, LevelEdit>,
//         Read<'s, Assets>,
//     );
//
//     fn run(&mut self, (mut ui_image, finder, status, level_edit, assets): Self::SystemData) {
//         let toggle_copy_air = get_image("toggle_copy_air", &finder, &mut ui_image);
//         if let Some(toggle_copy_air) = toggle_copy_air {
//             let sprite_nr = if status.copy_air { 0 } else { 1 };
//             *toggle_copy_air = UiImage::Sprite(load_sprite_render(
//                 SpriteType::EditorUiIcons,
//                 sprite_nr,
//                 &assets,
//             ));
//         }
//         let toggle_force_place = get_image("toggle_force_place", &finder, &mut ui_image);
//         if let Some(toggle_force_place) = toggle_force_place {
//             let sprite_nr = 2 + if status.force_place { 0 } else { 1 };
//             *toggle_force_place = UiImage::Sprite(load_sprite_render(
//                 SpriteType::EditorUiIcons,
//                 sprite_nr,
//                 &assets,
//             ));
//         }
//         let brush_preview = get_image("brush_preview", &finder, &mut ui_image);
//         if let Some(brush_preview) = brush_preview {
//             if let Some(sprite_render) = status
//                 .brush
//                 .get_key()
//                 .as_ref()
//                 .map(|selected_key| level_edit.get_tile_def(selected_key))
//                 .and_then(|tile_def| {
//                     if let AssetType::Still(sprite, sprite_nr) = tile_def.get_preview() {
//                         Some(load_sprite_render(sprite, sprite_nr, &assets))
//                     } else {
//                         None
//                     }
//                 })
//             {
//                 *brush_preview = UiImage::Sprite(sprite_render);
//             } else {
//                 *brush_preview = UiImage::SolidColor([0.0, 0.0, 0.0, 1.0]);
//             }
//         }
//     }
// }
//
// fn get_image<'a>(
//     key: &str,
//     finder: &UiFinder<'_>,
//     ui_image: &'a mut WriteStorage<'_, UiImage>,
// ) -> Option<&'a mut UiImage> {
//     let toggle_entity = finder.find(key);
//     toggle_entity.and_then(move |toggle_entity| ui_image.get_mut(toggle_entity))
// }
