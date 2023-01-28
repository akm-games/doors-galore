use bevy::{prelude::*, window::PresentMode};

const CLEAR: Color = Color::rgb(1.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1600.0,
                    height: 900.0,
                    title: "Doors Galore".to_string(),
                    present_mode: PresentMode::Fifo,
                    resizable: false,
                    ..default()
                },
                ..default()
            })
        )
        .add_startup_system(init_camera)
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
