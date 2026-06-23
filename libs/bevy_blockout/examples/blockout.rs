use std::f32::consts::FRAC_PI_6;

use bevy::{
    camera::{Exposure, Hdr},
    color::palettes::tailwind::*,
    core_pipeline::tonemapping::Tonemapping,
    light::{
        Atmosphere, AtmosphereEnvironmentMapLight,
        atmosphere::ScatteringMedium, light_consts::lux,
    },
    pbr::{AtmosphereSettings, ExtendedMaterial},
    post_process::bloom::Bloom,
    prelude::*,
};
use bevy_blockout::{BlockoutMaterialExt, BlockoutPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BlockoutPlugin))
        .add_systems(Startup, setup.spawn())
        .add_systems(Update, rotate_camera)
        .run();
}

#[derive(Default, Clone, Component)]
struct MainCamera;

fn setup() -> impl SceneList {
    bsn_list![
        #Floor
        Mesh3d(
            asset_value(Mesh::from(
                Plane3d::default()
                    .mesh()
                    .size(50., 50.)
                    .subdivisions(10),
            )),
        )
        Transform::from_xyz(0.0, 0.0, 0.0)
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: BLUE_50.into(),
                perceptual_roughness: 1.,
                reflectance: 0.,
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #Sphere
        Mesh3d(
            asset_value(Sphere::new(2.).mesh().uv(32, 18)),
        )
        Transform::from_xyz(0.0, 4., 0.0)
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: SKY_400.into(),
                perceptual_roughness: 1.,
                reflectance: 0.,
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #BoxA
        Mesh3d(
            asset_value(Cuboid::from_size(Vec3::new(
                2., 1., 4.,
            ))),
        )
        Transform::from_xyz(4.0, 0.5, 0.0)
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: LIME_400.into(),
                perceptual_roughness: 1.,
                reflectance: 0.,
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #BoxB
        Mesh3d(asset_value(Extrusion::new(
            RegularPolygon::new(2., 6),
            4.,
        )))
        template_value(Transform::from_xyz(-1.0, 0., 0.0).with_rotation(
            Quat::from_axis_angle(Vec3::Z, FRAC_PI_6),
        ))
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: RED_400.into(),
                perceptual_roughness: 1.,
                reflectance: 0.,
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #BoxC
        Mesh3d(asset_value(Cuboid::from_size(Vec3::new(
            10., 3., 14.,
        ))))
        Transform::from_xyz(-15.0, 1., 0.0)
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: SKY_400.into(),
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #BoxD
        Mesh3d(
            asset_value(Cuboid::from_size(Vec3::new(
                5., 4., 7.,
            ))),
        )
        Transform::from_xyz(-12.5, 2., -10.5)
        MeshMaterial3d::<ExtendedMaterial<StandardMaterial, BlockoutMaterialExt>>(asset_value(ExtendedMaterial {
            base: StandardMaterial {
                base_color: SKY_400.into(),
                ..default()
            },
            extension: BlockoutMaterialExt::default(),
        })),

        #Atmosphere
        template(|context| {
            let mut scattering_mediums = context.resource_mut::<
                Assets<ScatteringMedium>,
            >();
            Ok(Atmosphere::earth(
                scattering_mediums.add(ScatteringMedium::earth(256, 256)),
            ))
        }),

        #MainCamera
        MainCamera
        Camera3d::default()
        Camera::default()
        Hdr
        template_value(Transform::from_xyz(-10.2, 5., 10.0)
            .looking_at(Vec3::Y * 0.1, Vec3::Y))
        AtmosphereSettings::default()
        Exposure::SUNLIGHT
        template_value(Tonemapping::AcesFitted)
        Bloom::NATURAL
        AtmosphereEnvironmentMapLight::default(),

        #GlobalDirectionLight
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
        }
        template_value(
            Transform::from_xyz(1.0, -0.4, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .with_rotation(Quat::from_array([
                    0.688806,
                    0.10741661,
                    0.6988995,
                    -0.15983216,
                ]))
        )
    ]
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
