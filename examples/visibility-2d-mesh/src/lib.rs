use bevy::{
    color::palettes::tailwind::*,
    math::FloatOrd,
    mesh::{
        Indices, PrimitiveTopology, VertexAttributeValues,
    },
    prelude::*,
};
use either::Either;
use itertools::Itertools;

pub mod post_processing;

#[derive(Default)]
pub struct VisibilityMeshPlugin;

impl Plugin for VisibilityMeshPlugin {
    fn build(&self, app: &mut App) {
        // Running directly after propagation ensures that
        // the mesh is up to date at the beginning of each
        // frame which in turn allows placing the
        // visibility mesh as a child of an entity
        app.add_systems(
            PostUpdate,
            calculate_visibility
                .after(TransformSystems::Propagate),
        );
    }
}

#[derive(Resource)]
pub struct RayIndex(pub u32);

pub fn set_ray_index(
    input: Res<ButtonInput<KeyCode>>,
    mut ray_index: ResMut<RayIndex>,
) {
    if input.just_pressed(KeyCode::ArrowLeft) {
        ray_index.0 = ray_index.0.saturating_sub(1);
    };
    if input.just_pressed(KeyCode::ArrowRight) {
        ray_index.0 += 1;
    };
}

#[derive(Resource)]
pub struct VisibilityMesh(pub Handle<Mesh>);

#[derive(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[type_path="api"]
pub struct Obstacle;

#[derive(Debug)]
struct VisibilityData {
    mesh2d: Mesh2d,
    index: usize,
    angle_from_player: f32,
    global_position: Vec2,
    primitive_topology: PrimitiveTopology,
}

