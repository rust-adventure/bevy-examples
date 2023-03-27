//! Shows how to render a polygonal [`Mesh`], generated from a [`Quad`] primitive, in a 2D scene.
//! Adds a texture and colored vertices, giving per-vertex tinting.

use bevy::{
    pbr::{MAX_CASCADES_PER_LIGHT, MAX_DIRECTIONAL_LIGHTS},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderDefVal, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
    sprite::{
        Material2d, Material2dKey, Material2dPlugin,
        MaterialMesh2dBundle,
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(
            Material2dPlugin::<CustomMaterial>::default(),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    // time: Res<Time>,
) {
    let mesh = Mesh::from(shape::Quad::default());

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the quad with vertex colors
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_translation(Vec3::new(
            0., 0., 0.,
        ))
        .with_scale(Vec3::splat(4024.)),
        material: materials
            .add(CustomMaterial { color: Color::RED }),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}
