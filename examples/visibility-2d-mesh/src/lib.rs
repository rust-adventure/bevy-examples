use bevy::{
    color::palettes::tailwind::*, math::FloatOrd,
    mesh::VertexAttributeValues, prelude::*,
};
use itertools::Itertools;

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

#[derive(Component)]
pub struct Obstacle;

#[derive(Debug)]
struct VisibilityData {
    mesh2d: Mesh2d,
    index: usize,
    angle_from_player: f32,
    global_position: Vec2,
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
            }
            
        }).collect::<Vec<VisibilityData>>();

        Ok(positions2)

    }).collect::<Result<Vec<Vec<VisibilityData>>, BevyError>>()?;

    let mut vertex_iterator = all_positions
        .iter()
        .flatten()
        .sorted_by(|a, b| {
            b.angle_from_player
                .total_cmp(&a.angle_from_player)
        })
        .peekable();

    let mut hits = vec![];
    for visibility_data in vertex_iterator {
        let closest_hit = 
            all_positions.iter()
            // for each mesh, make all walls that are in
            // that mesh by connecting two vertices on
            // the perimeter
            .flat_map(|mesh_vertices| {
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
            // raycast against all walls from
            // player position, return hits.
            .filter_map(|wall|{
            let Some(intersection_point) = intersect(
                (
                    player_position,
                    visibility_data.global_position,
                ),
                wall,
            ) else {
                info!(?visibility_data, "intersect return None");
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
                gizmos.linestrip_2d(
                  [  player_position,
                    intersection_point],
                    GREEN_400,
                );
                // gizmos.circle_2d(
                //     intersection_point,
                //     10.,
                //     RED_400,
                // );
                return Some(intersection_point);
            } else {
                return None;
            }
        })
        .min_by(|hit_a, hit_b| {
            hit_a.distance(player_position).total_cmp(&hit_b.distance(player_position))
        });

        hits.push(closest_hit.unwrap());
        gizmos.circle_2d(
            closest_hit.unwrap(),
            10.,
            RED_400,
        );
    }

    // assume hits are sorted
    let triangles: Vec<[f32; 3]> = hits
        .iter()
        .circular_tuple_windows()
        .flat_map(|(hit_a, hit_b)| {
            [
                // player_position.extend(1.).to_array(),
                [0.;3],
                (hit_b - player_position).extend(1.).to_array(),
                (hit_a - player_position).extend(1.).to_array(),
            ]
        })
        .collect();

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
