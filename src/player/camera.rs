use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::{CursorGrabMode, CursorOptions}};

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct CameraRotation {
    pub yaw: f32,
    pub pitch: f32,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        SpotLight {
            intensity: 100_000.0,
            range: 100000.0,
            ..default()
        },
        CameraRotation {yaw: 0.0, pitch: 0.0},
        Transform::from_xyz(10.0, 20.0, 30.0)
            .looking_at(Vec3::new(0.0,1.0,0.0), Vec3::Y),
        Player,
    ));
}

pub fn update_player(
    mut query: Query<(&mut CameraRotation, &mut Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<AccumulatedMouseMotion>,
) {
    for (mut camera_rotation, mut transform) in &mut query {
        camera_rotation.pitch += mouse_input.delta.y;
        camera_rotation.yaw += mouse_input.delta.x;

        let const_transform = *transform;
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation -= const_transform.local_z().as_vec3();
            // transform.translation -= -Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += const_transform.local_z().as_vec3();
            // transform.translation -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation -= const_transform.local_x().as_vec3();
            // transform.translation -= -Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation += const_transform.local_x().as_vec3();
            // transform.translation -= Vec3::X;
        }
        transform.rotation = transform.looking_at(Vec3::new(
            transform.translation.x,transform.translation.y,transform.translation.z-1.0
        ), Vec3::Y).rotation;

        // transform.rotation = transform.looking_at(-transform.local_z().as_vec3(), Vec3::Y).rotation;
        transform.rotate_local_axis(Dir3::new(Vec3::Y).unwrap(), -camera_rotation.yaw/1000.0);
        transform.rotate_local_axis(Dir3::new(Vec3::X).unwrap(), -camera_rotation.pitch/1000.0);
        transform.rotation = transform.rotation.normalize();
        
    }
}

pub fn grab_mouse(
  mut cursor_options: Single<&mut CursorOptions, With<Window>>,
//   mouse: Res<ButtonInput<MouseButton>>,
  key: Res<ButtonInput<KeyCode>>,
) {
  if key.just_pressed(KeyCode::KeyP) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
  }

  if key.just_pressed(KeyCode::Escape) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
  }
}