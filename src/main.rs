use etos::{prelude::*, GamePlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: TITLE.to_string(),
                        canvas: Some(CANVAS.to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(GamePlugin)
        .run();
}
