use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::{
        GREEN_400, RED_400, SKY_400, YELLOW_400,
    },
    input::common_conditions::input_just_pressed,
    log::tracing_subscriber::field::MakeExt,
    math::FloatOrd,
    mesh::{PrimitiveTopology, VertexAttributeValues},
    prelude::*,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
};
use itertools::Itertools;
use visibility_2d_mesh::{
    Obstacle, Player, VisibilityMesh,
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
            Wireframe2dPlugin::default(),
            MeshPickingPlugin::default(),
            visibility_2d_mesh::VisibilityMeshPlugin::default()
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
            ),
        )
        .add_observer(on_drag);
    app.run();
}

// fn move_player(
//     mut query: Query<&mut Transform,
// With<Player>>, ) {
// }

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
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode:
                bevy::camera::ScalingMode::FixedHorizontal {
                    viewport_width: 1300.,
                },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // let shapes = [
    //     meshes.add(Circle::new(50.0)),
    //     meshes.add(CircularSector::new(50.0, 1.0)),
    //     meshes.add(CircularSegment::new(50.0,
    // 1.25)),     meshes.add(Ellipse::new(25.0,
    // 50.0)),     meshes.add(Annulus::new(25.0,
    // 50.0)),     meshes.add(Capsule2d::new(25.0,
    // 50.0)),     meshes.add(Rhombus::new(75.0,
    // 100.0)),     meshes.add(Rectangle::new(50.
    // 0, 100.0)),     meshes.
    // add(RegularPolygon::new(50.0, 6)),
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
    let shapes = [
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(Rectangle::new(1.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(Rectangle::new(100.0, 100.0)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(
            360. * i as f32 / num_shapes as f32,
            0.95,
            0.7,
        );

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to
                // +X_EXTENT/2.
                -X_EXTENT / 2.
                    + i as f32 / (num_shapes - 1) as f32
                        * X_EXTENT,
                Y_EXTENT / 2.,
                0.0,
            ),
            Obstacle,
        ));
    }

    // Rectangle::new(1200.0, 800.0)
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(600., 400.),
            Vec2::new(-600.0, 400.0),
        ))),
        MeshMaterial2d(
            materials.add(Color::from(
                GREEN_400.with_alpha(0.1),
            )),
        ),
        Transform::default(),
        Obstacle,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(600., -400.),
            Vec2::new(600.0, 400.0),
        ))),
        MeshMaterial2d(
            materials.add(Color::from(
                GREEN_400.with_alpha(0.1),
            )),
        ),
        Transform::default(),
        Obstacle,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(-600., -400.),
            Vec2::new(600.0, -400.0),
        ))),
        MeshMaterial2d(
            materials.add(Color::from(
                GREEN_400.with_alpha(0.1),
            )),
        ),
        Transform::default(),
        Obstacle,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(-600., 400.),
            Vec2::new(-600.0, -400.0),
        ))),
        MeshMaterial2d(
            materials.add(Color::from(
                GREEN_400.with_alpha(0.1),
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
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::from(SKY_400))),
        Transform::from_xyz(0., -40., 1.),
        Player,
        children!((
            Mesh2d(vismesh),
            MeshMaterial2d(materials.add(Color::from(
                YELLOW_400.with_alpha(0.5),
            )),),
            Transform::from_xyz(0., 0., 10.),
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