fn calculate_visibility(
    query: Query<
        (&Mesh2d, &GlobalTransform),
        With<Obstacle>,
    >,
    player: Single<&GlobalTransform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: Gizmos,
    ray_index: Res<RayIndex>,
    visibility_mesh: Res<VisibilityMesh>,
) -> Result {
    let player_position = player.translation().xy();

    // get all positions of vertices in global world
    // space
    let all_positions = query.iter().filter(|(mesh2d, _)| {
        // exclude visibility mesh from processing
        mesh2d.0 != visibility_mesh.0
    }).map(|(mesh2d, global_transform)| {
        let mesh = meshes.get(&mesh2d.0).ok_or(
            "mesh2d unavailable in meshes database",
        )?;

        let VertexAttributeValues::Float32x3(positions) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).ok_or("Mesh2d mesh asset doesn't have positions. WEIRD")? else {
                return Err(BevyError::from("Mesh2d Vertices are not in format".to_string()));
            };

            // let Some(Indices::U32(indices)) = mesh.indices() else {
            //     panic!("non-u32 mesh indices found");
            // };
            // info!(topo=?mesh.primitive_topology(), ?indices, ?positions, "mesh debug");
            // for triangle in indices.chunks(2) {

                
            //     let a = Vec2::from_slice(&positions[triangle[0] as usize]) + global_transform.translation().xy();
            //     let b = Vec2::from_slice(&positions[triangle[1] as usize])+ global_transform.translation().xy();

            //     gizmos.linestrip_2d(
            //       [  a, b],
            //         RED_400,
            //     );
            //     // gizmos.linestrip_2d(
            //     //   [  b, c],
            //     //     RED_400,
            //     // );
            //     // gizmos.linestrip_2d(
            //     //   [  c, a],
            //     //     RED_400,
            //     // );
            // }

        let positions2 = positions.iter().enumerate().map(|(index,position)| {
            let global_position = Vec2::new(
                    position[0] + global_transform.translation().x,
                    position[1] + global_transform.translation().y
                );
            VisibilityData {
                mesh2d: mesh2d.clone(),
                index: index,
                angle_from_player: (global_position - player_position).to_angle(),
                global_position,
                primitive_topology: mesh.primitive_topology()
            }
            
        }).collect::<Vec<VisibilityData>>();

        Ok(positions2)

    }).collect::<Result<Vec<Vec<VisibilityData>>, BevyError>>()?;

    //     for mesh in all_positions.iter() {
    //        dbg!(mesh.len());
    // for chunk in        &mesh.indices()
    //                     .inspect(|v| {dbg!(v);})

    //                     {
    //                        for (start, end) in
    // chunk.tuple_windows().map(|(a, b)| {
    //                         (
    //                             a.global_position,
    //                             b.global_position,
    //                         )
    //                     }) {
    // gizmos.linestrip_2d(
    //                   [  start, end],
    //                     RED_400,
    //                 );
    //                        }
    //                     }
    //                     {

    //             }
    //             }
    // return Ok(());

    let mut vertex_iterator =
        all_positions.iter().flatten().sorted_by(|a, b| {
            b.angle_from_player
                .total_cmp(&a.angle_from_player)
        });

    let mut all_rays_for_vertex: Vec<(
        &VisibilityData,
        Vec<Vec2>,
    )> = vertex_iterator
        // .take(2)
        .map(|vertex_visibility_data| {
            raycast(
                &player_position,
                &all_positions,
                &vertex_visibility_data,
                &mut gizmos,
            )
        })
        .collect();
    let mut hue = 100.;
    let mut triangles: Vec<[f32; 3]> = vec![];
    for ((vertex_a, ray_hits_a), (vertex_b, ray_hits_b)) in
        all_rays_for_vertex.iter().circular_tuple_windows()
    {
        let Some((hit_a, hit_b)) = ray_hits_a
            .iter()
            .cartesian_product(ray_hits_b.iter())
            .sorted_by(|start_end_a, start_end_b| {
                let a_distance = start_end_a
                    .0
                    .distance(player_position)
                    .min(
                        start_end_a
                            .0
                            .distance(player_position),
                    );
                let b_distance = start_end_b
                    .0
                    .distance(player_position)
                    .min(
                        start_end_b
                            .0
                            .distance(player_position),
                    );
                a_distance.total_cmp(&b_distance)
            })
            .find_map(|hits| {
                all_positions
                    .iter()
                    // for each mesh, make all walls that
                    // are in
                    // that mesh by connecting two vertices
                    // on the perimeter
                    .flat_map(|mesh_vertices| {
                        // TODO: TriangleList vs LineList
                        // require
                        // circular_tuple_windows vs
                        // tuple_windows
                        mesh_vertices
                            .iter()
                            .circular_tuple_windows()
                            .map(|(a, b)| {
                                (
                                    a.global_position,
                                    b.global_position,
                                )
                            })
                    })
                    .find_map(|(start, end)| {
                        let margin_bias = 1.;
                        let x_min = start.x.min(end.x)
                            - margin_bias;
                        let y_min = start.y.min(end.y)
                            - margin_bias;
                        let x_max = start.x.max(end.x)
                            + margin_bias;
                        let y_max = start.y.max(end.y)
                            + margin_bias;

                        // both points are on wall
                        (hits.0.x >= x_min
                            && hits.0.x <= x_max
                            && hits.0.y >= y_min
                            && hits.0.y <= y_max
                            && hits.1.x >= x_min
                            && hits.1.x <= x_max
                            && hits.1.y >= y_min
                            && hits.1.y <= y_max)
                            .then_some(hits)
                    })
            })
        else {
            continue;
        };
        // gizmos.linestrip_2d(
        //     [*hit_a, *hit_b],
        //     Color::lcha(1., 1., hue, 1.),
        // );
        hue += 20.;
        triangles.push([0.; 3]);
        triangles.push(
            (hit_b - player_position).extend(1.).to_array(),
        );
        triangles.push(
            (hit_a - player_position).extend(1.).to_array(),
        );
    }

    //  let mut current_vertex = None;
    //  let mut last_vertex = None;
    // assume hits are sorted
    // let triangles: Vec<[f32; 3]> = hits
    //     .iter()
    //     .circular_tuple_windows()
    //     .flat_map(|(hit_a, hit_b)| {
    //         [
    //             //
    // player_position.extend(1.).to_array(),
    //             [0.;3],
    //             (hit_b -
    // player_position).extend(1.).to_array(),
    //             (hit_a -
    // player_position).extend(1.).to_array(),
    //         ]
    //     })
    //     .collect();

    let vismesh =
        meshes.get_mut(&visibility_mesh.0).unwrap();
    vismesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        triangles,
    );

    Ok(())
}

