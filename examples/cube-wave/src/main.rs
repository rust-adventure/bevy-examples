use bevy::{
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        NotShadowCaster,
    },
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        camera::Projection,
        mesh::{
            MeshVertexBufferLayout, VertexAttributeValues,
        },
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType,
            RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError,
        },
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("e3eefc").unwrap(),
        ))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // watch_for_changes: true,
                ..default()
            }),
            ShaderUtilsPlugin,
            MaterialPlugin::<CustomMaterial>::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let cube_size = 0.2;
    let mut mesh =
        Mesh::from(shape::Cube { size: cube_size });
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
        let material =
            custom_materials.add(CustomMaterial {
                // time: 0.0,
                offset: ((f32::abs((x - half) as f32)
                    + f32::abs((z - half) as f32))
                    as f32)
                    .abs(),
                color: Color::rgb(0.92, 0.90, 0.73),
            });
        commands
            .spawn(MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    cube_size * x as f32
                        - num_squares as f32 * cube_size
                            / 2.0,
                    0.0,
                    cube_size * z as f32
                        - num_squares as f32 * cube_size
                            / 2.0,
                ),
                ..default()
            })
            .insert(NotShadowCaster);
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(
            OrthographicProjection {
                scale: 0.008,
                ..Default::default()
            },
        ),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-42ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    offset: f32,
    #[uniform(0)]
    color: Color,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/vertex_shader.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("cubes_{}", *label).into();
        }
        Ok(())
    }
}
