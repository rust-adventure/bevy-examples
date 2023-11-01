use bevy::prelude::*;
use bevy_shader_utils::{
    PristineGridMaterial, ShaderUtilsPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShaderUtilsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PristineGridMaterial>>,
) {
    // floor
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 40.,
            ..default()
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(PristineGridMaterial {
            color: Color::DARK_GRAY,
            cell_multiplier: Vec2::splat(80.),
            ..default()
        }),
        ..default()
    });

    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        transform: Transform::from_xyz(0.0, 0.8, 0.0),
        material: materials.add(PristineGridMaterial {
            color: Color::CYAN,
            cell_multiplier: Vec2::splat(20.),
            ..default()
        }),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 3.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

fn rotate_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let cam_transform = camera.single_mut().into_inner();

    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(
            Vec3::Y,
            45f32.to_radians() * time.delta_seconds(),
        ),
    );
    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
}
