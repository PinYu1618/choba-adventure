mod data;
mod map;
mod player;
pub mod plugins;
mod states;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::{
        prelude::{TileColor, TilePos, TileStorage, TileTextureIndex},
        tiles::TileBundle as TilemapTileBundle,
    };
    pub use iyes_loopless::prelude::*;

    pub use crate::{data::*, plugins, states::*};
}

use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use prelude::*;

const CLEAR: Color = Color::BLACK;
const TITLE: &str = "Choba Adventure";

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: TITLE.to_string(),
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .register_type::<Tile>()
        .add_plugin(RonAssetPlugin::<Tile>::new(&["tile.ron"]))
        .add_plugin(RonAssetPlugin::<Mob>::new(&["mob.ron"]))
        .add_loopless_state(AppState::AssetsLoading)
        .add_loading_state(
            LoadingState::new(AppState::AssetsLoading)
                .continue_to_state(AppState::MainMenu)
                .with_collection::<Fonts>()
                .with_collection::<Textures>()
                .with_collection::<Atlases>()
                .with_collection::<Tileset>()
                .with_collection::<Mobset>(),
        )
        .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
        .add_plugins(bevy_ui_navigation::DefaultNavigationPlugins)
        .add_startup_system(setup_camera)
        .add_enter_system(AppState::MainMenu, transition_to_ingame)
        .add_enter_system(AppState::InGame, map::setup_map)
        .add_enter_system(
            AppState::InGame,
            player::spawn_player, // ^TODO: use `run_if_resource_added`
        );

    #[cfg(feature = "dev")]
    {
        app.add_plugin(plugins::DevPlugin);
    }

    app.run();
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

// ^TODO: build up menu scene instead
fn transition_to_ingame(mut cmds: Commands) {
    cmds.insert_resource(NextState(AppState::InGame));
}
