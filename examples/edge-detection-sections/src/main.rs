//! This example shows how to create a custom render pass that runs after the main pass
//! and reads the texture generated by the main pass.
//!
//! The example shader uses a storage_texture containing the vertex color data to run a sobel filter, then
//! mixes that output with the main pass output to render an artist-controlled outline.
//!

use std::f32::consts::PI;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::*,
    pbr::ExtendedMaterial,
    prelude::*,
    render::{
        mesh::VertexAttributeValues, render_resource::*,
    },
    scene::SceneInstanceReady,
};
use edge_detection_sections::{
    post_process::{
        PostProcessPlugin, PostProcessSettings,
    },
    vertex_color_material::{
        VertexColorSectionId,
        VertexColorSectionTestMaterial,
        VertexColorSectionsPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            VertexColorSectionsPlugin,
            PostProcessPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

// demo
const SHAPES_X_EXTENT: f32 = 14.0;
// const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                VertexColorSectionTestMaterial,
            >,
        >,
    >,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 2048,
        height: 2048,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        // TextureFormat::Bgra8UnormSrgb,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::STORAGE_BINDING;

    let image_handle = images.add(image);

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Camera {
            clear_color: Color::from(SLATE_950).into(),
            ..default()
        },
        PostProcessSettings {
            stroke_color: SLATE_50.into(),
            width: 2,
        },
        VertexColorSectionId(image_handle.clone()),
    ));

    // a number of shapes, all with their vertex colors set
    // in fairly arbitrary ways. Typically this is done by
    // using the normal information to calulate colors for
    // the faces, but in practice these colors will be
    // hand-selected by an artist in software like Blender
    // where they can then control which lines appear based
    // on where they create "edges" by selecting sufficiently
    // different colored faces.
    let shapes = [
        meshes.add({
            let mesh = Cuboid::default().mesh().build();

            // cube without inner lines
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                return;
            };

            // no cube internal edges become lines,
            // which means the silhouette is the only
            // thing outlined
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|_| [1., 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Cuboid::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                return;
            };

            // all cube edges become lines
            // cube normals are always 1 (or -1) on one axis
            // and 0 on the other two axes
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, y, z]| {
                    match (*x != 0., *y != 0., *z != 0.) {
                        (true, false, false) => {
                            [1., 0., 0., 1.]
                        }
                        (false, true, false) => {
                            [0.02, 0., 0., 1.]
                        }
                        (false, false, true) => {
                            [0.06, 0., 0., 1.]
                        }
                        _ => [0., 0., 0., 1.],
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh =
                Tetrahedron::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, y, z]| {
                    match (*x > 0., *y > 0., *z > 0.) {
                        (false, false, false) => {
                            [0.8, 0., 0., 1.]
                        }
                        (true, false, true) => {
                            [1., 0., 0., 1.]
                        }
                        (true, true, false) => {
                            [0.2, 0., 0., 1.]
                        }
                        (false, true, true) => {
                            [0.6, 0., 0., 1.]
                        }
                        _ => [0., 0., 0., 1.],
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Capsule3d::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, _y, _z]| {
                    [(x + 0.5 * 100.).floor(), 0., 0., 1.]
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Torus::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, _y, _z]| {
                    [(x + 0.5 * 100.).floor(), 0., 0., 1.]
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Cylinder::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_x, y, _z]| {
                    if *y == 1. || *y == -1. {
                        [1., 0., 0., 1.]
                    } else {
                        [0.5, 0., 0., 1.]
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Cone::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_x, y, _z]| {
                    if *y == -1. {
                        [1., 0., 0., 1.]
                    } else {
                        [0.5, 0., 0., 1.]
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh =
                ConicalFrustum::default().mesh().build();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_x, y, _z]| {
                    if *y == 1. || *y == -1. {
                        [1., 0., 0., 1.]
                    } else {
                        [0.5, 0., 0., 1.]
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh =
                Sphere::default().mesh().ico(5).unwrap();
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, _y, _z]| {
                    [(x + 0.5 * 100.).floor(), 0., 0., 1.]
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
        meshes.add({
            let mesh = Sphere::default().mesh().uv(32, 18);
            let Some(VertexAttributeValues::Float32x3(
                positions,
            )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                return;
            };

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, _y, _z]| {
                    [(x + 0.5 * 100.).floor(), 0., 0., 1.]
                })
                .collect();

            mesh.with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            )
        }),
    ];

    // let mesh = Cuboid::default().mesh().build();
    // let Some(VertexAttributeValues::Float32x3(positions)) =
    //     mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    // else {
    //     return;
    // };

    // let colors: Vec<[f32; 4]> = positions
    //     .iter()
    //     .map(|[x, _y, _z]| {
    //
    //         [(x + 0.5 * 100.).floor(), 0., 0., 1.]
    //     })
    //     .collect();

    // let cuboid = mesh.with_inserted_attribute(
    //     Mesh::ATTRIBUTE_COLOR,
    //     colors,
    // );
    // cube
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(materials.add(
                ExtendedMaterial {
                    base: StandardMaterial {
                        base_color: GREEN_400.into(),
                        ..default()
                    },
                    extension:
                        VertexColorSectionTestMaterial {
                            quantize_steps: 3,
                            storage_texture:
                                image_handle.clone(),
                        },
                },
            )),
            Transform::from_xyz(
                -SHAPES_X_EXTENT / 2.
                    + i as f32 / (num_shapes - 1) as f32
                        * SHAPES_X_EXTENT,
                2.0,
                Z_EXTENT / 2.,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Rotates,
        ));
    }

    // gltf cube
    commands
        .spawn((
            SceneRoot(
                asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset(
                        "vertex_painted_cube.glb",
                    ),
                ),
            ),
            Transform::from_xyz(0.0, 4., 0.0),
            Rotates,
        ))
        .observe(on_level_spawn);

    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotates>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_secs());
        transform.rotate_z(0.15 * time.delta_secs());
    }
}

fn on_level_spawn(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                VertexColorSectionTestMaterial,
            >,
        >,
    >,
    std_materials: Res<Assets<StandardMaterial>>,
    children: Query<&Children>,
    entities_with_std_mat: Query<
        &MeshMaterial3d<StandardMaterial>,
    >,
) {
    let extension = materials
        .iter()
        .next()
        .unwrap()
        .1
        .extension
        .clone();

    for entity in
        children.iter_descendants(trigger.entity())
    {
        // swap standard material for extended material
        let Ok(mat) = entities_with_std_mat.get(entity)
        else {
            continue;
        };

        let old_mat = std_materials.get(&mat.0).unwrap();
        let new_mat = materials.add(ExtendedMaterial {
            base: old_mat.clone(),
            extension: extension.clone(),
        });
        commands
            .entity(entity)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(new_mat));
    }
}
