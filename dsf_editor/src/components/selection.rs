use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for the selection entity.
/// There should always be one of these (when in the editor).
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
#[serde(deny_unknown_fields)]
pub struct SelectionTag;
