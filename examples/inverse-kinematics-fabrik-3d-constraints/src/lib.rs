use std::f32::consts::FRAC_PI_8;

use bevy::platform::collections::HashSet;
use bevy::{color::palettes::tailwind::*, platform::collections::HashMap, prelude::*};
use itertools::Itertools;
use petgraph::{
    prelude::DiGraphMap,
    visit::{Dfs, DfsPostOrder, Walker},
};

pub struct FabrikPlugin;
impl Plugin for FabrikPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DottedGizmos>()
            .add_systems(Startup, config_gizmos)
            .add_systems(
                PostUpdate,
                process_inverse_kinematics.after(TransformSystems::Propagate),
            );
    }
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DottedGizmos;

fn config_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DottedGizmos>();
    config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 5.,
        line_scale: 10.,
    };
}

/// A tip of the IK chain.
///
/// Iterate from this entity up the ancestor chain to
/// find the root IK entity.
#[derive(Debug, Component)]
pub struct InverseKinematicEndEffector {
    /// How many bones are involved in the IK chain?
    ///
    /// This is "how many links between joints" there are
    pub affected_bone_count: u32,
    /// How close does the end_effector need to be to the target for
    /// it to be "a success" which means we can stop
    ///
    /// A good 2d tolerance is 1.0 in the default camera view
    pub tolerance: f32,
    /// The place this end effector "wants to be"
    pub target: Vec3,
}

#[derive(Debug, Component)]
pub struct Constraint;

