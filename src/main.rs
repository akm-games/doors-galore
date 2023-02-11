use bevy::{prelude::*, window::PresentMode, sprite::collide_aabb::Collision};

const CLEAR: Color = Color::rgb(0.5, 0.5, 0.5);

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
            .set(ImagePlugin::default_nearest())
        )
        .add_startup_system(init_camera)
        .add_startup_system(create_center_sprite)
        .add_startup_system(create_room)
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        },
        ..default()
    });
}

fn create_center_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/mc.png"),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
}

fn create_room(mut commands: Commands) {
    
}
