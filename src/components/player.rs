use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub sprite: SpriteSheetBundle,
    pub tilepos: TilePos,
    pub game_unload: GameUnload,
}

impl Player {
    pub fn spawn(mut cmds: Commands, atlas: Res<Atlases>, q: Query<Entity, Added<Player>>) {
        if let Ok(e) = q.get_single() {
            cmds.entity(e).insert(PlayerBundle {
                game_unload: GameUnload,
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
    }
}
