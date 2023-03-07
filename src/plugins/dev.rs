use crate::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            .add_system(print_state_on_change);
    }
}

fn print_state_on_change(appstate: Res<CurrentState<AppState>>) {
    if appstate.is_changed() {
        info!("{:?}", appstate);
    }
}
