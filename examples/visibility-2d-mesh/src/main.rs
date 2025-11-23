use std::f32::consts::FRAC_PI_3;

use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    color::palettes::tailwind::{
        GREEN_400, RED_400, SKY_400, YELLOW_400,
    },
    gltf::{GltfMesh, GltfPrimitive},
    input::common_conditions::input_just_pressed,
    log::tracing_subscriber::field::MakeExt,
    math::FloatOrd,
    mesh::{PrimitiveTopology, VertexAttributeValues},
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
    scene::SceneInstanceReady,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
    window::PrimaryWindow,
};
use bevy_skein::SkeinPlugin;
use itertools::Itertools;
use visibility_2d_mesh::{
    Obstacle, Player, VisibilityMesh,
    post_processing::{
        PostProcessPlugin, PostProcessSettings,
        VisibilityTexture,
    },
};

fn fmt_layer(
    _app: &mut App,
) -> Option<bevy::log::BoxedFmtLayer> {
    Some(Box::new(
        bevy::log::tracing_subscriber::fmt::Layer::default(
        )
        .without_time()
        .map_fmt_fields(|f| f.debug_alt())
        .with_writer(std::io::stderr),
    ))
}

fn main() {
    let mut app = App::new();
    app.insert_resource(visibility_2d_mesh::RayIndex(0))
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                fmt_layer,
                ..default()
            }),
            SkeinPlugin::default(),
            Wireframe2dPlugin::default(),
            MeshPickingPlugin::default(),
            visibility_2d_mesh::VisibilityMeshPlugin::default(),
            PostProcessPlugin
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_wireframe.run_if(
                    input_just_pressed(KeyCode::Space),
                ),
                visibility_2d_mesh::set_ray_index.run_if(
                    input_just_pressed(KeyCode::ArrowLeft)
                        .or(input_just_pressed(
                            KeyCode::ArrowRight,
                        )),
                ),
                update_visibility_texture,
                load_gltf
            ),
        )
        .add_observer(on_drag);
    app.run();
}

#[derive(Resource)]
struct Level(Handle<Gltf>);

fn load_gltf(
    mut reader: MessageReader<AssetEvent<Gltf>>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for event in reader.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                let gltf = gltf_assets.get(*id).unwrap();
                for primitive in gltf
                    .meshes
                    .iter()
                    .filter_map(|mesh_handle| {
                        gltf_meshes.get(mesh_handle)
                    })
                    .flat_map(|gltf_mesh| {
                        gltf_mesh.primitives.iter()
                    })
                {
                    let Some(mesh) =
                        meshes.get_mut(&primitive.mesh)
                    else {
                        continue;
                    };
                    // mesh.scale_by(Vec3::new(50., 50., 1.));
                    // mesh.transform_by(
                    //     Transform::default()
                    //         .with_scale(Vec3::splat(50.)),
                    // );
                }
                commands
                    .spawn(SceneRoot(
                        gltf.default_scene
                            .as_ref()
                            .unwrap()
                            .clone(),
                    ))
                    .observe(replace_3d_mats_with_2d);
            }
            _ => {}
        }
    }
}

