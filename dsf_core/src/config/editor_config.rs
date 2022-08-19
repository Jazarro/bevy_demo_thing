use std::fs;

use serde::{Deserialize, Serialize};

use crate::util::files::get_config_dir;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EditorConfig {
    pub cursor_move_high_cooldown: f32,
    pub cursor_move_low_cooldown: f32,
    /// Time in seconds that the cursor is visible during its blinking animation.
    pub cursor_blink_on_time: f32,
    /// Time in seconds that the cursor is invisible during its blinking animation.
    pub cursor_blink_off_time: f32,
}

impl EditorConfig {
    /// Loads the LoadingConfig from file.
    #[must_use]
    pub fn load_from_file() -> EditorConfig {
        let file = get_config_dir().join("editor.ron");
        let data = fs::read_to_string(file).expect("Unable to read editor config file");
        ron::de::from_str::<EditorConfig>(&data).expect("Unable to deserialise EditorConfig")
    }
}
