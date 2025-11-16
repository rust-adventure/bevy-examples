use std::collections::VecDeque;

use bevy::platform::collections::HashSet;
use bevy::{color::palettes::tailwind::*, platform::collections::HashMap, prelude::*};
use itertools::Itertools;
use petgraph::{
    graph::DiGraph,
    prelude::DiGraphMap,
    visit::{Dfs, DfsPostOrder, ReversedEdges, Walker},
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
    // Bone Entity to length
    mut bone_lengths: Local<HashMap<Entity, f32>>,
    // cached_ik_chains: Local<Entity>
    root_query: Query<Entity, With<IkRoot>>,
    // entity_collections
    mut ik_root_collections: Local<HashMap<RootEntity, HashSet<Entity>>>,
    children: Query<&Children>,
    mut ik_graphs: Local<HashMap<RootEntity, DiGraph<Entity, f32>>>,
) {
    // TODO: this part makes assumptions about how many bones each EndEffector
    // is expected to handle; specifically: that it goes all the way to the
    // root entity of the whole ik graph
    for (end_effector_entity, end_effector, end_effector_global_transform) in
        ik_end_effectors.iter()
    {
        let mut discovered_entities = HashSet::<Entity>::new();
        let mut found_root = None;
        for entity in
            std::iter::once(end_effector_entity).chain(parents.iter_ancestors(end_effector_entity))
        {
            discovered_entities.insert(entity);
            found_root = root_query.get(entity).ok();
        }
        if let Some(root_entity) = found_root {
            // add entities to root_entity hashset
            let collection = ik_root_collections
                .entry(root_entity)
                .or_insert(HashSet::new());

            collection.extend(discovered_entities);
        } else {
            // find an ik chain with at least one entity
        }
    }

    // build graphs
    for (root, entities) in ik_root_collections.iter() {
        // build graph
        let mut fabrik_graph: DiGraphMap<Entity, f32> = DiGraphMap::new();
        let mut root_id = None;
        let mut positions = HashMap::<Entity, Vec3>::new();

        for entity in entities {
            positions.insert(
                *entity,
                global_transforms
                    .get(*entity)
                    .expect("Bone Joints must have GlobalTransforms")
                    .translation(),
            );

            let id = fabrik_graph.add_node(*entity);

            // we need the root id to iterate later
            if root == entity {
                info!(?id, ?entity, "root entity was added");
                root_id = Some(id);
            }
        }

        let original_root_position = *positions.get(root).unwrap();

        // make edges for entities
        for entity in entities {
            let Ok(kiddos) = children.get(*entity) else {
                continue;
            };

            let entity_global_position = positions
                .get(entity)
                .expect("Bone Joints must have GlobalTransforms");
            // filter to keep child entities that are actually bones
            for child_entity in kiddos.iter().filter(|entity| entities.contains(entity)) {
                let child_entity_global_position = positions
                    .get(&child_entity)
                    .expect("Bone Joints must have GlobalTransforms");

                // edge is bone_length, which is distance between two nodes in the
                // starting position.
                fabrik_graph.add_edge(
                    *entity,
                    child_entity,
                    entity_global_position.distance(*child_entity_global_position),
                );
            }
        }

        println!("{:?}", petgraph::dot::Dot::with_config(&fabrik_graph, &[]));

        // This is the Forward Pass, done as a graph traversal!
        for node in DfsPostOrder::new(&fabrik_graph, root_id.unwrap()).iter(&fabrik_graph) {
            // "neighbors" in a directed graph where edges
            // are pointed towards children is the same as "children".
            let num_children = fabrik_graph.neighbors(node).count();
            info!(?node, ?num_children, "DfsPostOrder");
            match num_children {
                0 => {
                    // end effector, we need the target
                    let target = ik_end_effectors
                        .get(node)
                        .expect("getting an end_effector with a Node that has 0 children should be an EndEffector")
                        .1.target;
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

                    for child in fabrik_graph.neighbors(node) {
                        let child_pos = positions.get(&child).unwrap();
                        let bone_length = fabrik_graph.edge_weight(node, child).unwrap();

                        let vector = child_pos - current_node_position;

                        total_points += child_pos - vector.normalize() * bone_length;
                    }

                    *positions
                        .get_mut(&node)
                        .expect("preconstructed HashMap should have all values") =
                        total_points / n as f32;
                }
            }
        }

        // This is the Backward Pass, done as a graph traversal!
        for node in Dfs::new(&fabrik_graph, root_id.unwrap()).iter(&fabrik_graph) {
            // "neighbors" in a directed graph where edges
            // are pointed towards children is the same as "children".
            let num_children = fabrik_graph.neighbors(node).count();
            info!(?node, ?num_children, "Dfs");
            let mut it = fabrik_graph.edges_directed(node, petgraph::Direction::Incoming);
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
                    let bone_length = fabrik_graph
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

        // .iter()
    }

    // every ik chain
    // for (root, entities) in ik_root_collections.iter() {
    //     // let mut current_positions = discovered_entities
    //     //     .iter()
    //     //     .map(|entity| {
    //     //         (
    //     //             entity,
    //     //             CurrentPosition {
    //     //                 position: global_transforms.get(entity).unwrap().translation(),
    //     //                 bone_length: todo!(),
    //     //                 entity,
    //     //             },
    //     //         )
    //     //     })
    //     //     .collect::<HashMap<Entity, CurrentPosition>>();
    //     // chains is a RootEntity, Vec<Chain> from that RootEntity
    //     // to the next sub-base
    //     let mut chains = HashMap::<Entity, Vec<Vec<Entity>>>::new();
    //     // "key" and "next node" are both required to pick a direction
    //     // given a node with multiple children
    //     let mut next_nodes: VecDeque<(Option<Entity>, Entity)> = VecDeque::from([(None, *root)]);

    //     loop {
    //         let Some(current_root) = next_nodes.pop_front() else {
    //             break;
    //         };

    //         let (chain, next_roots) = get_chain_and_new_roots(current_root, &children, entities);
    //         info!(?chain, ?next_roots);

    //         // place the next_roots in next_nodes for the next iteration
    //         if let Some(next_roots) = next_roots {
    //             next_nodes.extend(next_roots);
    //         }
    //         // insert the chain we calculated, keyed by the root node/base
    //         let chain_entry = chains
    //             .entry(current_root.0.unwrap_or(current_root.1))
    //             .or_insert(vec![]);
    //         chain_entry.push(chain);
    //     }
    //     info!(?chains);

    //     // bases and how many values they need to be valid?
    //     let bases: Vec<&Entity> = chains.keys().collect();

    //     // values calculated for sub-bases
    //     let forward_base_values: HashMap<Entity, Vec<Vec3>> = HashMap::new();

    //     // for each end_effector
    //     //   - iter_ancestors to a sub-node
    //     //   - store sub-node value
    //     // then check to see which sub-nodes are ready-to-process
    //     //   by checking to see if they have enough values from their
    //     //   children

    //     // build_subchains(*root, &children, &entities);
    // }

    // iterate over all ik bodies in the scene
    // using 'ik_bodies as a label in case we have to
    // abandon a specific ik root's processing
    // 'ik_bodies: for (end_effector_entity, end_effector, end_effector_global_transform) in
    //     ik_end_effectors.iter()
    // {
    //     // let Some(root_entity) = parents
    //     //     .iter_ancestors(end_effector_entity)
    //     //     .nth(end_effector.affected_bone_count as usize - 1)
    //     // else {
    //     //     // if no root entity, continue to another body
    //     //     warn!("no root!");
    //     //     continue 'ik_bodies;
    //     // };

    //     // // either fetch the `total_length` and `bone_lengths` from
    //     // // the Local cache, or insert it if this is our first time
    //     // // dealing with this entity's ik chain
    //     // let (total_length, bones) =
    //     //     // borrow here because we never want mutable access to bone lengths
    //     //     &bone_lengths.entry(end_effector_entity).or_insert_with(|| {
    //     //         info!("inserting bones");
    //     //         // no entry for this ik chain, calculate bone lengths
    //     //         // and cache them
    //     //         // this is implicitly storing the end_effector -> root_entity
    //     //         // ordering for bone lengths
    //     //         let bones = std::iter::once(end_effector_entity)
    //     //             .chain(
    //     //                 parents
    //     //                     .iter_ancestors(end_effector_entity)
    //     //                     .take(end_effector.affected_bone_count as usize),
    //     //             )
    //     //             .scan(None, |state, joint| {
    //     //                 let position = global_transforms.get(joint).unwrap().translation();

    //     //                 let Some(previous_joint) = state else {
    //     //                     *state = Some(position);
    //     //                     return Some(0.);
    //     //                 };

    //     //                 let length = previous_joint.distance(position);

    //     //                 *state = Some(position);

    //     //                 Some(length)
    //     //             })
    //     //             .collect::<Vec<f32>>();

    //     //         (bones.iter().sum::<f32>(), bones)
    //     //     });

    //     // gizmos.circle(
    //     //     global_transforms.get(root_entity).unwrap().translation(),
    //     //     *total_length,
    //     //     SLATE_400,
    //     // );

    //     // // We use this `Vec` to store the calculations
    //     // // we make that mutate the `GlobalPosition`s.
    //     // // After the loop ends, we take this `Vec` and
    //     // // use the values to update the `Transform`
    //     // // components
    //     // let mut current_positions: Vec<CurrentPosition> = std::iter::once(end_effector_entity)
    //     //     .chain(
    //     //         parents
    //     //             .iter_ancestors(end_effector_entity)
    //     //             .take(end_effector.affected_bone_count as usize),
    //     //     )
    //     //     .zip(bones)
    //     //     .map(|(entity, bone_length)| CurrentPosition {
    //     //         position: global_transforms.get(entity).unwrap().translation(),
    //     //         bone_length: *bone_length,
    //     //         entity,
    //     //     })
    //     //     .collect::<Vec<CurrentPosition>>();

    //     // // put root_entity at beginning
    //     // current_positions.reverse();

    //     // // if target isn't reachable, return
    //     // //
    //     // // if the `total_length` of the bones is less than
    //     // // the distance required to reach the mouse, then
    //     // // we can't make it to the target mouse location
    //     // let root_translation = global_transforms.get(root_entity).unwrap().translation();
    //     // if *total_length < root_translation.distance(end_effector.target) {
    //     //     // mouse is out of reach!
    //     //     // orient all bones in straight line to mouse
    //     //     // direction
    //     //     let target_direction = (end_effector.target - root_translation).normalize();

    //     //     // produce a new current_positions by setting
    //     //     // every bone joint to the edge of the previous
    //     //     // bone in the direction of the target, forming
    //     //     // a straight line.
    //     //     let current_positions: Vec<CurrentPosition> = current_positions
    //     //         .into_iter()
    //     //         .scan(None, |state, next| {
    //     //             let Some(p) = state else {
    //     //                 *state = Some(next);
    //     //                 return state.clone();
    //     //             };

    //     //             *state = Some(CurrentPosition {
    //     //                 position: p.position + target_direction * p.bone_length,
    //     //                 ..next
    //     //             });

    //     //             return state.clone();
    //     //         })
    //     //         .collect();

    //     //     set_transforms(&current_positions, &mut transforms);

    //     //     // continue processing other bodies
    //     //     continue 'ik_bodies;
    //     // }

    //     // // `diff` is "how far off is the end joint from
    //     // // the target?"
    //     // let mut diff = end_effector_global_transform
    //     //     .translation()
    //     //     .distance(end_effector.target);

    //     // // loop for forward/backward passes
    //     // //
    //     // // This is "The Algorithm"
    //     // //
    //     // // keeps track of iteration count because
    //     // // if the bones can't physically reach the point
    //     // // the loop will never finish
    //     // //
    //     // // 10 iterations is an entirely arbitrary number
    //     // // of maximum iterations.
    //     // let mut iterations = 0;
    //     // while diff > end_effector.tolerance && iterations < 10 {
    //     //     iterations += 1;
    //     //     // if a pass returns an error, something is
    //     //     // horribly wrong, but other bodies might still
    //     //     // be ok, so we don't panic, but do skip this
    //     //     // ik chain
    //     //     if forward_pass(&mut current_positions, &end_effector.target).is_err() {
    //     //         continue 'ik_bodies;
    //     //     };
    //     //     // constraint
    //     //     if backward_pass(&mut current_positions, &root_translation).is_err() {
    //     //         continue 'ik_bodies;
    //     //     };

    //     //     // end_effector_position.distance(target)
    //     //     diff = current_positions
    //     //         .last()
    //     //         .unwrap()
    //     //         .position
    //     //         .distance(end_effector.target);
    //     // }

    //     // // optional gizmos
    //     // for (a, b) in current_positions.iter().tuple_windows() {
    //     //     dotted_gizmos.arrow(a.position, b.position, PINK_400);
    //     // }

    //     // // set the Transform hierarchy for the bones using the current_positions
    //     // // as source data
    //     // set_transforms(&current_positions, &mut transforms);
    // }
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