/// The primary system that checks for ik chains that should be processed,
/// then does some setup before kicking off FABRIK
pub fn process_inverse_kinematics(
    ik_end_effectors: Query<(Entity, &InverseKinematicEndEffector, &GlobalTransform)>,
    parents: Query<&ChildOf>,
    mut gizmos: Gizmos,
    mut dotted_gizmos: Gizmos<DottedGizmos>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    mut processed_end_effectors: Local<HashSet<Entity>>,
    // every ik graph in one petgraph
    mut all_ik_graphs: Local<DiGraphMap<Entity, f32>>,
    // root nodes of ik graphs
    mut roots: Local<HashSet<Entity>>,
    constraints: Query<(), With<Constraint>>,
) {
    // end_effectors we haven't processed yet
    let new_effectors = ik_end_effectors
        .iter()
        .filter(|(entity, _, _)| !processed_end_effectors.contains(entity))
        .map(|(entity, _, _)| entity)
        .collect::<Vec<Entity>>();

    let mut has_new_roots = false;
    for (end_effector_entity, end_effector, _end_effector_global_transform) in ik_end_effectors
        .iter()
        .filter(|(entity, _, _)| new_effectors.contains(entity))
    {
        has_new_roots = true;
        processed_end_effectors.insert(end_effector_entity);

        let mut it = std::iter::once(end_effector_entity)
            .chain(
                parents
                    .iter_ancestors(end_effector_entity)
                    .take(end_effector.affected_bone_count as usize),
            )
            .peekable();
        while let Some(entity) = it.next() {
            all_ik_graphs.add_node(entity);
            if let Some(parent_entity) = it.peek() {
                all_ik_graphs.add_node(*parent_entity);

                let entity_global_position = global_transforms
                    .get(entity)
                    .expect("Bone Joints must have GlobalTransforms")
                    .translation();

                let parent_entity_global_position = global_transforms
                    .get(*parent_entity)
                    .expect("Bone Joints must have GlobalTransforms")
                    .translation();

                // edge is bone_length, which is distance between two nodes in the
                // starting position.
                all_ik_graphs.add_edge(
                    *parent_entity,
                    entity,
                    entity_global_position.distance(parent_entity_global_position),
                );
            }
        }
    }
    if has_new_roots {
        // if there's new end_effectors,
        // get roots (which have no incoming edge) and update
        // the roots Local
        for new_root in all_ik_graphs.nodes().filter_map(|node| {
            let incoming_edges = all_ik_graphs
                .edges_directed(node, petgraph::Direction::Incoming)
                .count();
            (incoming_edges == 0).then_some(node)
        }) {
            roots.insert(new_root);
        }
    }
    // print dot for viz
    // println!(
    //     "{:?}",
    //     petgraph::dot::Dot::with_config(&*all_ik_graphs, &[])
    // );

    // build `positions` for everything
    let mut positions: HashMap<Entity, Vec3> = all_ik_graphs
        .nodes()
        .map(|entity| {
            (
                entity,
                global_transforms
                    .get(entity)
                    .expect("Bone Joints must have GlobalTransforms")
                    .translation(),
            )
        })
        .collect();

    // iterate graphs
    for root in roots.iter() {
        let original_root_position = *positions.get(root).unwrap();

        let end_effectors_for_root: Vec<Entity> = all_ik_graphs
            .nodes()
            .filter_map(|node| {
                let outgoing_edges = all_ik_graphs
                    .edges_directed(node, petgraph::Direction::Outgoing)
                    .count();
                (outgoing_edges == 0).then_some(node)
            })
            .collect();

        // are all the end effectors within their tolerances for
        // their desired target?
        let mut all_end_effectors_are_at_targets = end_effectors_for_root
            .iter()
            .map(|entity| ik_end_effectors.get(*entity).unwrap())
            .all(
                |(end_effector_entity, end_effector, _end_effector_global_transform)| {
                    positions
                        .get(&end_effector_entity)
                        .unwrap()
                        .distance(end_effector.target)
                        < end_effector.tolerance
                },
            );

        // loop for forward/backward passes
        //
        // This is "The Algorithm"
        //
        // keeps track of iteration count because
        // if the bones can't physically reach the point
        // the loop will never finish
        //
        // 10 iterations is an entirely arbitrary number
        // of maximum iterations.
        let mut iterations = 0;
        while !all_end_effectors_are_at_targets && iterations < 3 {
            iterations += 1;

            // This is the Forward Pass, done as a graph traversal!
            for node in DfsPostOrder::new(&*all_ik_graphs, *root).iter(&*all_ik_graphs) {
                // "neighbors" in a directed graph where edges
                // are pointed towards children is the same as "children".
                let num_children = all_ik_graphs.neighbors(node).count();
                // info!(?node, ?num_children, "DfsPostOrder");
                match num_children {
                    0 => {
                        // end effector, we need the target
                        let target = ik_end_effectors
                        .get(node)
                        .expect("getting an end_effector with a Node that has 0 children should be an EndEffector")
                        .1.target;
                        // info!(?node, ?target, "setting position to target");
                        *positions
                            .get_mut(&node)
                            .expect("preconstructed HashMap should have all values") = target;
                        continue;
                    }
                    n => {
                        // we could special-case 1, but N/1 is N anyway.
                        // and calculating the centroid is the average.
                        let current_node_position = positions.get(&node).unwrap();
                        // for sub-bases, this has to be calculated N times
                        // and averaged to find the centroid.
                        // each child "wants" the sub-base to be in a different
                        // position, and we use the centroid to negotiate that.
                        let mut total_points = Vec3::ZERO;

                        for child in all_ik_graphs.neighbors(node) {
                            let child_pos = positions.get(&child).unwrap();
                            let bone_length = all_ik_graphs.edge_weight(node, child).unwrap();

                            // info!(?child_pos, ?current_node_position);
                            let vector = child_pos - current_node_position;

                            if constraints.get(child).is_ok()
                                // TK: only apply constraints to children with one child for now
                                && let Some(grandchild) = all_ik_graphs.neighbors(child).next()
                                && let Some(grandchild_position) = positions.get(&grandchild)
                            {
                                // #3.1: find the line equation L1
                                // grandchild is `Pi+1`, so this vector is `L1` from the paper,
                                // figure 4
                                let grandchild_vector = grandchild_position - child_pos;

                                gizmos.circle(
                                    Isometry3d::from_translation(*child_pos),
                                    1.,
                                    RED_400,
                                );
                                gizmos.circle(
                                    Isometry3d::from_translation(*current_node_position),
                                    1.,
                                    INDIGO_400,
                                );
                                // Temp cone constraint
                                // gizmos.line(*child_pos, *child_pos - grandchild_vector, BLUE_400);

                                // 3.2: find the projection O of the target t on line L1
                                let o = vector.project_onto(grandchild_vector);
                                gizmos.line(*child_pos, *child_pos - o, RED_400);

                                // 3.3: find the distance between O and the joint
                                let s = o.length();
                                // 3.4: Map the target (rotate and translate) in such a way that O
                                //   is now located at the axis origin and oriented according to
                                //   the x and y-axis ) Now it is a 2D simplified problem
                                // 3.5/3.6: cone so all sectors are the same

                                // gizmos.line(*child_pos, *child_pos - grandchild_vector, SLATE_50);

                                let qj = s * FRAC_PI_8.tan();

                                // 3.8: Check whether the target is within the conic section or not
                                let t_position = child_pos - o;
                                let is_inside_circle = t_position.length() < qj;
                                if !is_inside_circle {
                                    info!("move point");

                                    //
                                    let new_pos = (t_position).normalize() * qj;
                                    let new_pos2 = (new_pos + o).normalize() * bone_length;
                                    gizmos.circle(
                                        Isometry3d::from_translation(new_pos2),
                                        0.2,
                                        RED_400,
                                    );
                                    total_points += new_pos;
                                }
                                // dbg!(
                                //     current_node_position,
                                //     grandchild_vector,
                                //     qj,
                                //     vector.length()
                                // );
                                // gizmos.circle(Isometry3d::from_translation(point), 0.2, SLATE_50);
                                // end temp cone constraint
                            } else {
                                total_points += child_pos - vector.normalize() * bone_length;
                            }
                        }

                        // info!(?total_points, n, total = ?total_points / n as f32);
                        *positions
                            .get_mut(&node)
                            .expect("preconstructed HashMap should have all values") =
                            total_points / n as f32;

                        // // test cone restriction
                        // gizmos
                        //     .primitive_3d(
                        //         &Cone {
                        //             radius: 0.5,
                        //             height: 2.,
                        //         },
                        //         Isometry3d::new(
                        //             (total_points / n as f32),
                        //             Quat::from_axis_angle(axis, angle),
                        //         ),
                        //         BLUE_400,
                        //     )
                        //     .resolution(20);
                    }
                }
            }

            for (_, transform) in &positions {
                gizmos.axes(Transform::from_translation(*transform), 1.);
            }

            // This is the Backward Pass, done as a graph traversal!
            // TODO: Is EdgesReversed useful here? instead of edged_directed
            for node in Dfs::new(&*all_ik_graphs, *root).iter(&*all_ik_graphs) {
                // "neighbors" in a directed graph where edges
                // are pointed towards children is the same as "children".
                match all_ik_graphs
                    .edges_directed(node, petgraph::Direction::Incoming)
                    .next()
                {
                    None => {
                        // This is the root node of the IK graph!
                        let target = original_root_position;
                        *positions
                            .get_mut(&node)
                            .expect("preconstructed HashMap should have all values") = target;
                        continue;
                    }
                    Some(incoming_edge) => {
                        let previous_node_position = positions.get(&incoming_edge.0).unwrap();
                        let current_node_position = positions.get(&incoming_edge.1).unwrap();

                        let bone_length = all_ik_graphs
                            .edge_weight(incoming_edge.0, incoming_edge.1)
                            .unwrap();

                        let vector = previous_node_position - current_node_position;
                        *positions
                            .get_mut(&incoming_edge.1)
                            .expect("preconstructed HashMap should have all values") =
                            previous_node_position - vector.normalize() * bone_length;
                    }
                }
            }

            // "diff", but for many end_effectors
            // *same code* as when we test it before the loop
            all_end_effectors_are_at_targets = end_effectors_for_root
                .iter()
                .map(|entity| ik_end_effectors.get(*entity).unwrap())
                .all(
                    |(end_effector_entity, end_effector, _end_effector_global_transform)| {
                        positions
                            .get(&end_effector_entity)
                            .unwrap()
                            .distance(end_effector.target)
                            < end_effector.tolerance
                    },
                );
        }

        // gizmos for calculated node positions
        // for (entity, position) in positions.iter() {
        //     gizmos.sphere(*position, 0.2, GREEN_400);
        // }
        for (start, end, _distance) in all_ik_graphs.all_edges() {
            // gizmos.arrow(
            //     *positions.get(&start).unwrap(),
            //     *positions.get(&end).unwrap(),
            //     GREEN_400,
            // );
        }

        // set the Transform hierarchy for the bones using the calculated
        // `positions` as source data
        //
        // Also build up a temporary cache of "global transforms" so that
        // we can access them while iterating
        // These Transforms represent the entities as if they were
        // lone entities, not part of any hierarchy.
        let mut new_global_transforms = HashMap::<Entity, Transform>::new();

        for node in Dfs::new(&*all_ik_graphs, *root).iter(&*all_ik_graphs) {
            // Check to see if there's a "parent" node
            let Some(incoming) = all_ik_graphs
                .edges_directed(node, petgraph::Direction::Incoming)
                .next()
            else {
                // if there isn't a parent node,
                // rotate in direction of next child
                let mut it = all_ik_graphs.neighbors(node);
                let child = it.next().expect("root nodes must have at least one child");

                // TODO: maybe debug_assert this check?
                if it.next().is_some() {
                    panic!("we don't handle the case where a root node has more than one child")
                }

                let new_transform = current_transform_with_rotation(
                    *positions.get(&node).unwrap(),
                    *positions.get(&child).unwrap(),
                );

                // if there's no parent, then we're
                // dealing with the root bone, which
                // doesn't move so we can set rotation
                // and parent_global_transform, then
                // continue

                let mut transform = transforms.get_mut(node).unwrap();
                transform.rotation = new_transform.rotation;
                new_global_transforms.insert(node, new_transform);
                continue;
            };

            // "neighbors" in a directed graph where edges
            // are pointed towards children is the same as "children".
            let transform_to_insert = match all_ik_graphs.neighbors(node).count() {
                1 => {
                    // rotate in direction of next child
                    let child = all_ik_graphs
                        .neighbors(node)
                        .next()
                        .expect("we already confirmed this node has a child.");

                    let new_transform = current_transform_with_rotation(
                        *positions.get(&node).unwrap(),
                        *positions.get(&child).unwrap(),
                    );

                    new_global_transforms.insert(node, new_transform);

                    new_transform
                }
                0 => {
                    // if there is no `child` node, we're
                    // dealing with the tail, which does
                    // all the same calculations, but uses
                    // the parent joint's rotation value
                    let new_transform =
                        Transform::from_translation(*positions.get(&incoming.1).unwrap())
                            .with_rotation(
                                new_global_transforms.get(&incoming.0).unwrap().rotation,
                            );
                    new_global_transforms.insert(incoming.1, new_transform);
                    new_transform
                }
                _ => {
                    // more than 1 child means we are at a sub-base; (since we haven't handled "loops" here)
                    //
                    // for now, rotate sub-bases according to their parent
                    // similar to how we handle end nodes.
                    //
                    // This means we don't handle a root with an immediate
                    // split well... but I'm not really sure what the
                    // rotation should be if there's a root node with say:
                    // an 8 way immediate split.
                    let new_transform =
                        Transform::from_translation(*positions.get(&incoming.1).unwrap())
                            .with_rotation(
                                new_global_transforms.get(&incoming.0).unwrap().rotation,
                            );
                    new_global_transforms.insert(incoming.1, new_transform);

                    new_transform
                }
            };

            // use the "global" Transforms to calculate
            // the proper rotations using affine inverse
            let parent = new_global_transforms.get(&incoming.0).unwrap();
            let (scale, rotation, translation) = (parent.compute_affine().inverse()
                * transform_to_insert.compute_affine())
            .to_scale_rotation_translation();

            let mut transform = transforms.get_mut(incoming.1).unwrap();
            transform.scale = scale;
            transform.rotation = rotation;
            transform.translation = translation;
        }
    }
}

/// Takes the "current" position and the "next" position and
/// return a `Transform` that is location at `current` and rotated
/// to point `Vec3::Z` towards `next`.
fn current_transform_with_rotation(current: Vec3, next: Vec3) -> Transform {
    Transform::from_translation(current).with_rotation({
        let angle = next - current;
        Quat::from_rotation_arc(Vec3::Z, angle.normalize())
    })
}
