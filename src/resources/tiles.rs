use bevy::reflect::{self, TypeUuid};

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Floor,
    //Wall,
    //DownStairs,
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TileHandle(Handle<Tile>);

pub fn load_tiles(mut cmds: Commands, ass: Res<AssetServer>) {
    let floor = TileHandle(ass.load("data/tiles/floor.tile.ron"));
    cmds.insert_resource(floor);
}
