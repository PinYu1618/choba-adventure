pub mod plugins;
mod resources;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::{plugins, resources::*};
}

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
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RonAssetPlugin::<TileInfo>::new(&["tile.ron"]))
        .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
        .add_plugins(bevy_ui_navigation::DefaultNavigationPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(assets::load_fonts)
                .with_system(assets::load_textures)
                .with_system(tiles::load_tileset),
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
