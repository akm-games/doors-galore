use bevy::{
    prelude::*,
    input::{mouse::{MouseWheel, MouseScrollUnit, MouseMotion, MouseButtonInput}, ButtonState}
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system(handle_mouse_scroll)
            .add_system(handle_mouse_middle_click);
    }
}

#[derive(Component)]
struct IsMoving(bool);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            ..default()
        },
        ..default()
    })
        .insert(IsMoving(false))
        .insert(Name::new("Main Camera"));
}

fn handle_mouse_scroll(
    mut scroll_evr: EventReader<MouseWheel>,
    mut ortho_proj_qry: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    let zoom_speed = 10.;
    for ev in scroll_evr.iter() {
        let mut ortho_proj = ortho_proj_qry.single_mut();
        let scroll_dir = ev.y;
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut scale = ortho_proj.scale - (scroll_dir * zoom_speed * time.delta_seconds());
                if scale < 0.01 { scale = 0.01 };
                ortho_proj.scale = scale;
            }
            MouseScrollUnit::Pixel => ()
        }
    }
}

fn handle_mouse_middle_click(
    mut button_evr: EventReader<MouseButtonInput>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_qry: Query<(&mut Transform, &mut IsMoving), With<Camera>>,
) {
    let (mut transform, mut is_moving) = camera_qry.single_mut();
    
    for ev in button_evr.iter() {
        if ev.button == MouseButton::Middle {
            if ev.state == ButtonState::Pressed {
                is_moving.0 = true;
            }
            if ev.state == ButtonState::Released {
                is_moving.0 = false;
            }
        }
    }
    
    if is_moving.0 {
        for ev in motion_evr.iter() {
            let delta = ev.delta;
            let mut pos = transform.translation;
            pos = Vec3::new(pos.x - delta.x, pos.y + delta.y, pos.z);
            transform.translation = pos;
        }
    }
    



    /*
    if input.just_pressed(MouseButton::Middle) {
        let mut transform = transform_qry.single_mut();
        transform.translation = Vec3::new(100., 0., 999.);
    }
    if input.just_released(MouseButton::Middle) {
        let mut transform = transform_qry.single_mut();
    }
    */
}