//
fn intersect(
    line_1: (Vec2, Vec2),
    line_2: (Vec2, Vec2),
) -> Option<Vec2> {
    let line_1_3d =
        (line_1.0.extend(1.), line_1.1.extend(1.));
    let line_2_3d =
        (line_2.0.extend(1.), line_2.1.extend(1.));

    let line_1_cross = line_1_3d.0.cross(line_1_3d.1);
    let line_2_cross = line_2_3d.0.cross(line_2_3d.1);

    let cross_cross = line_1_cross.cross(line_2_cross);
    if cross_cross.z.abs() < f32::EPSILON {
        return None;
    }
    Some(Vec2::new(
        cross_cross.x / cross_cross.z,
        cross_cross.y / cross_cross.z,
    ))
}

fn raycast<'a>(
    player_position: &Vec2,
    all_positions: &[Vec<VisibilityData>],
    visibility_data: &'a VisibilityData,
    gizmos: &mut Gizmos,
) -> (&'a VisibilityData, Vec<Vec2>) {
    let mut ray_hits: Vec<Vec2> = all_positions.iter()
            // for each mesh, make all walls that are in
            // that mesh by connecting two vertices on
            // the perimeter
            .flat_map(|mesh_vertices| {
                match visibility_data.primitive_topology {
                    PrimitiveTopology::LineList => {
                     Either::Left(mesh_vertices
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| {
                            (
                                a.global_position,
                                b.global_position,
                            )
                        }))
                    },
                    PrimitiveTopology::TriangleList => {
                        Either::Right(mesh_vertices
                            .iter()
                            .circular_tuple_windows()
                            .map(|(a, b)| {
                                (
                                    a.global_position,
                                    b.global_position,
                                )
                            }))
                    },
                    _ => {
                        panic!("Unhandled PrimitiveTopology");
                   }
                }
                
            })
            // raycast against all walls from
            // player position, return hits.
            .filter_map(|wall|{
            let Some(intersection_point) = intersect(
                (
                    *player_position,
                    visibility_data.global_position,
                ),
                wall,
            ) else {
                // info!(?visibility_data, "intersect return None");
                return None;
            };

            // check if is in bounds of line segment
            let margin_bias = 1.;
            let x_min = wall.0.x.min(wall.1.x) - margin_bias;
            let y_min = wall.0.y.min(wall.1.y) - margin_bias;
            let x_max = wall.0.x.max(wall.1.x) + margin_bias;
            let y_max = wall.0.y.max(wall.1.y) + margin_bias;

            let player_to_vertex_gradient = (visibility_data
                .global_position
                - player_position)
                .normalize()
                .signum();
            let player_to_hit_gradient = (intersection_point
                - player_position)
                .normalize()
                .signum();

                
            if intersection_point.x >= x_min
                && intersection_point.x <= x_max
                && intersection_point.y >= y_min
                && intersection_point.y <= y_max
                // TODO: test horizontal/vertical lines?
                && player_to_vertex_gradient == player_to_hit_gradient
            {
                // gizmos.linestrip_2d(
                //   [  *player_position,
                //     intersection_point],
                //     GREEN_400,
                // );
                // gizmos.circle_2d(
                //     intersection_point,
                //     10.,
                //     RED_400,
                // );
                return Some(intersection_point);
            } else {
                return None;
            }
        }).collect();

    ray_hits.sort_by(|a, b| {
        a.distance(*player_position)
            .total_cmp(&b.distance(*player_position))
    });
    // if there are hits before the vertex, drop the
    // vertex and future hits
    if let Some(active_vertex_index) =
        ray_hits.iter().position(|hit| {
            // info!(
            //     distance = hit.distance(
            //         visibility_data.global_position
            //     )
            // );
            hit.distance(visibility_data.global_position)
                < 0.1
        })
        && active_vertex_index > 0
    // TODO: also drop if we hit other vertices
    // such as in the case of perfectly horizontal or
    // vertical lines
    {
        // for hit in
        // ray_hits.drain(active_vertex_index..) {
        //     gizmos.circle_2d(hit, 10.,
        // RED_400); }
        // info!(?ray_hits, "dropped");
    }

    // for hit in ray_hits.iter() {
    //     gizmos.linestrip_2d(
    //         [*player_position, *hit],
    //         GREEN_400,
    //     );
    //     gizmos.circle_2d(*hit, 10., GREEN_400);
    // }
    (visibility_data, ray_hits)
}
