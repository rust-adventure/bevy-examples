use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<PristineMaterial>::default(),
            ShaderUtilsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PristineMaterial>>,
) {
    // floor
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 40.,
            subdivisions: 100,
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(PristineMaterial {
            color: Color::DARK_GRAY,
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
        material: materials
            .add(PristineMaterial { color: Color::CYAN }),
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

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for PristineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pristine_material.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PristineMaterial {
    #[uniform(0)]
    color: Color,
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
