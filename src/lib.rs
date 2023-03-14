pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bracket_geometry::prelude::{Point, Rect as BRect};
    pub use iyes_loopless::prelude::*;
    pub const CLEAR: Color = Color::GRAY;
    pub const TITLE: &str = "Escape the Os";
    pub const CANVAS: &str = "#canvas";
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const MAP_CONSOLE: usize = 0;
    pub const ENTITY_CONSOLE: usize = 1;
    pub const UI_CONSOLE: usize = 2;
    pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
    pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 32.0, y: 32.0 };
    pub const MAP_TYPE: TilemapType = TilemapType::Square;
    pub const MAP_SIZE: TilemapSize = TilemapSize { x: 80, y: 50 };
    pub use crate::{
        components::*, despawn_with, events::*, quit_app, resources::*, states::*, systems,
    };
}

use bevy::app::AppExit;
use bevy_asset_loader::prelude::*;

use self::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin);

        app.add_loopless_state(AppState::AssetsLoading)
            .add_loopless_state(DungeonState::default())
            .add_loopless_state(TurnState::Paused)
            .add_loading_state(
                LoadingState::new(AppState::AssetsLoading)
                    //.continue_to_state(AppState::MainMenu)
                    .continue_to_state(AppState::DungeonCrawl)
                    .with_collection::<Fonts>()
                    .with_collection::<Textures>()
                    .with_collection::<Atlases>(),
            )
            .add_plugin(plugins::MainMenuPlugin)
            .add_plugin(plugins::DungeonCrawlPlugin)
            .add_startup_system(setup);

        #[cfg(feature = "dev")]
        {
            app.add_plugin(plugins::DevPlugin);
        }
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

pub fn quit_app(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

pub fn to_main_menu(mut cmds: Commands) {
    cmds.insert_resource(NextState(AppState::MainMenu));
}

pub fn esc_just_pressed(kb: Res<Input<KeyCode>>) -> bool {
    kb.just_pressed(KeyCode::Escape)
}

pub fn despawn_with<T: Component>(mut cmds: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmds.entity(e).despawn_recursive();
    }
}
