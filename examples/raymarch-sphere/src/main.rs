use bevy::{
    color::palettes::tailwind::BLUE_400,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
    },
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<SdfMaterial>::default(),
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SdfMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Rectangle::from_size(Vec2::splat(512.)))
            .into(),
        material: materials.add(SdfMaterial {
            color: BLUE_400.into(),
        }),
        ..default()
    });
}

impl Material2d for SdfMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sdf.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SdfMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}
