use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize, Serializer};

use crate::systems::motion::structs::direction::Direction2D;
use crate::systems::motion::structs::pos::Pos;

/// This is used in the adventure and level selector. The entity with this component represents
/// where the player is on the map.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct MapCursor {
    pub last_direction: Direction2D,
    pub cooldown: f32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PositionOnMap {
    pub pos: Pos,
}

impl PositionOnMap {
    pub fn new(pos: Pos) -> Self {
        PositionOnMap { pos }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LevelSelectionInstruction {
    pub adventure: Option<PathBuf>,
    pub level: Option<PathBuf>,
    pub editor_open: bool,
}

/// All adventures must start at position (0, 0).
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Adventure {
    #[serde(serialize_with = "ordered_map")]
    pub(crate) nodes: HashMap<Pos, MapElement>,
}

/// A function used by serde to serialise the tile map in a deterministic way.
/// This will prevent the output being different each time the level is saved, which will
/// prevent lots of unnecessarily large diffs in the git commits.
fn ordered_map<S>(value: &HashMap<Pos, MapElement>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MapElement {
    Road,
    Node(AdventureNode),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdventureNode {
    pub name: String,
    // pub description: String,
    pub details: NodeDetails,
    // If true, the player must defeat this node before they can move further.
    // If false, nodes behind this node are reachable and playable even if this node was never
    // entered.
    // pub blocking: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeDetails {
    /// This node is an adventure: a collection of levels.
    /// Opening this node will push a new LevelSelectState for this adventure.
    Adventure(String),
    /// This node is a level. Opening this node will open the level in the PlayState.
    Level(String),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Road {
    pub start_id: u16,
    pub end_id: u16,
}
