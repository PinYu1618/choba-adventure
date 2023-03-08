//! Almost the same as `TilemapBundle`, copied my own one to make implementation of map algos easier

use bevy_ecs_tilemap::{
    prelude::{
        TilemapGridSize, TilemapSize, TilemapSpacing, TilemapTexture, TilemapTileSize, TilemapType,
    },
    FrustumCulling,
};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DistanceAlg, Point, SmallVec};

use crate::prelude::*;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

#[derive(Bundle, Debug, Default, Clone)]
pub struct MapBundle {
    pub grid_size: TilemapGridSize,
    pub map_type: TilemapType,
    pub size: TilemapSize,
    pub spacing: TilemapSpacing,
    pub storage: TileStorage,
    pub texture: TilemapTexture,
    pub tile_size: TilemapTileSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted
    /// for rendering
    pub computed_visibility: ComputedVisibility,
    /// User indication of whether tilemap should be frustum culled.
    pub frustum_culling: FrustumCulling,
}

impl MapBundle {
    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    #[allow(unused)]
    pub fn get(&self, tile_pos: &TilePos) -> Option<Entity> {
        self.storage.checked_get(tile_pos)
    }

    pub fn xy(&self, idx: usize) -> (i32, i32) {
        (
            idx as i32 % self.width() as i32,
            idx as i32 / self.width() as i32,
        )
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width() as i32 - 1 || y < 1 || y > self.height() as i32 - 1 {
            return false;
        }
        //let idx = self.xy_idx(x, y);
        //self.blocked[idx]

        false
    }
}

impl BaseMap for MapBundle {
    fn is_opaque(&self, idx: usize) -> bool {
        let (_x, _y) = self.xy(idx);
        //self.tiles[idx] == TileType::Wall
        //todo!()
        true
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width() as i32;
        let y = idx as i32 / self.width() as i32;
        let w = self.width() as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width() as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for MapBundle {
    fn dimensions(&self) -> Point {
        Point::new(self.width(), self.height())
    }
}
