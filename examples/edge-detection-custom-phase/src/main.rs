use std::f32::consts::PI;

use bevy::{
    color::palettes::tailwind::{
        GREEN_400, SLATE_800, SLATE_950,
    },
    gltf::GltfPlugin,
    prelude::*,
    render::mesh::VertexAttributeValues,
};
use edge_detection_custom_phase::{
    ATTRIBUTE_SECTION_COLOR, DrawSection, SectionGroupId,
    SectionTexturePhasePlugin, SectionsPrepass,
    post_process::{
        PostProcessPlugin, PostProcessSettings,
    },
};

fn main() {
    App::new()
        // bevy plugins
        .add_plugins((
            DefaultPlugins.set(
                GltfPlugin::default()
                    // Map a custom glTF attribute name to a `MeshVertexAttribute`.
                    .add_custom_vertex_attribute(
                        "SECTION_COLOR",
                        ATTRIBUTE_SECTION_COLOR,
                    ),
            ),
            MeshPickingPlugin,
        ))
        // our plugins
        .add_plugins((
            SectionTexturePhasePlugin,
            PostProcessPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, vertical))
        .add_observer(
            |trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             query: Query<
                Has<DrawSection>,
                With<DemoShape>,
            >| {
                let Ok(has_draw_section) =
                    query.get(trigger.target())
                else {
                    return;
                };
                match has_draw_section {
                    true => {
                        commands
                            .entity(trigger.target())
                            .remove::<DrawSection>();
                    }
                    false => {
                        commands
                            .entity(trigger.target())
                            .insert(DrawSection);
                    }
                }
            },
        )
        .run();
}

const SHAPES_X_EXTENT: f32 = 14.0;
// const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
                .map(|_| [0.5, 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
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
                            [0.1, 0., 0., 1.]
                        }
                        (false, true, false) => {
                            [1., 0., 0., 1.]
                        }
                        (false, false, true) => {
                            [0.6, 0., 0., 1.]
                        }
                        _ => [0., 0., 0., 1.],
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
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
                ATTRIBUTE_SECTION_COLOR,
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
                .map(|[_x, _y, _z]| [0.5, 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
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
                .map(|_| [0.5, 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
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
                ATTRIBUTE_SECTION_COLOR,
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
                ATTRIBUTE_SECTION_COLOR,
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
                ATTRIBUTE_SECTION_COLOR,
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
                .map(|[x, _y, _z]| [0., 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
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
                .map(|[x, _y, _z]| [0.5, 0., 0., 1.])
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
                colors,
            )
        }),
    ];

    // cube
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(materials.add(
                StandardMaterial {
                    base_color: GREEN_400.into(),
                    ..default()
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
            DrawSection,
            DemoShape,
            // TODO: more testing for SectionGroupId overrides
            // until then, probably not end-user usable
            // SectionGroupId { id: 3 },
        ));
    }

    // background cubes
    commands.spawn((
        Mesh3d(meshes.add({
            let mesh =
                Cuboid::new(20., 2., 2.).mesh().build();
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
                            [0.2, 0., 0., 1.]
                        }
                        (false, false, true) => {
                            [0.6, 0., 0., 1.]
                        }
                        _ => [0., 0., 0., 1.],
                    }
                })
                .collect();

            mesh.with_inserted_attribute(
                ATTRIBUTE_SECTION_COLOR,
                colors,
            )
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SLATE_800.into(),
            ..default()
        })),
        Transform::from_xyz(0., 2.0, -Z_EXTENT / 2.)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        Vertical,
        DrawSection,
        DemoShape,
    ));

    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Camera {
            clear_color: Color::from(SLATE_950).into(),
            // THIS ONLY WORKS WITH HDR CAMERAS
            hdr: true,
            ..default()
        },
        // disable msaa for simplicity
        Msaa::Off,
        PostProcessSettings {
            stroke_color: Color::WHITE.into(),
            width: 2,
        },
        SectionsPrepass,
    ));
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

#[derive(Component)]
struct Vertical;

fn vertical(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Vertical>>,
) {
    for mut transform in &mut query {
        transform.translation.y =
            (time.elapsed_secs() / 1.).sin() * 3.;
    }
}

#[derive(Component)]
struct DemoShape;
