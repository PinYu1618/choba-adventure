use super::*;

const MAP_TYPE: TilemapType = TilemapType::Square;

pub fn spawn_tilemap(
    cmds: &mut Commands,
    tiles_image: Res<TextureAssets>,
    tileset: Res<Tileset>,
    tiles: Res<Assets<Tile>>,
    schema: &Schema,
) {
    let map_size = TilemapSize { x: 80, y: 50 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = cmds.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tiletype: TileType = schema.tiles[xy_idx(x as i32, y as i32)];
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
        texture: TilemapTexture::Single(tiles_image.tiles.clone()),
        tile_size,
        spacing: TilemapSpacing { x: 1.0, y: 1.0 },
        transform: get_tilemap_center_transform(&map_size, &grid_size, &MAP_TYPE, 0.0),
        ..default()
    });
}
