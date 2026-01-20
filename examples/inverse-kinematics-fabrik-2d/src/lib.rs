use bevy::{color::palettes::tailwind::*, prelude::*};
use itertools::Itertools;

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
    pub target: Vec2,
}

/// a bone_length. This is explicit, but could
/// be derived from the initial starting positions
/// instead
#[derive(Debug, Component, Clone)]
pub struct BoneLength(pub f32);

/// This is a useful struct for keeping some data we need
/// It represents the "current position" of one joint,
/// which can have a 0 or longer bone length.
#[derive(Debug, Clone)]
struct CurrentPosition {
    position: Vec2,
    bone_length: BoneLength,
    entity: Entity,
}

/// The primary system that checks for ik chains that should be processed,
/// then does some setup before kicking off FABRIK
pub fn process_inverse_kinematics(
    ik_end_effectors: Query<(Entity, &InverseKinematicEndEffector, &GlobalTransform)>,
    parents: Query<&ChildOf>,
    bone_lengths: Query<(&BoneLength, &GlobalTransform, Entity)>,
    mut gizmos: Gizmos,
    mut dotted_gizmos: Gizmos<DottedGizmos>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    // iterate over all ik bodies in the scene
    // using 'ik_bodies as a label in case we have to
    // abandon a specific ik root's processing
    'ik_bodies: for (end_effector_entity, end_effector, end_effector_global_transform) in
        ik_end_effectors.iter()
    {
        let Some(root_entity) = parents
            .iter_ancestors(end_effector_entity)
            .nth(end_effector.affected_bone_count as usize - 1)
        else {
            // if no root entity, continue to another body
            warn!("no root!");
            continue 'ik_bodies;
        };
        // dotted_gizmos.arrow_2d(
        //     root_transform.translation.xy(),
        //     target.0,
        //     PINK_400.with_alpha(0.4),
        // );

        // use `ChildOf` relationship to iter bones and
        // sum the length of all bones.
        // `iter_descendants` doesn't include the root
        // element, so we add the root bone length
        // if end_effector has length for some reason:
        // add it here
        let total_length = parents
            .iter_ancestors(end_effector_entity)
            .take(end_effector.affected_bone_count as usize)
            .filter_map(|entity| bone_lengths.get(entity).ok())
            .map(|bone| bone.0 .0)
            .sum::<f32>();

        // info!(?total_length);

        gizmos.circle_2d(
            global_transforms
                .get(root_entity)
                .unwrap()
                .translation()
                .xy(),
            total_length,
            SLATE_400,
        );

        // We use this `Vec` to store the calculations
        // we make that mutate the `GlobalPosition`s.
        // After the loop ends, we take this `Vec` and
        // use the values to update the `Transform`
        // components
        let mut current_positions: Vec<CurrentPosition> = std::iter::once(CurrentPosition {
            position: end_effector_global_transform.translation().xy(),
            bone_length: BoneLength(0.),
            entity: end_effector_entity,
        })
        .chain(
            parents
                .iter_ancestors(end_effector_entity)
                .take(end_effector.affected_bone_count as usize)
                .map(|entity| {
                    bone_lengths
                        .get(entity)
                        .map(|(bone, global, entity)| CurrentPosition {
                            position: global.translation().xy(),
                            bone_length: bone.clone(),
                            entity,
                        })
                        .unwrap()
                }),
        )
        .collect();
        // put root_entity at beginning
        current_positions.reverse();

        // if target isn't reachable, return
        //
        // if the `total_length` of the bones is less than
        // the distance required to reach the mouse, then
        // we can't make it to the target mouse location
        let root_translation = global_transforms
            .get(root_entity)
            .unwrap()
            .translation()
            .xy();
        if total_length < root_translation.distance(end_effector.target) {
            // mouse is out of reach!
            // orient all bones in straight line to mouse
            // direction
            let target_direction = (end_effector.target - root_translation).normalize();

            // produce a new current_positions by setting
            // every bone joint to the edge of the previous
            // bone in the direction of the target, forming
            // a straight line.
            let current_positions: Vec<CurrentPosition> = current_positions
                .into_iter()
                .scan(None, |state, next| {
                    let Some(p) = state else {
                        *state = Some(next);
                        return state.clone();
                    };

                    *state = Some(CurrentPosition {
                        position: p.position + target_direction * p.bone_length.0,
                        ..next
                    });

                    return state.clone();
                })
                .collect();

            set_transforms(&current_positions, &mut transforms);

            // continue processing other bodies
            continue 'ik_bodies;
        }

        // `diff` is "how far off is the end joint from
        // the target?"
        let mut diff = end_effector_global_transform
            .translation()
            .xy()
            .distance(end_effector.target);

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
        while diff > end_effector.tolerance && iterations < 10 {
            iterations += 1;
            // if a pass returns an error, something is
            // horribly wrong, but other bodies might still
            // be ok, so we don't panic, but do skip this
            // ik chain
            if forward_pass(&mut current_positions, &end_effector.target).is_err() {
                continue 'ik_bodies;
            };
            if backward_pass(&mut current_positions, &root_translation).is_err() {
                continue 'ik_bodies;
            };

            // end_effector_position.distance(target)
            diff = current_positions
                .last()
                .unwrap()
                .position
                .distance(end_effector.target);
        }

        // optional gizmos
        for (a, b) in current_positions.iter().tuple_windows() {
            dotted_gizmos.arrow_2d(a.position, b.position, PINK_400.with_alpha(0.4));
        }

        // set the Transform hierarchy for the bones using the current_positions
        // as source data
        set_transforms(&current_positions, &mut transforms);
    }
}

// #########################################
// #                                       #
// #  Paper calls this the "Forward Pass"  #
// #                                       #
// #########################################
//
// forward pass is an iteration from the
// end_effector bone, to the root bone
fn forward_pass(current_positions: &mut [CurrentPosition], target: &Vec2) -> Result<(), String> {
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
        current.position = previous.position - vector.normalize() * current.bone_length.0;
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
    root_translation: &Vec2,
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
        current.position = previous.position - vector.normalize() * previous.bone_length.0;
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
        let current_node = Transform::from_xyz(current.position.x, current.position.y, 0.)
            // if there is no `next` node, we're
            // dealing with the tail, which does
            // all the same calculations, but uses
            // the last joint's rotation value
            .with_rotation(match next {
                Some(_) => Quat::from_axis_angle(
                    Vec3::Z,
                    (next.unwrap().position - current.position).to_angle(),
                ),
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
