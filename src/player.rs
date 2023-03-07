use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub sprite: SpriteSheetBundle,
    pub tilepos: TilePos,
}

pub fn spawn_player(mut cmds: Commands, atlas: Res<TilesAtlas>) {
    cmds.spawn(PlayerBundle {
        player: Player,
        name: Name::new("Choba"),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 30,
                color: Color::YELLOW,
                ..default()
            },
            texture_atlas: atlas.clone(),
            ..default()
        },
        tilepos: TilePos { x: 40, y: 0 },
    });
}