fn replace_3d_mats_with_2d(
    _: On<SceneInstanceReady>,
    children: Query<&Children>,
    query: Query<(
        Entity,
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    materials_std: Res<Assets<StandardMaterial>>,
) {
    for (entity, mesh, material) in &query {
        commands
            .entity(entity)
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(Mesh2d(mesh.0.clone()))
            .insert(MeshMaterial2d(
                materials.add(
                    materials_std
                        .get(&material.0)
                        .unwrap()
                        .base_color,
                ),
            ));
        // Obstacle inserted in Blender
        // .insert(Obstacle);
    }
}
/// An observer to rotate an entity when it is
/// dragged
fn on_drag(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = transforms.get_mut(drag.entity)
    else {
        return;
    };
    transform.translation.x += drag.delta.x;
    transform.translation.y -= drag.delta.y;
}

const X_EXTENT: f32 = 1000.;
const Y_EXTENT: f32 = 150.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // gltf example loaded here
    // commands.insert_resource(Level(
    //     asset_server.load("Untitled.gltf"),
    // ));

    let image = Image::new_target_texture(
        512,
        512,
        TextureFormat::bevy_default(),
    );

    let image_handle = images.add(image);

    commands.spawn((
        Camera2d::default(),
        // Projection::Orthographic(OrthographicProjection {
        //     scaling_mode:
        //         bevy::camera::ScalingMode::FixedHorizontal {
        //             viewport_width: 1300.,
        //         },
        //     ..OrthographicProjection::default_2d()
        // }),
        PostProcessSettings {
            width: 512,
            height: 512,
            ..default()
        },
        VisibilityTexture(image_handle.clone()),
    ));

    // let shapes = [
    //     meshes.add(Circle::new(50.0)),
    //     meshes.add(CircularSector::new(50.0, 1.0)),
    //     meshes.add(CircularSegment::new(50.0, 1.25)),
    //     meshes.add(Ellipse::new(25.0, 50.0)),
    //     // meshes.add(Annulus::new(25.0, 50.0)),
    //     meshes.add(Capsule2d::new(25.0, 50.0)),
    //     meshes.add(build_rhombus(Vec2::new(75.0, 100.0))),
    //     meshes.add(Rectangle::new(50.0, 100.0)),
    //     meshes.add(RegularPolygon::new(50.0, 6)),
    //     meshes.add(Triangle2d::new(
    //         Vec2::Y * 50.0,
    //         Vec2::new(-50.0, -50.0),
    //         Vec2::new(50.0, -50.0),
    //     )),
    //     meshes.add(Segment2d::new(
    //         Vec2::new(-50.0, 50.0),
    //         Vec2::new(50.0, -50.0),
    //     )),
    //     meshes.add(Polyline2d::new(vec![
    //         Vec2::new(-50.0, 50.0),
    //         Vec2::new(0.0, -50.0),
    //         Vec2::new(50.0, 50.0),
    //     ])),
    // ];

    // let shapes = [
    //     meshes.add(Rectangle::new(50.0, 100.0)),
    //     meshes.add(Rectangle::new(1.0, 100.0)),
    //     meshes.add(Capsule2d::new(25.0, 50.0)),
    //     // meshes.add(Rhombus::new(75.0, 100.0)),
    //     meshes.add(build_rhombus(Vec2::new(75.0, 100.0))),
    //     meshes.add(Rectangle::new(100.0, 100.0)),
    // ];
    // let shapes = [
    //     meshes.add(Rectangle::new(50.0, 100.0)),
    //     meshes.add(Rectangle::new(50.0, 100.0)),
    // ];

    // let shapes = [
    //     meshes.add(Segment2d::new(
    //         Vec2::new(-50.0, 50.0),
    //         Vec2::new(50.0, -50.0),
    //     )),
    //     meshes.add(Polyline2d::new(vec![
    //         Vec2::new(-50.0, 50.0),
    //         Vec2::new(0.0, -50.0),
    //         Vec2::new(50.0, 50.0),
    //     ])),
    // ];

    // let num_shapes = shapes.len();

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(
    //         360. * i as f32 / num_shapes as f32,
    //         0.95,
    //         0.7,
    //     );

    //     commands.spawn((
    //         Mesh2d(shape),
    //         MeshMaterial2d(materials.add(color)),
    //         Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT/2 to
    //             // +X_EXTENT/2.
    //             -X_EXTENT / 2.
    //                 + i as f32 / (num_shapes - 1) as f32
    //                     * X_EXTENT,
    //             Y_EXTENT / 2.,
    //             0.0,
    //         ),
    //         Obstacle,
    //     ));
    // }

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1200.0, 800.0))),
        MeshMaterial2d(
            materials.add(Color::from(
                GREEN_400.with_alpha(0.01),
            )),
        ),
        Transform::default(),
        Obstacle,
    ));

    let empty_vec: Vec<[f32; 3]> = vec![];
    let vismesh = meshes.add(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        // Add 4 vertices, each with its own position
        // attribute (coordinate in 3D space), for
        // each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            empty_vec,
        ),
    );
    commands
        .insert_resource(VisibilityMesh(vismesh.clone()));

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: -1,
            target: image_handle.into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::from(SKY_400))),
        Transform::from_xyz(0., -40., 1.),
        Player,
        children!((
            Mesh2d(vismesh),
            MeshMaterial2d(materials.add(Color::BLACK),),
            RenderLayers::layer(1),
        )),
    ));
    // commands.spawn((
    //     Mesh2d(vismesh),
    //     MeshMaterial2d(
    //         materials.add(Color::from(
    //             YELLOW_400.with_alpha(0.5),
    //         )),
    //     ),
    //     Transform::from_xyz(0., 0., 10.),
    // ));
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
) {
    wireframe_config.global = !wireframe_config.global;
}

