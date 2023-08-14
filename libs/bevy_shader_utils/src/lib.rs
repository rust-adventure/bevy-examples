use bevy::{asset::HandleId, prelude::*};

// some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

// Noise Functions
pub const PERLIN_NOISE_2D: &str =
    include_str!("../shaders/perlin_noise_2d.wgsl");
pub const PERLIN_NOISE_3D: &str =
    include_str!("../shaders/perlin_noise_3d.wgsl");
pub const SIMPLEX_NOISE_2D: &str =
    include_str!("../shaders/simplex_noise_2d.wgsl");
pub const SIMPLEX_NOISE_3D: &str =
    include_str!("../shaders/simplex_noise_3d.wgsl");
// pub const FBM: &str =
// include_str!("../shaders/fbm.wgsl");
pub const VORONOISE: &str =
    include_str!("../shaders/voronoise.wgsl");
// other utility functions
pub const MOCK_FRESNEL: &str =
    include_str!("../shaders/mock_fresnel.wgsl");

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
/// #import bevy_shader_utils::perlin_noise_2d perlin_noise_2d
/// ```
///
pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderUtils>();
    }
}

#[allow(dead_code)]
#[derive(Resource)]
struct ShaderUtils {
    perlin_noise_2d: HandleId,
    perlin_noise_3d: HandleId,
    simplex_noise_2d: HandleId,
    simplex_noise_3d: HandleId,
    // fbm: HandleId,
    voronoise: HandleId,
    mock_fresnel: HandleId,
}

impl FromWorld for ShaderUtils {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world
            .get_resource_mut::<Assets<Shader>>()
            .unwrap();

        ShaderUtils {
            perlin_noise_2d: load_shader(
                &mut shaders,
                "perlin_noise_2d",
                PERLIN_NOISE_2D,
            ),
            perlin_noise_3d: load_shader(
                &mut shaders,
                "perlin_noise_3d",
                PERLIN_NOISE_3D,
            ),
            simplex_noise_2d: load_shader(
                &mut shaders,
                "simplex_noise_2d",
                SIMPLEX_NOISE_2D,
            ),
            simplex_noise_3d: load_shader(
                &mut shaders,
                "simplex_noise_3d",
                SIMPLEX_NOISE_3D,
            ),
            // TODO: no higher order functions, so
            // how would we even implement fbm in wgsl?
            // if you want it, copy/paste it from the
            // fbm.wgsl file.
            // fbm: load_shader(&mut shaders, "fbm", FBM),
            voronoise: load_shader(
                &mut shaders,
                "voronoise",
                VORONOISE,
            ),
            mock_fresnel: load_shader(
                &mut shaders,
                "mock_fresnel",
                MOCK_FRESNEL,
            ),
        }
    }
}

fn load_shader(
    shaders: &mut Mut<Assets<Shader>>,
    name: &str,
    shader_str: &'static str,
) -> HandleId {
    let shader = Shader::from_wgsl(shader_str, name);
    let id = HandleId::random::<Shader>();
    shaders.set_untracked(id, shader);
    id
}
