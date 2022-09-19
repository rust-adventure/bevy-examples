//! Shows how to render a polygonal [`Mesh`], generated from a [`Quad`] primitive, in a 2D scene.
//! Adds a texture and colored vertices, giving per-vertex tinting.

use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderRef, SpecializedMeshPipelineError,
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
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(
            Material2dPlugin::<CustomMaterial>::default(),
        )
        .add_startup_system(setup)
        .add_system(update_time_in_shader)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    let mesh = Mesh::from(shape::Quad::default());

    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the quad with vertex colors
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_translation(Vec3::new(
            0., 0., 0.,
        ))
        .with_scale(Vec3::splat(4024.)),
        material: materials.add(CustomMaterial {
            color: Color::RED,
            time: time.seconds_since_startup() as f32,
        }),
        ..default()
    });
}

fn update_time_in_shader(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for material in materials.iter_mut() {
        material.1.time =
            time.seconds_since_startup() as f32;
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor
            .vertex
            .shader_defs
            .push("VERTEX_UVS".to_string());
        let fragment =
            descriptor.fragment.as_mut().unwrap();
        fragment.shader_defs.push("VERTEX_UVS".to_string());

        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
}
