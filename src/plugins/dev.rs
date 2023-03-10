use crate::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            .add_system(print_state_on_change)
            .add_system(clear_on_del.run_if(in_game))
            .add_system(quit_app.run_if(lshift_esc_pressed))
            .add_exit_system(AppState::DungeonCrawl, debug_leave_game);
    }
}

fn print_state_on_change(
    appstate: Res<CurrentState<AppState>>,
    dungeon_state: Res<CurrentState<DungeonState>>,
) {
    if appstate.is_changed() {
        info!("{:?}", appstate);
    }
    if dungeon_state.is_changed() {
        info!("{:?}", dungeon_state);
    }
}

fn clear_on_del(mut cmds: Commands, kb: Res<Input<KeyCode>>) {
    if kb.just_pressed(KeyCode::Delete) {
        cmds.insert_resource(NextState(AppState::DungeonCrawl));
        info!("Game is reseted.");
    }
}

fn lshift_esc_pressed(kb: Res<Input<KeyCode>>) -> bool {
    kb.pressed(KeyCode::Escape) && kb.pressed(KeyCode::LShift)
}

fn in_game(state: Res<CurrentState<AppState>>) -> bool {
    *state == CurrentState(AppState::DungeonCrawl)
}

fn debug_leave_game() {
    info!("Unloading game...");
}
