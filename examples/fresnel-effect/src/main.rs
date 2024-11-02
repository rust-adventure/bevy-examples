use bevy::{
    color::palettes::tailwind::*,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
// use bevy_panorbit_camera::{
//     PanOrbitCamera, PanOrbitCameraPlugin,
// };

fn main() {
    App::new()
        .insert_resource(ClearColor(
            SLATE_950.into()
        ))
        .add_plugins((
            DefaultPlugins,
            // PanOrbitCameraPlugin,
        MaterialPlugin::<
            NormalVisualizerMaterial,
        >::default()
    ))
        .add_systems(Startup, startup)
        .add_systems(Update, (animate_light_direction, swap_components))
        .run();
}

fn startup(
    mut commands: Commands,
    mut custom_materials: ResMut<
        Assets<NormalVisualizerMaterial>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let normals = [
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
    ];

    let mesh = meshes
        .add(Sphere { radius: 1.4 }.mesh().uv(40, 40));
    for (i, selection) in normals.iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh.clone()),
            Transform::from_xyz(
                4.0 * i as f32 - 4.0,
                2.0,
                0.0,
            ),
            MeshMaterial3d(custom_materials.add(
                NormalVisualizerMaterial {
                    selection: *selection,
                    show_components: 0.,
                },
            )),
        ));

        let mut second_selection = *selection;
        second_selection.w = 1.0;
        commands.spawn((
            Mesh3d(mesh.clone()),
            Transform::from_xyz(
                4.0 * i as f32 - 4.0,
                -2.0,
                0.0,
            ),
            MeshMaterial3d(custom_materials.add(
                NormalVisualizerMaterial {
                    selection: second_selection,
                    show_components: 0.,
                },
            )),
        ));
    }

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        // PanOrbitCamera::default(),
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight::default(),
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_4,
            ),
            ..default()
        },
    ));
}

impl Material for NormalVisualizerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/normal_visualizer.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NormalVisualizerMaterial {
    #[uniform(0)]
    selection: Vec4,
    #[uniform(0)]
    show_components: f32,
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        With<DirectionalLight>,
    >,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn swap_components(
    mut materials: ResMut<Assets<NormalVisualizerMaterial>>,
    time: Res<Time>,
) {
    if time.elapsed_secs().sin() > 0. {
        for (_, mat) in materials.iter_mut() {
            mat.show_components = 0.;
        }
    } else {
        for (_, mat) in materials.iter_mut() {
            mat.show_components = 1.;
        }
    }
}

pub struct SpawnSphere {
    pub transform: Transform,
    pub material: Handle<NormalVisualizerMaterial>,
}
