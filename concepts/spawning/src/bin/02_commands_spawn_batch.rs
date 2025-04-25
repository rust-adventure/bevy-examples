use bevy::{color::palettes::tailwind::*, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_batch([
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(
                materials.add(Color::from(SKY_400)),
            ),
            Transform::from_xyz(-2.0, 0.0, 0.0),
        ),
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(
                materials.add(Color::from(SKY_400)),
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(
                materials.add(Color::from(SKY_400)),
            ),
            Transform::from_xyz(2.0, 0.0, 0.0),
        ),
    ]);

    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.0, 2.0, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
