use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub sprite: SpriteSheetBundle,
    pub tilepos: TilePos,
    //pub game_unload: GameUnload,
}

pub fn spawn_player(mut cmds: Commands, atlas: Res<Atlases>) {
    cmds.spawn(PlayerBundle {
        player: Player,
        //game_unload: GameUnload,
        name: Name::new("Choba"),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 30,
                color: Color::YELLOW,
                ..default()
            },
            texture_atlas: atlas.onebit_mono.clone(),
            ..default()
        },
        tilepos: TilePos { x: 40, y: 0 },
    });
}
