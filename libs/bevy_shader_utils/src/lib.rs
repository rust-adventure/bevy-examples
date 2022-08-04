use bevy::{asset::HandleId, prelude::*};

// some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

pub const PERLIN_NOISE_2D: &str =
    include_str!("../shaders/perlin_noise_2d.wgsl");

pub const PERLIN_NOISE_3D: &str =
    include_str!("../shaders/perlin_noise_3d.wgsl");
pub const SIMPLEX_NOISE_2D: &str =
    include_str!("../shaders/simplex_noise_2d.wgsl");
pub const SIMPLEX_NOISE_3D: &str =
    include_str!("../shaders/simplex_noise_3d.wgsl");
pub const FBM: &str = include_str!("../shaders/fbm.wgsl");
pub const VORO_NOISE_2D: &str =
    include_str!("../shaders/voro_noise_2d.wgsl");
pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderUtils>();
    }
}
struct ShaderUtils {
    perlin_noise_2d: HandleId,
    perlin_noise_3d: HandleId,
    simplex_noise_2d: HandleId,
    simplex_noise_3d: HandleId,
    fbm: HandleId,
    voro_noise_2d: HandleId,
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
            fbm: load_shader(&mut shaders, "fbm", FBM),
            voro_noise_2d: load_shader(
                &mut shaders,
                "voro_noise_2d",
                VORO_NOISE_2D,
            ),
        }
    }
}

fn load_shader(
    shaders: &mut Mut<Assets<Shader>>,
    name: &str,
    shader_str: &'static str,
) -> HandleId {
    let mut shader = Shader::from_wgsl(shader_str);
    shader.set_import_path(format!(
        "bevy_shader_utils::{}",
        name
    ));
    let id = HandleId::random::<Shader>();
    shaders.set_untracked(id, shader);
    id
}
