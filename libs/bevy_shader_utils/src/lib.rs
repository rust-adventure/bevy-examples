use bevy::{
    asset::{
        embedded_asset, load_internal_asset, uuid_handle,
    },
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

// some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

// Noise Functions

const PERLIN_NOISE_2D: Handle<Shader> =
    uuid_handle!("990ee5ac-3d4a-4593-841f-6b46f02abcb3");
pub const PERLIN_NOISE_3D: Handle<Shader> =
    uuid_handle!("70568ff4-c5ca-4411-b099-692926332401");
pub const SIMPLEX_NOISE_2D: Handle<Shader> =
    uuid_handle!("f6a37262-741d-442e-b990-fb2851253284");
pub const SIMPLEX_NOISE_3D: Handle<Shader> =
    uuid_handle!("6239ad8c-41e1-4302-8f03-b2e41c154764");
// pub const FBM: Handle<Shader> =
// uuid_handle!("b84472cd-83dc-4cc2-b18a-021b42d11cb8");
pub const VORONOISE: Handle<Shader> =
    uuid_handle!("06dc4bde-2702-4fe2-aff5-df69078c4b59");
// other utility functions
pub const MOCK_FRESNEL: Handle<Shader> =
    uuid_handle!("294d2ab5-554b-41f2-a1ca-95fb3689c1c3");
pub const PRISTINE_GRID: Handle<Shader> =
    uuid_handle!("3511cc56-fc6c-44c6-bde3-1591164fbc79");

/// To use the shader utility functions, add the
/// plugin to your app.
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
/// then import the relevant function in your
/// shader.
///
/// ```ignore
/// #import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d
/// ```
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
        // embedded_asset!(app,
        // "shaders/mock_fresnel.wgsl");
        // embedded_asset!(app,
        // "shaders/pristine_grid.wgsl");

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

// This is the struct that will be passed to your
// shader
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
