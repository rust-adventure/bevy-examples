use bevy::{color::palettes::tailwind::*, prelude::*};
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
    commands.spawn((
        Mesh3d(
            meshes.add(Mesh::from(
                Plane3d::default()
                    .mesh()
                    .size(40., 40.)
                    .subdivisions(10),
            )),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(
            PristineGridMaterial {
                color: GREEN_400.into(),
                cell_multiplier: Vec2::splat(80.),
                ..default()
            },
        )),
    ));

    // sphere
    commands.spawn((
        Mesh3d(
            meshes.add(Sphere::default().mesh().uv(32, 18)),
        ),
        Transform::from_xyz(0.0, 0.8, 0.0),
        MeshMaterial3d(materials.add(
            PristineGridMaterial {
                color: SKY_400.into(),
                cell_multiplier: Vec2::splat(20.),
                ..default()
            },
        )),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
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
