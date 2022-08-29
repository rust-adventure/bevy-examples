use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct CubeMaterialPlugin;

impl Plugin for CubeMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            MaterialPlugin::<CubeMaterial>::default(),
        );
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-42ab-8225-97e2a3f056e0"]
// #[bind_group_data(StandardMaterialKey)]
// #[uniform(0, CubeMaterialUniform)]
pub struct CubeMaterial {
    pub color: Color,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
