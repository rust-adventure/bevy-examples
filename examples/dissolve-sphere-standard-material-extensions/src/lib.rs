use bevy::{
    pbr::{
        ExtendedMaterial, MaterialExtension,
        OpaqueRendererMethod,
    },
    prelude::*,
    reflect::TypePath,
    render::render_resource::*,
};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct DissolveExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    // #[uniform(100)]
    // quantize_steps: u32,
}

impl MaterialExtension for DissolveExtension {
    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/dissolve_material_prepass.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/dissolve_material.wgsl".into()
    }

    // fn deferred_fragment_shader() -> ShaderRef {
    //     "shaders/dissolve_material_prepass.wgsl".into()
    // }
}
