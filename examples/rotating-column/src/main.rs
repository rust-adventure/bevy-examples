use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{
        camera::Projection, mesh::VertexAttributeValues,
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use bevy_tweening::*;

use rotating_column::{
    cube::{SpawnCube, TweenEvents},
    materials::{CubeMaterial, CubeMaterialPlugin},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("e3eefc").unwrap(),
        ))
        .add_plugins((
            DefaultPlugins,
            TweeningPlugin,
            ShaderUtilsPlugin,
            CubeMaterialPlugin,
        ))
        .add_systems(Startup, setup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_systems(Update, cube_completed)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cube_materials: ResMut<Assets<CubeMaterial>>,
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
    let num_squares = 4;
    for y in 0..num_squares {
        let material = cube_materials.add(CubeMaterial {
            color: Color::rgb(0.92, 0.90, 0.73),
        });

        commands
            .spawn(MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    0.0,
                    cube_size * y as f32
                        - num_squares as f32 * cube_size,
                    0.0,
                ),
                ..default()
            })
            .insert(NotShadowCaster);
    }

    commands.add(SpawnCube);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(
            OrthographicProjection {
                // left: -num_squares as f32 * 0.2,
                // right: num_squares as f32 * 0.2,
                // bottom: 0.0,
                // top: 2.0,
                // near: todo!(),
                // far: todo!(),
                // window_origin: todo!(),
                // scaling_mode: todo!(),
                scale: 0.001,
                // depth_calculation: todo!(),
                ..Default::default()
            },
        ),
        ..default()
    });
}

fn cube_completed(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
) {
    for ev in reader.iter() {
        match TweenEvents::try_from(ev.user_data) {
            Ok(TweenEvents::SpawnNewCube) => {
                commands.add(SpawnCube);
            }
            Ok(TweenEvents::DespawnSelf) => {
                commands
                    .entity(ev.entity)
                    .despawn_recursive();
            }
            Err(err) => warn!(err),
        };
    }
}
