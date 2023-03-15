use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(1))
        .run();
}

fn setup(mut cmds: Commands, ass: Res<AssetServer>) {
    cmds.spawn(Camera2dBundle::default());
    cmds.spawn(LdtkWorldBundle {
        ldtk_handle: ass.load("ldtks/World.01.ldtk"),
        ..default()
    });
}
