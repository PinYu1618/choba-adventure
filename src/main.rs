mod prelude {
    pub use bevy::prelude::*;
}

use prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(hello_system)
        .run();
}

fn hello_system(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
    //println!("Hello.");
}
