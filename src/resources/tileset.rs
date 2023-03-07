use bevy::reflect::TypeUuid;

use crate::prelude::*;

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct TileInfo {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default)]
pub struct Tileset {
    floor: Handle<TileInfo>,
    wall: Handle<TileInfo>,
    downstairs: Handle<TileInfo>,
}

impl Tileset {
    #[allow(unused)]
    pub fn select(&self, tiletype: &Tile) -> Handle<TileInfo> {
        use Tile::*;

        match *tiletype {
            Floor => self.floor.clone(),
            Wall => self.wall.clone(),
            DownStairs => self.downstairs.clone(),
        }
    }
}

pub fn load_tileset(mut cmds: Commands, ass: Res<AssetServer>) {
    let tileset = Tileset {
        floor: ass.load("data/tiles/floor.tile.ron"),
        wall: ass.load("data/tiles/wall.tile.ron"),
        downstairs: ass.load("data/tiles/downstairs.tile.ron"),
    };
    cmds.insert_resource(tileset);
}
