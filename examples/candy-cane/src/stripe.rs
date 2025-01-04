//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        render_resource::*, storage::ShaderStorageBuffer,
    },
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/candy_cane.wgsl";

pub struct StripeMaterialPlugin;

impl Plugin for StripeMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                CandyCaneMaterial,
            >,
        >::default());
    }
}

#[derive(
    encase::ShaderType, Default, Debug, Clone, Copy,
)]
pub struct Stripe {
    pub frequency: f32,
    pub minimum_value: f32,
    pub power_value: f32,
    pub offset: f32,
    pub color: LinearRgba,
}

impl From<Stripe> for [f32; 8] {
    fn from(stripe: Stripe) -> Self {
        [
            stripe.frequency,
            stripe.minimum_value,
            stripe.power_value,
            stripe.offset,
            stripe.color.red,
            stripe.color.green,
            stripe.color.blue,
            stripe.color.alpha,
        ]
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct CandyCaneMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[storage(100, read_only)]
    pub stripes_buffer: Handle<ShaderStorageBuffer>,
}

impl MaterialExtension for CandyCaneMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
