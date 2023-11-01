use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct CubeMaterialPlugin;

impl Plugin for CubeMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MaterialPlugin::<CubeMaterial>::default(),
        );
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CubeMaterial {
    pub color: Color,
}

impl Material for CubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
