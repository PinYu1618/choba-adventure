use bevy::reflect::TypeUuid;

use super::*;

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

#[derive(serde::Deserialize, TypeUuid, Reflect)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default, AssetCollection)]
pub struct Tileset {
    #[asset(path = "data/tiles/floor.tile.ron")]
    pub floor: Handle<Tile>,
    #[asset(path = "data/tiles/wall.tile.ron")]
    pub wall: Handle<Tile>,
    #[asset(path = "data/tiles/downstairs.tile.ron")]
    pub downstairs: Handle<Tile>,
}

impl Tileset {
    pub fn select(&self, tiletype: &TileType) -> Handle<Tile> {
        use TileType::*;

        match *tiletype {
            Floor => self.floor.clone(),
            Wall => self.wall.clone(),
            DownStairs => self.downstairs.clone(),
        }
    }
}
