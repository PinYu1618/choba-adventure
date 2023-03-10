use crate::prelude::*;

#[derive(Resource)]
pub struct Schema {
    pub tiles: Vec<TileType>,
    pub size: IVec2,
    pub mob_spawns: Vec<(i32, i32)>,
}

impl Schema {
    pub fn width(&self) -> i32 {
        self.size.x
    }

    pub fn height(&self) -> i32 {
        self.size.y
    }
}
