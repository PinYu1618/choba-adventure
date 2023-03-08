use bevy::reflect::TypeUuid;

use super::*;
use crate::prelude::*;

#[derive(serde::Deserialize, TypeUuid, Reflect)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default, AssetCollection)]
pub struct TileAssets {
    #[asset(path = "data/tiles/floor.tile.ron")]
    pub floor: Handle<Tile>,
    #[asset(path = "data/tiles/wall.tile.ron")]
    pub wall: Handle<Tile>,
    #[asset(path = "data/tiles/downstairs.tile.ron")]
    pub downstairs: Handle<Tile>,
}

impl TileAssets {
    pub fn select(&self, tiletype: &TileType) -> Handle<Tile> {
        use TileType::*;

        match *tiletype {
            Floor => self.floor.clone(),
            Wall => self.wall.clone(),
            DownStairs => self.downstairs.clone(),
        }
    }
}
