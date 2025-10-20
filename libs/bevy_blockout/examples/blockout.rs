use bevy::{
    camera::Exposure,
    color::palettes::tailwind::*,
    core_pipeline::tonemapping::Tonemapping,
    light::light_consts::lux,
    pbr::{
        Atmosphere, AtmosphereSettings, ExtendedMaterial,
    },
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};
use bevy_blockout::{BlockoutMaterialExt, BlockoutPlugin};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 4000.,
            ..default()
        })
        .add_plugins((DefaultPlugins, BlockoutPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        // .add_systems(Update, dynamic_scene)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                BlockoutMaterialExt,
            >,
        >,
    >,
    asset_server: Res<AssetServer>,
) {
    // floor
    commands.spawn((
        Mesh3d(
            meshes.add(Mesh::from(
                Plane3d::default()
                    .mesh()
                    .size(300., 300.)
                    .subdivisions(10),
            )),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: BLUE_50.into(),
                reflectance: 0.,
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),
    ));

    // sphere
    commands.spawn((
        Mesh3d(
            meshes.add(Sphere::new(6.).mesh().uv(32, 18)),
        ),
        Transform::from_xyz(0.0, 14., 0.0),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: SKY_400.into(),
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(
            10., 4., 20.,
        )))),
        Transform::from_xyz(10.0, 2., 0.0),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: SKY_400.into(),
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),
    ));

    // commands.spawn((
    //     Mesh3d(meshes.
    // add(Cuboid::from_size(Vec3::new(
    //         10., 4., 20.,
    //     )))),
    //     Transform::from_xyz(10.0, 1., 0.0),
    //     MeshMaterial3d(materials.
    // add(ExtendedMaterial {         base:
    // StandardMaterial {             base_color:
    // SKY_400.into(),             ..default()
    //         },
    //         extension:
    // BlockoutMaterialExt::default(),     })),
    // ));
    // commands.spawn((
    //     Mesh3d(meshes.
    // add(Cuboid::from_size(Vec3::new(
    //         10., 4., 20.,
    //     )))),
    //     Transform::from_xyz(10.0, 1., 0.0),
    //     MeshMaterial3d(materials.
    // add(ExtendedMaterial {         base:
    // StandardMaterial {             base_color:
    // SKY_400.into(),             ..default()
    //         },
    //         extension:
    // BlockoutMaterialExt::default(),     })),
    // ));
    // camera
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        // HDR is required for atmospheric scattering to be properly applied to the scene
        Camera {
            ..default()
        },
        Hdr,
        Transform::from_xyz(-50.2, 20., 50.0)
            .looking_at(Vec3::Y * 0.1, Vec3::Y),
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings {
            // aerial_view_lut_max_distance: 3.2e5,
            // scene_units_to_m: 1e+1,
            ..default()
        },
        // The directional light illuminance  used in this scene
        // (the one recommended for use with this feature) is
        // quite bright, so raising the exposure compensation helps
        // bring the scene to a nicer brightness range.
        Exposure::SUNLIGHT,
        // Tonemapper chosen just because it looked good with the scene, any
        // tonemapper would be fine :)
        Tonemapping::AcesFitted,
        // Bloom gives the sun a much more natural look.
        Bloom::NATURAL,
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 20000.0,
            ..default()
        }
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            // lux::RAW_SUNLIGHT is recommended for use with
            // this feature, since other values
            // approximate sunlight *post-scattering* in
            // various conditions. RAW_SUNLIGHT
            // in comparison is the illuminance of the
            // sun unfiltered by the atmosphere, so it is
            // the proper input for sunlight to
            // be filtered by the atmosphere.
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, -0.4, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_rotation(Quat::from_array([
                0.688806,
                0.10741661,
                0.6988995,
                -0.15983216,
            ])),
    ));
}

fn rotate_camera(
    mut cam_transform: Single<
        &mut Transform,
        With<MainCamera>,
    >,
    time: Res<Time>,
) {
    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(
            Vec3::Y,
            45f32.to_radians() * time.delta_secs(),
        ),
    );
    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
}

// fn dynamic_scene(
//     mut suns: Query<&mut Transform,
// With<DirectionalLight>>,     time: Res<Time>,
// ) {
//     suns.iter_mut().for_each(|mut tf| {
//         tf.rotate_x(-time.delta_secs() * PI /
// 10.0);         dbg!(tf);
//     });
// }
