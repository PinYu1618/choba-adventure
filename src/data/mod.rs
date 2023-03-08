use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod assets;
mod mob;
mod tile;

pub use self::{
    assets::{Atlases, Fonts, Textures},
    mob::{Mob, MobData, MobType},
    tile::{Tile, TileData, TileType},
};
