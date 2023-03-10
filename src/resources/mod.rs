use crate::prelude::*;

use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

mod assets;
mod map;
mod mobs;

pub use self::assets::*;
pub use self::map::*;
pub use self::mobs::*;
