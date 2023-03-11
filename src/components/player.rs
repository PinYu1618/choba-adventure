use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub tile: TileBundle,
    pub game_unload: GameCleanup,
}

impl Player {
    pub fn spawn(cmds: &mut Commands, player_entity: Entity, emap: Entity, position: TilePos) {
        cmds.entity(player_entity).insert(PlayerBundle {
            player: Player,
            game_unload: GameCleanup,
            name: Name::new("Choba"),
            tile: TileBundle {
                position,
                texture_index: TileTextureIndex(30),
                tilemap_id: TilemapId(emap),
                color: TileColor(Color::YELLOW),
                ..default()
            },
        });
    }
}
