use bevy::{asset::load_internal_asset, prelude::*};

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
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         ShaderUtilsPlugin,
///     )
/// );
/// ```
///
/// then import the relevant function in your shader.
///
/// ```
/// #import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d
/// ```
///
pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PERLIN_NOISE_2D,
            "../shaders/perlin_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PERLIN_NOISE_3D,
            "../shaders/perlin_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_2D,
            "../shaders/simplex_noise_2d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_NOISE_3D,
            "../shaders/simplex_noise_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VORONOISE,
            "../shaders/voronoise.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            MOCK_FRESNEL,
            "../shaders/mock_fresnel.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PRISTINE_GRID,
            "../shaders/pristine_grid.wgsl",
            Shader::from_wgsl
        );
    }
}
