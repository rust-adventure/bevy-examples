use bevy::{color::palettes::tailwind::GREEN_400, mesh::VertexAttributeValues, prelude::*};
use itertools::Itertools;

#[derive(Default)]
pub struct VisibilityMeshPlugin;

impl Plugin for VisibilityMeshPlugin {
    fn build(&self, app: &mut App) {
        // Running directly after propagation ensures that the mesh is up to date at the beginning of each frame
        // which in turn allows placing the visibility mesh as a child of an entity
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

    // get all positions of vertices in global world space
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

    let mut triangle_bases: Vec<[f32; 3]> = vec![];
    // todo: maybe track the positions?
    let mut skip_to: Option<(Mesh2d, usize)> = None;
    let mut current_vertex: Option<Vec2> = None;
    info!("\n\n");
    let mut initial_hue = 0.;

    // }
    let mut i = 0;
    while let Some(VisibilityData {
        mesh2d,
        index,
        angle_from_player,
        global_position,
    }) = vertex_iterator.next()
    {
        if ray_index.0 == i {
        gizmos.line_2d(
            player_position,
            global_position.xy(),
            GREEN_400,
        );
        }
        i += 1;

        // info!(
        //     ?global_position,
        //     ?angle_from_player,
        //     // handle_id=?skip_to.as_ref().map(|h| h.0.id()),
        //     skip_index=?skip_to.as_ref().map(|h| h.1),
        //     vertex_index=index,
        // );
        match skip_to {
            Some((ref skip_mesh2d, ref skip_index))
                if skip_mesh2d == mesh2d
                    && skip_index == index =>
            {
                info!(?global_position, "skipping");
                // gizmos.line_2d(
                //     player_position,
                //     global_position.xy(),
                //     // Color::Oklcha(Oklcha {
                //     //     lightness: 1.,
                //     //     chroma: 1.,
                //     //     hue: initial_hue,
                //     //     alpha: 1.,
                //     // }),
                //     RED_400,
                // );

                // player position is "center"
                // theoretically we could use indices and only add this vertex once
                // for now we use a more naive "every three vertex positions are a triangle" approach
                triangle_bases.push([0., 0., 0.]);
                triangle_bases.push([
                    global_position.x - player_position.x,
                    global_position.y - player_position.y,
                    0.,
                ]);
                triangle_bases.push([
                    current_vertex.unwrap().x
                        - player_position.x,
                    current_vertex.unwrap().y
                        - player_position.y,
                    0.,
                ]);

                // write global positions if we switch back to global vismeshes
                // triangle_bases.push(player_position);
                // triangle_bases
                //     .push(global_position.clone());
                // triangle_bases
                //     .push(current_vertex.unwrap());

                // todo: not just next index
                // if angle of next index is less than current, its "behind"
                // also works if find() doesn't find it.
                let mesh = meshes.get(&mesh2d.0).ok_or(
                    "mesh2d unavailable in meshes database",
                )?;

                let VertexAttributeValues::Float32x3(positions) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION).ok_or("Mesh2d mesh asset doesn't have positions. WEIRD")? else {
                        return Err(BevyError::from("Mesh2d Vertices are not in format".to_string()));
                };
                // info!(len=?positions.len(), skip_in);
                // if next vertex position is out of the list, cycle around to 0
                let new_skip_index: usize = positions
                    .iter()
                    .enumerate()
                    .cycle()
                    .nth(skip_index + 1)
                    .unwrap()
                    .0;

                // calc angle of position from player
                let next_vertex = all_positions.iter().flatten().find(|visibility_data| {
                    &visibility_data.mesh2d == mesh2d && new_skip_index == visibility_data.index
                }).expect("looping around vertices should always find a vertex");

                // if current angle > next angle, then next vertex is "behind"
                if angle_from_player
                    < &next_vertex.angle_from_player
                {
                    if let Some(next) =
                        vertex_iterator.peek()
                    {
                        info!("next'ing the gap");
                        skip_to = Some((
                            next.mesh2d.clone(),
                            next.index,
                        ));
                        current_vertex =
                            Some(global_position.clone());
                    }
                } else {
                    skip_to = Some((
                        skip_mesh2d.clone(),
                        new_skip_index,
                    ));
                    current_vertex =
                        Some(global_position.clone());
                }

                continue;
            }
            Some(_) => {
                info!("continue!");
                // abuse that triangle face winding is counter-clockwise
                // to skip forward in iterator.
                continue;
            }
            None => {
                info!(?global_position, "starting at");
                current_vertex =
                    Some(global_position.clone());
                skip_to =
                    Some((mesh2d.clone(), *index + 1));
                // gizmos.line_2d(
                //     player_position,
                //     global_position.xy(),
                //     // Color::Oklcha(Oklcha {
                //     //     lightness: 1.,
                //     //     chroma: 1.,
                //     //     hue: initial_hue,
                //     //     alpha: 1.,
                //     // }),
                //     GREEN_400,
                // );
                initial_hue += 30.;
            }
        }
    }

    // dbg!(triangle_bases);

    // for (x,y,z) in  triangle_bases.iter().tuple_windows() {
    //        gizmos.line_2d(
    //                     *x,*y,
    //                     GREEN_400,
    //                 );
    //             }
    let vismesh =
        meshes.get_mut(&visibility_mesh.0).unwrap();
    vismesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        triangle_bases,
    );

    Ok(())
}
