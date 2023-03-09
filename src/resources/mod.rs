use crate::prelude::*;

use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

mod assets;
mod mobs;

pub use self::assets::*;
pub use self::mobs::*;
