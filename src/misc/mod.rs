use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct GameUnload;

pub fn cleanup_on<T: Component>(mut cmds: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmds.entity(e).despawn_recursive();
    }
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
