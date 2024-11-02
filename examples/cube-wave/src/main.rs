use bevy::{
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        NotShadowCaster,
    },
    prelude::*,
    reflect::TypePath,
    render::{
        camera::{Projection, ScalingMode},
        mesh::VertexAttributeValues,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("e3eefc").unwrap().into(),
        ))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // watch_for_changes: true,
                ..default()
            }),
            ShaderUtilsPlugin,
            MaterialPlugin::<CubeMaterial>::default(),
            // WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CubeMaterial>>,
) {
    let cube_size = 0.2;
    let mut mesh = Mesh::from(Cuboid::from_size(
        Vec3::splat(cube_size),
    ));
    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| {
                [
                    (1. - *r) / 2.,
                    (1. - *g) / 2.,
                    (1. - *b) / 2.,
                    1.,
                ]
            })
            .collect();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors,
        );
    };

    let mesh_handle = meshes.add(mesh);
    let num_squares = 15;
    let half = (num_squares - 1) / 2;
    for (z, x) in
        (0..num_squares).cartesian_product(0..num_squares)
    {
        let material = custom_materials.add(CubeMaterial {
            // time: 0.0,
            offset: ((f32::abs((x - half) as f32)
                + f32::abs((z - half) as f32))
                as f32)
                .abs(),
            color: Color::srgb(0.92, 0.90, 0.73)
                .to_linear(),
        });
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(
                cube_size * x as f32
                    - num_squares as f32 * cube_size / 2.0,
                0.0,
                cube_size * z as f32
                    - num_squares as f32 * cube_size / 2.0,
            ),
            NotShadowCaster,
        ));
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Orthographic(OrthographicProjection {
            // scale: 0.008,
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: 10.,
            },
            ..OrthographicProjection::default_3d()
        }),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CubeMaterial {
    #[uniform(0)]
    offset: f32,
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for CubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cube_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/cube_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(label) = &mut descriptor.label {
            *label = format!("cubes_{}", *label).into();
        }
        Ok(())
    }
}
