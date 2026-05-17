use bevy::{
    asset::HandleTemplate,
    camera_controller::free_camera::{
        FreeCamera, FreeCameraPlugin,
    },
    core_pipeline::prepass::{
        DepthPrepass, MotionVectorPrepass, NormalPrepass,
    },
    math::Affine2,
    pbr::{ExtendedMaterial, MeshMaterial3dTemplate},
    prelude::*,
};

use bevy_shader_utils::ShaderUtilsPlugin;
use dissolve_sphere_standard_material_extension::DissolveExtension;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("1fa9f4").unwrap().into(),
        ))
        .add_plugins((
            DefaultPlugins,
            ShaderUtilsPlugin,
            MaterialPlugin::<
                ExtendedMaterial<
                    StandardMaterial,
                    DissolveExtension,
                >,
            >::default(),
            FreeCameraPlugin,
        ))
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, animate_light_direction)
        .run();
}

fn scene() -> impl SceneList {
    bsn_list![
        (
            Camera3d::default()
            template_value(Transform::from_xyz(-1.0, 1.25, 2.5)
                .looking_at(Vec3::new(0.,0.5,0.), Vec3::Y))
            FreeCamera
            DepthPrepass
            NormalPrepass
            MotionVectorPrepass
        ),
        (
            DirectionalLight {
                shadow_maps_enabled: true,
            }
            Transform {
                translation: Vec3::new(10.0, 20.0, 10.0),
                rotation: Quat::from_rotation_x(
                    -std::f32::consts::FRAC_PI_4,
                ),
            }
        ),
        (
        Mesh3d(asset_value(Sphere::default().mesh().uv(32, 18)))
        Transform::from_xyz(0.0, 0.5, 0.0)
        template(|ctx| {
            let asset_server = ctx.resource::<AssetServer>();
            let mat = ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("concrete/sekjcawb_2K_Albedo.jpg")),
                    normal_map_texture: Some(asset_server.load("concrete/sekjcawb_2K_Normal.jpg")),
                    metallic_roughness_texture: Some(asset_server.load("concrete/sekjcawb_2K_Roughness.jpg")),
                    double_sided: true,
                    cull_mode: None,
                    alpha_mode: AlphaMode::Mask(0.5),
                    ..default()
                },
                extension: DissolveExtension {},
            };
            let mat = ctx.resource_mut::<Assets<ExtendedMaterial<StandardMaterial, DissolveExtension>>>().add(mat);
            Ok(MeshMaterial3d(mat))
        })
        ),
        (
            Mesh3d(asset_value(
                Plane3d::default().mesh().size(10., 10.),
            ))
            template(|ctx| {
                let asset_server = ctx.resource::<AssetServer>();
                let mat = StandardMaterial {
                    base_color_texture: Some(asset_server.load("concrete/sekjcawb_2K_Albedo.jpg")),
                    normal_map_texture: Some(asset_server.load("concrete/sekjcawb_2K_Normal.jpg")),
                    metallic_roughness_texture: Some(asset_server.load("concrete/sekjcawb_2K_Roughness.jpg")),
                    uv_transform: Affine2::from_scale(Vec2::splat(2.5)),
                    ..default()
                };
                let mat = ctx.resource_mut::<Assets<StandardMaterial>>().add(mat);
                Ok(MeshMaterial3d(mat))
            })
        )
    ]
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

// #[derive(Component, Default, Clone)]
// struct Movable;

// fn movement(
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<Movable>>,
// ) {
//     for mut transform in query.iter_mut() {
//         let mut direction = Vec3::ZERO;
//         if input.pressed(KeyCode::ArrowUp) {
//             direction.y += 1.0;
//         }
//         if input.pressed(KeyCode::ArrowDown) {
//             direction.y -= 1.0;
//         }
//         if input.pressed(KeyCode::ArrowLeft) {
//             direction.x -= 1.0;
//         }
//         if input.pressed(KeyCode::ArrowRight) {
//             direction.x += 1.0;
//         }

//         transform.translation +=
//             time.delta_secs() * 2.0 * direction;
//     }
// }
