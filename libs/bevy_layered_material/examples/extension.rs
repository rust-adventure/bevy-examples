//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use bevy::{
    color::palettes::basic::RED,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::*,
    shader::ShaderRef,
};
use bevy_image::ImageLoaderSettings;
use bevy_layered_materials::{LayeredMaterial, LayeredMaterialsPlugin};
use bevy_math::Affine2;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "extension.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LayeredMaterialsPlugin))
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<LayeredMaterial, MyExtension>,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<LayeredMaterial, MyExtension>>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials_std.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    let mut cube = Cuboid::new(2.0, 2.0, 2.0).mesh().build();
    // tangents are required for normals to affect lighting!
    cube.generate_tangents().unwrap();
    commands.spawn((
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: LayeredMaterial {
                // layer: u32,
                // base_color: LinearRgba::RED.into(),
                base_color_texture: Some(asset_server.load_with_settings(
                    "base_color.ktx2",
                    |settings: &mut ImageLoaderSettings| {
                        settings.is_srgb = true;
                        let sampler = settings.sampler.get_or_init_descriptor();
                        sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                    },
                )),
                normal_map_texture: Some(asset_server.load_with_settings(
                    "normal_map.ktx2",
                    |settings: &mut ImageLoaderSettings| {
                        // settings.is_srgb = true;
                        let sampler = settings.sampler.get_or_init_descriptor();
                        sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                    },
                )),
                // base_color_texture: Some(asset_server.load_with_settings(
                //     "base_color.ktx2",
                //     |settings: &mut ImageLoaderSettings| {
                //         settings.is_srgb = true;
                //         let sampler = settings.sampler.get_or_init_descriptor();
                //         sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                //         sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                //         sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                //     },
                // )),
                uv_transform: Affine2::from_scale(Vec2::splat(1. / 2.)),
                ..default()
            },
            extension: MyExtension::new(1),
        })),
        Transform::from_xyz(0.0, 1., 0.0),
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate,
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
struct MyExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
    // Web examples WebGL2 support: structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_8b: u32,
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_12b: u32,
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_16b: u32,
}
impl MyExtension {
    fn new(quantize_steps: u32) -> Self {
        Self {
            quantize_steps,
            ..default()
        }
    }
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
