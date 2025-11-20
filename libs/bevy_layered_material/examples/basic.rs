//! A LayeredMaterial in action

use bevy::{
    image::ImageLoaderSettings, prelude::*, reflect::TypePath,
    render::render_resource::AsBindGroup, shader::ShaderRef,
};
use bevy_color::palettes::tailwind::*;
use bevy_layered_materials::{LayeredMaterial, LayeredMaterialsPlugin};
use bevy_math::Affine2;

/// This example uses a shader source file from the assets subdirectory
// const SHADER_ASSET_PATH: &str = "custom_material.wgsl";

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_800.into()))
        .add_plugins((DefaultPlugins, LayeredMaterialsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, swap_layers)
        .run();
}

fn swap_layers(
    query: Query<&mut MeshMaterial3d<LayeredMaterial>>,
    mut materials: ResMut<Assets<LayeredMaterial>>,
    time: Res<Time>,
) {
    for mat in &query {
        let material = materials.get_mut(mat).unwrap();
        material.layer_index = (time.elapsed_secs() as u32 / 2) % 3;
        // material.base_color = Color::hsl(material.base_color.hue() + 1., 1., 2.);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LayeredMaterial>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
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
    cube.generate_tangents().unwrap();
    commands.spawn((
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(LayeredMaterial {
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
        })),
        Transform::from_xyz(0.0, 1., 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
