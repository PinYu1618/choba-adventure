use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod atlas;
mod font;
mod texture;
mod tile;

pub use self::{
    atlas::AtlasAssets,
    font::FontAssets,
    texture::TextureAssets,
    tile::{Tile, TileAssets},
};
