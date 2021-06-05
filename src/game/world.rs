use bevy::prelude::*;

pub struct World;
impl Plugin for World {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera.system())
            .add_startup_system(plane.system())
            .add_startup_system(cube.system())
            .add_startup_system(light.system());
    }
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::X, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
}

fn plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..PbrBundle::default()
    });
}

fn cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..PbrBundle::default()
    });
}

fn light(mut commands: Commands) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..LightBundle::default()
    });
}
