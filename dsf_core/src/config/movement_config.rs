use std::fs;

use serde::{Deserialize, Serialize};

use crate::util::files::get_config_dir;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MovementConfig {
    /// The max speed of the player in meters per second.
    pub player_speed: f32,
    /// The max speed of the enemies in meters per second.
    pub enemy_speed: f32,
    /// How many seconds can pass between starting your jump and starting to move sideways for it to
    /// still register. If you start moving sideways later than that, it will not work and the
    /// character will simply jump straight up into the air instead.
    pub jump_allowance: f32,
    /// How many seconds must pass after turning around whilst standing still before the character
    /// starts walking. This gives the player a bit of time to let go of the walking controls if
    /// they just want to turn around, but not want to start walking.
    pub turn_allowance: f32,
    /// When the player first starts pressing down a movement key (e.g. RIGHT), how many seconds
    /// does it take between moving the first step and moving the second step? The first step is
    /// taken instantly, the second step takes a while. This prevents a single key press registering
    /// as more than one step.
    pub map_cursor_move_high_cooldown: f32,
    /// When the player is holding down a movement key (e.g. RIGHT), how many seconds between two
    /// steps? The first step takes longer, that's what the high cooldown is for. Each subsequent
    /// step takes much shorter.
    pub map_cursor_move_low_cooldown: f32,
}

impl MovementConfig {
    /// Loads the MovementConfig from file.
    #[must_use]
    pub fn load_from_file() -> MovementConfig {
        let file = get_config_dir().join("movement.ron");
        let data = fs::read_to_string(file).expect("Unable to read movement config file");
        ron::de::from_str::<MovementConfig>(&data).expect("Unable to deserialise MovementConfig")
    }
}
