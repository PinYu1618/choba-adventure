pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::{
        prelude::{TileColor, TilePos, TileStorage, TileTextureIndex},
        tiles::TileBundle as TilemapTileBundle,
    };
    pub use bracket_geometry::prelude::{Point, Rect as BRect};
    pub use iyes_loopless::prelude::*;
    pub const CLEAR: Color = Color::BLACK;
    pub const TITLE: &str = "Choba Adventure";
    pub const CANVAS: &str = "#canvas";
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const MAP_CONSOLE: usize = 0;
    pub const ENTITY_CONSOLE: usize = 1;
    pub const UI_CONSOLE: usize = 2;
    pub use crate::{cleanup_on, components::*, events::*, quit_app, resources::*, states::*};
}

use bevy::app::AppExit;
use bevy_asset_loader::prelude::*;

use self::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppState::AssetsLoading)
            .add_loopless_state(TurnState::Paused)
            .add_loading_state(
                LoadingState::new(AppState::AssetsLoading)
                    //.continue_to_state(AppState::MainMenu)
                    .continue_to_state(AppState::InGame)
                    .with_collection::<Fonts>()
                    .with_collection::<Textures>()
                    .with_collection::<Atlases>(),
            )
            .add_plugin(plugins::ConsolePlugin)
            .add_plugin(plugins::MainMenuPlugin)
            .add_plugin(plugins::SpawnPlugin)
            .add_startup_system(setup)
            //.add_enter_system(AppState::InGame, map::setup_map)
            .add_exit_system(AppState::InGame, cleanup_on::<GameUnload>)
            .add_system(
                to_main_menu
                    .run_if(esc_just_pressed)
                    .run_in_state(AppState::InGame),
            )
            .add_system(
                quit_app
                    .run_if(esc_just_pressed)
                    .run_in_state(AppState::MainMenu),
            );

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

pub fn to_ingame(mut cmds: Commands) {
    cmds.insert_resource(NextState(AppState::InGame));
}

pub fn esc_just_pressed(kb: Res<Input<KeyCode>>) -> bool {
    kb.just_pressed(KeyCode::Escape)
}

pub fn cleanup_on<T: Component>(mut cmds: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmds.entity(e).despawn_recursive();
    }
}
