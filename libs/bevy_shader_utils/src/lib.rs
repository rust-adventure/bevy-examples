use bevy::{asset::HandleId, prelude::*};

// some wgsl from https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

pub const perlin_noise_2d: &str =
    include_str!("../shaders/perlin_noise_2d.wgsl");

pub const perlin_noise_3d: &str =
    include_str!("../shaders/perlin_noise_3d.wgsl");
pub const simplex_noise_2d: &str =
    include_str!("../shaders/simplex_noise_2d.wgsl");
pub const simplex_noise_3d: &str =
    include_str!("../shaders/simplex_noise_3d.wgsl");
pub const fbm: &str = include_str!("../shaders/fbm.wgsl");
pub const voro_noise_2d: &str =
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
                perlin_noise_2d,
            ),
            perlin_noise_3d: load_shader(
                &mut shaders,
                "perlin_noise_3d",
                perlin_noise_3d,
            ),
            simplex_noise_2d: load_shader(
                &mut shaders,
                "simplex_noise_2d",
                simplex_noise_2d,
            ),
            simplex_noise_3d: load_shader(
                &mut shaders,
                "simplex_noise_3d",
                simplex_noise_3d,
            ),
            fbm: load_shader(&mut shaders, "fbm", fbm),
            voro_noise_2d: load_shader(
                &mut shaders,
                "voro_noise_2d",
                voro_noise_2d,
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
