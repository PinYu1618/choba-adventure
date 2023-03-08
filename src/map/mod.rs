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
    TilemapBundle,
};

use crate::prelude::*;

const MAP_TYPE: TilemapType = TilemapType::Square;

pub fn setup_map(
    mut cmds: Commands,
    tiles_image: Res<Textures>,
    tileset: Res<Tileset>,
    tiles: Res<Assets<Tile>>,
    (mobset, mobs): (Res<Mobset>, Res<Assets<Mob>>),
) {
    let (_, map) = helpers::rooms_and_corridors();
    spawn::tiles_map(&mut cmds, &tiles_image, tileset, tiles, &map);
    spawn::mobs_map(&mut cmds, &tiles_image, mobset, mobs, &map);
}
