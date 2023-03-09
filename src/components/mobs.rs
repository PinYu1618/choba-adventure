use super::*;

#[derive(Component)]
pub struct Mob;

#[derive(Bundle)]
pub struct MobBundle {
    pub name: Name,
    pub tile: TilemapTileBundle,
}
