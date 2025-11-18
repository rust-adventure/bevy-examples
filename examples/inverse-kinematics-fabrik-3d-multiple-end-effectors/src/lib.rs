use std::collections::VecDeque;

use bevy::platform::collections::HashSet;
use bevy::{color::palettes::tailwind::*, platform::collections::HashMap, prelude::*};
use itertools::Itertools;
use petgraph::{
    graph::DiGraph,
    prelude::DiGraphMap,
    visit::{Dfs, DfsPostOrder, EdgeIndexable, ReversedEdges, Walker},
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

#[derive(Component)]
pub struct IkRoot;

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

/// This is a useful struct for keeping some data we need
/// It represents the "current position" of one joint,
/// which can have a 0 or longer bone length.
#[derive(Debug, Clone)]
struct CurrentPosition {
    position: Vec3,
    bone_length: f32,
    entity: Entity,
}

type TotalBoneLength = f32;
type BoneLength = f32;
type RootEntity = Entity;

// if there's a loop glhf
struct IkJointNode {
    entity: Entity,
    children: Vec<(Entity, BoneLength)>,
}

// ChildOf/Children
// if children *and* they are bones, then this is a sub-base

// type IkChain = Vec<Entity>
// type ChainOrder = Vec<IkChain>
// Vec<IkChain>

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
    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&*all_ik_graphs, &[])
    );

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
        while !all_end_effectors_are_at_targets && iterations < 10 {
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

                            total_points += child_pos - vector.normalize() * bone_length;
                        }

                        // info!(?total_points, n, total = ?total_points / n as f32);
                        *positions
                            .get_mut(&node)
                            .expect("preconstructed HashMap should have all values") =
                            total_points / n as f32;
                    }
                }
            }

            // This is the Backward Pass, done as a graph traversal!
            // TODO: Is EdgesReversed useful here? instead of edged_directed
            for node in Dfs::new(&*all_ik_graphs, *root).iter(&*all_ik_graphs) {
                // "neighbors" in a directed graph where edges
                // are pointed towards children is the same as "children".
                // let num_children = fabrik_graph.neighbors(node).count();
                // info!(?node, ?num_children, "Dfs");
                let mut it = all_ik_graphs.edges_directed(node, petgraph::Direction::Incoming);
                let incoming = it.next();
                match incoming {
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

                        // let child_pos = positions.get(&child).unwrap();
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

        // set the Transform hierarchy for the bones using the current_positions
        // as source data
        // set_transforms(&current_positions, &mut transforms);
        //
        // Also build up a temporary cache of "global transforms" so that
        // we can access them while iterating
        // These Transforms represent the entities as if they were
        // lone entities, not part of any hierarchy.
        let mut new_global_transforms = HashMap::<Entity, Transform>::new();
        // info!(?positions);
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

                let new_transform = Transform::from_translation(*positions.get(&child).unwrap())
                    // if there is no `next` node, we're
                    // dealing with the tail, which does
                    // all the same calculations, but uses
                    // the last joint's rotation value
                    .with_rotation({
                        let angle = positions.get(&child).unwrap() - positions.get(&node).unwrap();
                        Quat::from_rotation_arc(Vec3::Z, angle.normalize())
                    });
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
            let num_children = all_ik_graphs.neighbors(node).count();
            // info!(?node, ?num_children, "set_transform");
            let transform_to_insert = match num_children {
                1 => {
                    // rotate in direction of next child
                    let mut it = all_ik_graphs.neighbors(node);
                    let child = it
                        .next()
                        // technically would panic if one node were passed in, but that's
                        // an exceedingly weird configuration of EndEffector::affected_bone_count == 0.
                        .expect("we already confirmed this node has a child.");
                    let new_transform =
                        Transform::from_translation(*positions.get(&child).unwrap())
                            // if there is no `next` node, we're
                            // dealing with the tail, which does
                            // all the same calculations, but uses
                            // the last joint's rotation value
                            .with_rotation({
                                let angle =
                                    positions.get(&child).unwrap() - positions.get(&node).unwrap();
                                Quat::from_rotation_arc(Vec3::Z, angle.normalize())
                            });
                    new_global_transforms.insert(node, new_transform);

                    new_transform
                }
                0 => {
                    let new_transform =
                        Transform::from_translation(*positions.get(&incoming.1).unwrap())
                            // if there is no `next` node, we're
                            // dealing with the tail, which does
                            // all the same calculations, but uses
                            // the last joint's rotation value
                            .with_rotation(
                                new_global_transforms.get(&incoming.0).unwrap().rotation,
                            );
                    new_global_transforms.insert(incoming.1, new_transform);
                    new_transform
                }
                // more than 1 child means we are at a sub-base;
                // for now, rotate sub-bases according to their parent
                // similar to how we handle end nodes.
                // This means we don't handle a root with an immediate
                // split well... but I'm not really sure what the
                // rotation should be if there's a root node with say:
                // an 8 way immediate split.
                _ => {
                    let new_transform =
                        Transform::from_translation(*positions.get(&incoming.1).unwrap())
                            // if there is no `next` node, we're
                            // dealing with the tail, which does
                            // all the same calculations, but uses
                            // the last joint's rotation value
                            .with_rotation(
                                new_global_transforms.get(&incoming.0).unwrap().rotation,
                            );
                    new_global_transforms.insert(incoming.1, new_transform);

                    new_transform
                }
            };

            // info!(?new_global_transforms, ?incoming);
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

type Chain = Vec<Entity>;
type SubBase = Entity;
type SubBaseDirection = Entity;
type NewRootEntities = Vec<(Option<SubBase>, SubBaseDirection)>;
fn get_chain_and_new_roots(
    original_root: (Option<SubBase>, Entity),
    children: &Query<&Children>,
    entities: &HashSet<Entity>,
) -> (Chain, Option<NewRootEntities>) {
    let mut chain = original_root
        .0
        .map(|sub_base| vec![sub_base, original_root.1])
        .unwrap_or_else(|| vec![original_root.1]);

    let roots = loop {
        // no children is always end joint of full chain
        let Ok(kiddos) = children.get(*chain.last().unwrap()) else {
            break None;
        };

        // filter for entities that are actually bones
        let next_entities = kiddos
            .iter()
            .filter(|entity| entities.contains(entity))
            .collect::<Vec<Entity>>();

        if next_entities.is_empty() {
            break None;
        }

        if next_entities.len() == 1 {
            chain.push(next_entities[0]);
            continue;
        }

        break Some(
            next_entities
                .into_iter()
                .map(|next| (chain.last().cloned(), next))
                .collect(),
        );
    };

    return (chain, roots);
}
// fn build_subchains(
//     original_root: Entity,
//     children: &Query<&Children>,
//     entities: &HashSet<Entity>,
// ) -> Vec<Vec<Entity>> {
//     let mut chain = vec![original_root];

//     let next_es = loop {
//         let current_root = chain.iter().last().unwrap().clone();
//         // always end joint of full chain
//         let Ok(kiddos) = children.get(current_root) else {
//             break None;
//         };

//         // entities that are actually bones
//         let next_entities = kiddos
//             .iter()
//             .filter(|entity| entities.contains(entity))
//             .collect::<Vec<Entity>>();

//         if next_entities.is_empty() {
//             break None;
//         }

//         if next_entities.len() == 1 {
//             chain.push(next_entities[0]);
//             continue;
//         }

//         break Some(next_entities);
//     };
//     chains.last_mut().unwrap().push(chain);

//     // this is a sub-base.
//     // close the previous vec
//     // open a new vec by recursing
//     // next_entities.iter()

//     for child in next_es {
//         build_subchains(*root, &children, &entities);
//     }
// }
// #########################################
// #                                       #
// #  Paper calls this the "Forward Pass"  #
// #                                       #
// #########################################
//
// forward pass is an iteration from the
// end_effector bone, to the root bone
fn forward_pass(current_positions: &mut [CurrentPosition], target: &Vec3) -> Result<(), String> {
    if let Some(end_effector) = current_positions.last_mut() {
        end_effector.position = *target;
    } else {
        return Err("bones list must have a bone".to_string());
    }

    // options here are using `windows_mut` from
    // `lending_iterator` https://docs.rs/lending-iterator/latest/lending_iterator/#windows_mut
    // or using peekable.
    // We could also use indices, but I prefer
    // avoiding indices when possible
    let mut it = current_positions.iter_mut().rev().peekable();
    while let (Some(previous), Some(current)) = (it.next(), it.peek_mut()) {
        let vector = previous.position - current.position;
        current.position = previous.position - vector.normalize() * current.bone_length;
    }

    Ok(())
}

/// #########################################
/// #                                       #
/// # Paper calls this the "Backward Pass"  #
/// #                                       #
/// #########################################
///
/// backward pass is an iteration from the root to
/// the end_effector
fn backward_pass(
    current_positions: &mut [CurrentPosition],
    root_translation: &Vec3,
) -> Result<(), String> {
    if let Some(root) = current_positions.first_mut() {
        root.position = *root_translation;
    } else {
        return Err("bones list must have a bone".to_string());
    }

    // options here are using `windows_mut` from
    // `lending_iterator` https://docs.rs/lending-iterator/latest/lending_iterator/#windows_mut
    // or using peekable.
    // We could also use indices, but I prefer
    // avoiding indices when possible
    let mut it = current_positions.iter_mut().peekable();
    while let (Some(previous), Some(current)) = (it.next(), it.peek_mut()) {
        let vector = previous.position - current.position;
        current.position = previous.position - vector.normalize() * previous.bone_length;
    }
    Ok(())
}

// Take a list of positions and bone lengths,
// turning that into a Transform hierarchy with
// the proper rotations, etc.
fn set_transforms(current_positions: &[CurrentPosition], transforms: &mut Query<&mut Transform>) {
    // info!(?current_positions);
    // At this point we have all of the global positions
    // and the FABRIK calculation is over.
    // everything below this point is taking the global
    // positions and translating them into the
    // Transform hierarchy so we can apply them to the
    // actual Transforms
    let mut parent_global_transform: Option<Transform> = None;
    let mut it = current_positions.iter().peekable();
    while let (Some(current), next) = (it.next(), it.peek()) {
        let current_node =
            Transform::from_xyz(current.position.x, current.position.y, current.position.z)
                // if there is no `next` node, we're
                // dealing with the tail, which does
                // all the same calculations, but uses
                // the last joint's rotation value
                .with_rotation(match next {
                    Some(_) => {
                        let angle = next.unwrap().position - current.position;
                        Quat::from_rotation_arc(Vec3::Z, angle.normalize())
                    }
                    None => parent_global_transform.unwrap().rotation,
                });

        // if there's no parent, then we're
        // dealing with the root bone, which
        // doesn't move so we can set rotation
        // and parent_global_transform, then
        // continue
        let Some(parent) = parent_global_transform else {
            let mut transform = transforms.get_mut(current.entity).unwrap();
            transform.rotation = current_node.rotation;
            parent_global_transform = Some(current_node);
            continue;
        };

        // use the "global" Transforms to calculate
        // the proper rotations using affine inverse
        let (scale, rotation, translation) = (parent.compute_affine().inverse()
            * current_node.compute_affine())
        .to_scale_rotation_translation();

        let mut transform = transforms.get_mut(current.entity).unwrap();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;

        // store the values we calculated for future
        // processing
        parent_global_transform = Some(current_node);
    }
}
