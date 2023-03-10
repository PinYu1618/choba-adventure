use bevy::reflect::TypeUuid;

use super::*;

#[derive(serde::Deserialize, TypeUuid, Reflect)]
#[uuid = "b746ca1f-db74-430b-9176-615a31d0933a"]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
}
