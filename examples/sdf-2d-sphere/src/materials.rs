use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

pub struct SdfMaterialPlugin;

impl Plugin for SdfMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            Material2dPlugin::<SdfMaterial>::default(),
        );
    }
}
/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for SdfMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/sdf.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/sdf.wgsl".into()
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
pub struct SdfMaterial {
    #[uniform(0)]
    pub color: Color,
}
