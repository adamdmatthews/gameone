use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::f32::consts::PI;

pub struct Player;
impl Plugin for Player {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_system(camera.system())
            .add_system(movement.system());
    }
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 1.0, 3.0).looking_at(Vec3::Y, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
}

#[allow(clippy::needless_pass_by_value)] //systems need to be passed by value
fn movement(keyboard_input: Res<Input<KeyCode>>, mut transforms: Query<(&mut Transform, &Camera)>) {
    for (mut transform, _camera) in transforms.iter_mut() {
        let mut translation = Vec3::ZERO;
        let mut rotation = Quat::IDENTITY;
        for key in keyboard_input.get_just_pressed() {
            let (t, r) = key_movement(*key, &transform);
            translation += t;
            rotation = rotation.lerp(r, 1.0);
        }
        transform.translation += translation;
        transform.rotate(rotation);
    }
}

fn key_movement(key: KeyCode, transform: &Transform) -> (Vec3, Quat) {
    match key {
        KeyCode::W => (-transform.local_z(), Quat::IDENTITY),
        KeyCode::A => (-transform.local_x(), Quat::IDENTITY),
        KeyCode::S => (transform.local_z(), Quat::IDENTITY),
        KeyCode::D => (transform.local_x(), Quat::IDENTITY),
        KeyCode::Up => (Vec3::ZERO, Quat::from_rotation_x(PI / 16.0)),
        KeyCode::Left => (
            Vec3::ZERO,
            Quat::from_axis_angle(transform.local_y(), PI / 16.0),
        ),
        KeyCode::Down => (Vec3::ZERO, Quat::from_rotation_x(-PI / 16.0)),
        KeyCode::Right => (
            Vec3::ZERO,
            Quat::from_axis_angle(-transform.local_y(), PI / 16.0),
        ),
        _ => (Vec3::ZERO, Quat::IDENTITY),
    }
}
