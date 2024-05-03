use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Component)]
pub struct AppStartLoadingIndicator;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct LoadingUiMaterial {
    #[uniform(0)]
    pub color: Vec4,
    #[uniform(0)]
    pub progress: f32,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Handle<Image>,
}

impl UiMaterial for LoadingUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/loading.wgsl".into()
    }
}
