mod helpers;
mod map;
mod rect;
mod schema;
mod spawn;

pub use self::map::*;
use self::rect::Rect;
use self::schema::Schema;

use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapSpacing, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, Debug, Default)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
    DownStairs,
}

impl TileType {
    #[allow(unused)]
    pub fn opaque(self) -> bool {
        matches!(self, Self::Wall)
    }
}

pub fn setup_map(
    mut cmds: Commands,
    tiles_image: Res<TextureAssets>,
    tileset: Res<Tileset>,
    tiles: Res<Assets<Tile>>,
) {
    let (_, map) = helpers::rooms_and_corridors();
    spawn::spawn_tilemap(&mut cmds, tiles_image, tileset, tiles, &map);
}
