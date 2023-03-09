use crate::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            .add_system(print_state_on_change)
            .add_system(clear_on_del.run_if(in_game))
            .add_system(quit_app.run_if(lshift_esc_pressed))
            .add_system(
                to_ingame
                    .run_in_state(AppState::MainMenu)
                    .run_if(lshift_enter_pressed),
            )
            .add_exit_system(AppState::InGame, debug_leave_game);
    }
}

fn print_state_on_change(appstate: Res<CurrentState<AppState>>) {
    if appstate.is_changed() {
        info!("{:?}", appstate);
    }
}

fn clear_on_del(mut cmds: Commands, kb: Res<Input<KeyCode>>) {
    if kb.just_pressed(KeyCode::Delete) {
        cmds.insert_resource(NextState(AppState::InGame));
        info!("Game is reseted.");
    }
}

fn lshift_esc_pressed(kb: Res<Input<KeyCode>>) -> bool {
    kb.pressed(KeyCode::Escape) && kb.pressed(KeyCode::LShift)
}

fn lshift_enter_pressed(kb: Res<Input<KeyCode>>) -> bool {
    kb.pressed(KeyCode::Return) && kb.pressed(KeyCode::LShift)
}

fn in_game(state: Res<CurrentState<AppState>>) -> bool {
    *state == CurrentState(AppState::InGame)
}

fn debug_leave_game() {
    info!("Unloading game...");
}
