mod map;
mod mapgen;

pub use self::map::*;
use self::mapgen::rooms_and_corridors;

use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};

use crate::prelude::*;

const MAP_TYPE: TilemapType = TilemapType::Square;

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
    tiles_image: Res<TilesImage>,
    tileset: Res<Tileset>,
    tiles: Res<Assets<Tile>>,
) {
    let (_, map) = rooms_and_corridors();
    create_tilemap(cmds, tiles_image, tileset, tiles, &map);
}

pub fn create_tilemap(
    mut cmds: Commands,
    tiles_image: Res<TilesImage>,
    tileset: Res<Tileset>,
    tiles: Res<Assets<Tile>>,
    map: &Map,
) {
    let map_size = TilemapSize { x: 80, y: 50 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = cmds.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tiletype: TileType = map.0[xy_idx(x as i32, y as i32)];
            let tile_handle = tileset.select(&tiletype);
            let tile = tiles.get(&tile_handle).unwrap();
            let tile_entity = cmds
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile.index),
                    color: TileColor(tile.fg),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    cmds.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        map_type: MAP_TYPE,
        texture: TilemapTexture::Single(tiles_image.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &MAP_TYPE, 0.0),
        ..default()
    });
}
