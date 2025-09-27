//! Shows how to render a polygonal [`Mesh`],
//! generated from a [`Quad`] primitive, in a 2D
//! scene. Adds a texture and colored vertices,
//! giving per-vertex tinting.

use bevy::{
    color::palettes::tailwind::*,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
            ShaderUtilsPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mesh = Rectangle::default();

    // Spawn camera
    commands.spawn(Camera2d::default());

    // Spawn the quad with vertex colors
    commands.spawn((
        Mesh2d(meshes.add(mesh).into()),
        Transform::from_translation(Vec3::new(0., 0., 0.))
            .with_scale(Vec3::splat(4024.)),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: RED_400.into(),
        })),
    ));
}

/// The Material trait is very configurable, but
/// comes with sensible defaults for all methods.
/// You only need to implement functions for
/// features that need non-default behavior. See
/// the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// This is the struct that will be passed to your
// shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}
