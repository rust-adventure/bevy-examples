use bevy::{
    asset::load_internal_asset,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::*,
    },
};

const PBR_FRAGMENT_REPLACEMENT: Handle<Shader> =
    Handle::weak_from_u128(11924612342344596158);

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "extended_material.wgsl";

pub struct VertexColorSectionsPlugin;

impl Plugin for VertexColorSectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                VertexColorSectionTestMaterial,
            >,
        >::default());

        // load the custom replacement for bevy_pbr::pbr_fragment
        // that removes vertex coloring
        load_internal_asset!(
            app,
            PBR_FRAGMENT_REPLACEMENT,
            "custom_pbr_fragment.wgsl",
            Shader::from_wgsl
        );
    }
}

#[derive(Clone, Component, ExtractComponent)]
pub struct VertexColorSectionId(pub Handle<Image>);

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct VertexColorSectionTestMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub quantize_steps: u32,
    #[storage_texture(101, visibility(all))]
    pub storage_texture: Handle<Image>,
}

impl MaterialExtension for VertexColorSectionTestMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
