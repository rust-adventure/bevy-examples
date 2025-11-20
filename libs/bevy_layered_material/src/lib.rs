use std::path::PathBuf;

use bevy_app::Plugin;
use bevy_asset::AssetPath;
use bevy_pbr::{MaterialPlugin, ParallaxMappingMethod};
use bevy_shader::{load_shader_library, ShaderRef};

mod pbr_material;

pub use pbr_material::*;

pub struct LayeredMaterialsPlugin;

impl Plugin for LayeredMaterialsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_shader_library!(app, "render/pbr_types.wgsl");
        load_shader_library!(app, "render/pbr_bindings.wgsl");
        // load_shader_library!(app, "render/utils.wgsl");
        // load_shader_library!(app, "render/clustered_forward.wgsl");
        // load_shader_library!(app, "render/pbr_lighting.wgsl");
        // load_shader_library!(app, "render/pbr_transmission.wgsl");
        // load_shader_library!(app, "render/shadows.wgsl");
        // load_shader_library!(app, "deferred/pbr_deferred_types.wgsl");
        // load_shader_library!(app, "deferred/pbr_deferred_functions.wgsl");
        // load_shader_library!(app, "render/shadow_sampling.wgsl");
        load_shader_library!(app, "render/pbr_functions.wgsl");
        // load_shader_library!(app, "render/rgb9e5.wgsl");
        // load_shader_library!(app, "render/pbr_ambient.wgsl");
        load_shader_library!(app, "render/pbr_fragment.wgsl");
        load_shader_library!(app, "render/pbr.wgsl");
        // load_shader_library!(app, "render/pbr_prepass_functions.wgsl");
        load_shader_library!(app, "render/pbr_prepass.wgsl");
        // load_shader_library!(app, "render/parallax_mapping.wgsl");
        // load_shader_library!(app, "render/view_transformations.wgsl");

        app.add_plugins(MaterialPlugin::<LayeredMaterial>::default());
    }
}

// helper function from:
// https://github.com/bevyengine/bevy/blob/7ad5fb703c71b49f9cc91a7ea2684050c256394e/crates/bevy_pbr/src/lib.rs#L152C1-L154C2
fn shader_ref(path: PathBuf) -> ShaderRef {
    ShaderRef::Path(AssetPath::from_path_buf(path).with_source("embedded"))
}

// max_steps is usually on ParallaxMappingMethod
// but its pub(crate) so we can't access it.
// exact copy
// https://github.com/bevyengine/bevy/blob/7ad5fb703c71b49f9cc91a7ea2684050c256394e/crates/bevy_pbr/src/parallax.rs#L41
pub(crate) fn max_steps(mapping: &ParallaxMappingMethod) -> u32 {
    match mapping {
        ParallaxMappingMethod::Occlusion => 0,
        ParallaxMappingMethod::Relief { max_steps } => *max_steps,
    }
}
