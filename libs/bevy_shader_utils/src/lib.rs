use bevy::{
    asset::{embedded_asset, load_internal_asset},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

// Noise Functions

const PERLIN_NOISE_2D: Handle<Shader> =
    Handle::weak_from_u128(11918512342344596158);
pub const PERLIN_NOISE_3D: Handle<Shader> =
    Handle::weak_from_u128(11918512442344596158);
pub const SIMPLEX_NOISE_2D: Handle<Shader> =
    Handle::weak_from_u128(11918512542344596158);
pub const SIMPLEX_NOISE_3D: Handle<Shader> =
    Handle::weak_from_u128(11918512642344596158);
// pub const FBM: Handle<Shader> =
// Handle::weak_from_u128(11918512342344596158);
pub const VORONOISE: Handle<Shader> =
    Handle::weak_from_u128(11918512742344596158);
// other utility functions
pub const MOCK_FRESNEL: Handle<Shader> =
    Handle::weak_from_u128(11918512842344596158);
pub const PRISTINE_GRID: Handle<Shader> =
    Handle::weak_from_u128(11918512942344596158);

/// To use the shader utility functions, add the plugin to your
/// app.
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_shader_utils::ShaderUtilsPlugin;
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         ShaderUtilsPlugin,
///     ));
/// ```
///
/// then import the relevant function in your shader.
///
/// ```ignore
/// #import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d
/// ```
///
pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        // embedded_asset!(
        //     app,
        //     "shaders/perlin_noise_2d.wgsl"
        // );
        // embedded_asset!(
        //     app,
        //     "shaders/perlin_noise_3d.wgsl"
        // );
        // embedded_asset!(
        //     app,
        //     "shaders/simplex_noise_2d.wgsl"
        // );
        // embedded_asset!(
        //     app,
        //     "shaders/simplex_noise_3d.wgsl"
        // );
        // embedded_asset!(app, "shaders/voronoise.wgsl");
        // embedded_asset!(app, "shaders/mock_fresnel.wgsl");
        // embedded_asset!(app, "shaders/pristine_grid.wgsl");

        embedded_asset!(
            app,
            "materials/pristine_grid.wgsl"
        );

        load_internal_asset!(
            app,
            PERLIN_NOISE_2D,
            "shaders/perlin_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PERLIN_NOISE_3D,
            "shaders/perlin_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_2D,
            "shaders/simplex_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_3D,
            "shaders/simplex_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VORONOISE,
            "shaders/voronoise.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            MOCK_FRESNEL,
            "shaders/mock_fresnel.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PRISTINE_GRID,
            "shaders/pristine_grid.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<
            PristineGridMaterial,
        >::default());
    }
}

impl Material for PristineGridMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_shader_utils/materials/pristine_grid.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PristineGridMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub cell_multiplier: Vec2,
    #[uniform(0)]
    pub line_size: Vec2,
}

impl Default for PristineGridMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::WHITE,
            cell_multiplier: Vec2::splat(10.),
            line_size: Vec2::splat(0.1),
        }
    }
}
