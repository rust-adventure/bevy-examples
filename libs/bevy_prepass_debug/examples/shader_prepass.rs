//! Bevy has an optional prepass that is
//! controlled per-material. A prepass is a
//! rendering pass that runs before the main pass.
//! It will optionally generate various view
//! textures. Currently it supports depth, normal,
//! and motion vector textures. The textures are
//! not generated for any material using alpha
//! blending.

use bevy::{
    core_pipeline::prepass::{
        DepthPrepass, MotionVectorPrepass, NormalPrepass,
    },
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
use bevy_prepass_debug::PrepassDebugPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PrepassDebugPlugin,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        // .insert_resource(Msaa::Off)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3., 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
        // To enable the prepass you need to add the
        // components associated with the ones you need
        // This will write the depth buffer to a texture
        // that you can use in the main pass
        DepthPrepass,
        // This will generate a texture containing world
        // normals (with normal maps applied)
        NormalPrepass,
        // This will generate a texture containing screen
        // space pixel motion vectors
        MotionVectorPrepass,
    ));

    // plane
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(5., 5.)
                    .subdivisions(10),
            ),
        ),
        MeshMaterial3d(
            std_materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ),
    ));

    // Opaque cube using the StandardMaterial
    commands.spawn((
        Mesh3d(
            meshes.add(Mesh::from(Cuboid::from_size(
                Vec3::ONE,
            ))),
        ),
        MeshMaterial3d(
            std_materials.add(Color::srgb(0.8, 0.7, 0.6)),
        ),
        Transform::from_xyz(-1.0, 0.5, 0.0),
    ));

    // Cube with alpha mask
    commands.spawn((
        Mesh3d(
            meshes.add(Mesh::from(Cuboid::from_size(
                Vec3::ONE,
            ))),
        ),
        MeshMaterial3d(std_materials.add(
            StandardMaterial {
                alpha_mode: AlphaMode::Mask(1.0),
                base_color_texture: Some(
                    asset_server.load("branding/icon.png"),
                ),
                ..default()
            },
        )),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Cube with alpha blending.
    // Transparent materials are ignored by the
    // prepass
    commands.spawn((
        Mesh2d(
            meshes.add(Mesh::from(Cuboid::from_size(
                Vec3::ONE,
            ))),
        ),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(
                asset_server.load("branding/icon.png"),
            ),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(1.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

// This is the struct that will be passed to your
// shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// Not shown in this example, but if you need to
/// specialize your material, the specialize
/// function will also be used by the prepass
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // You can override the default shaders used in
    // the prepass if your material does
    // anything not supported by the default prepass
    // fn prepass_fragment_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }
}
