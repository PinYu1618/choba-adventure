use bevy::reflect::TypeUuid;

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    DownStairs,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
}

// ^TODO: implement Index for TileType
#[derive(Resource, Reflect, Default)]
pub struct Tileset {
    floor: Handle<Tile>,
}

pub fn load_tileset(mut cmds: Commands, ass: Res<AssetServer>) {
    let tileset = Tileset {
        floor: ass.load("data/tiles/floor.tile.ron"),
    };
    cmds.insert_resource(tileset);
}