// fn create_simple_parallelogram() -> Mesh {
//     // Create a new mesh using a triangle list
// topology, where each set of 3 vertices composes
// a triangle.
//     Mesh::new(PrimitiveTopology::TriangleList,
// RenderAssetUsages::default())         // Add 4
// vertices, each with its own position attribute
// (coordinate in         // 3D space), for each
// of the corners of the parallelogram.
//         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_POSITION,
//             vec![[0.0, 0.0, 0.0], [1.0, 2.0,
// 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]
//         )
//         // Assign a UV coordinate to each
// vertex.         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_UV_0,
//             vec![[0.0, 1.0], [0.5, 0.0], [1.0,
// 0.0], [0.5, 1.0]]         )
//         // Assign normals (everything points
// outwards)         .with_inserted_attribute(
//             Mesh::ATTRIBUTE_NORMAL,
//             vec![[0.0, 0.0, 1.0], [0.0, 0.0,
// 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
//         )
//         // After defining all the vertices and
// their attributes, build each triangle using the
//         // indices of the vertices that make it
// up in a counter-clockwise order.
//         .with_inserted_indices(Indices::U32(vec![
//             // First triangle
//             0, 3, 1,
//             // Second triangle
//             1, 3, 2
//         ]))
// }

fn build_rhombus(size: Vec2) -> Mesh {
    // 75.0, 100.0
    let [hhd, vhd] = [size.x / 2., size.y / 2.];
    // let positions = vec![
    //     [hhd, 0.0, 0.0],
    //     [-hhd, 0.0, 0.0],
    //     [0.0, vhd, 0.0],
    //     [0.0, -vhd, 0.0],
    // ];
    let positions = vec![
        [hhd, 0.0, 0.0],
        [0.0, vhd, 0.0],
        [-hhd, 0.0, 0.0],
        [0.0, -vhd, 0.0],
    ];
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![
        [1.0, 0.5],
        [0.0, 0.5],
        [0.5, 0.0],
        [0.5, 1.0],
    ];
    let indices =
        bevy::mesh::Indices::U32(vec![2, 0, 1, 2, 3, 0]);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(indices)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn update_visibility_texture(
    mut query: Query<(
        &mut VisibilityTexture,
        &Camera,
        &mut PostProcessSettings,
    )>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = window.physical_size();
    for (texture, camera, mut settings) in &mut query {
        let Some(image) = images.get_mut(&texture.0) else {
            continue;
        };

        settings.width = size.x;
        settings.height = size.y;

        image.resize(Extent3d {
            width: size.x / 2, // TODO: this is the scale_factor, but how is this supposed to be handled?
            height: size.y / 2,
            depth_or_array_layers: 1,
        });
    }
}
