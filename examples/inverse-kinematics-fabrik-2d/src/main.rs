use bevy::{color::palettes::tailwind::*, prelude::*};
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .init_gizmo_group::<DottedGizmos>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (debug_transforms, update))
        .run();
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct DottedGizmos;

#[derive(Debug, Component)]
struct InverseKinematics;

#[derive(Debug, Component, Clone)]
struct BoneLength(f32);

// #[derive(Debug, Component, Clone)]
// struct BoneAngleInital(f32);

fn startup(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    window: Single<Entity, With<Window>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(*window).observe(observe_mouse);

    let (config, _) =
        config_store.config_mut::<DottedGizmos>();
    config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 5.,
        line_scale: 10.,
    };

    commands.spawn(Camera2d);

    let root_position = Transform::default();
    let joint_1_position =
        Transform::from_xyz(100., 200., 0.);
    let joint_2_position =
        Transform::from_xyz(100., 100., 0.);

    commands.spawn((
        Name::new("IKRoot"),
        InverseKinematics,
        root_position,
        BoneLength(
            root_position
                .translation
                .distance(joint_1_position.translation),
        ),
        Mesh2d(
            meshes.add(Capsule2d::new(
                5.0,
                root_position
                    .translation
                    .distance(joint_1_position.translation),
            )),
        ),
        MeshMaterial2d(
            materials.add(Color::hsl(200., 0.95, 0.7)),
        ),
        children![(
            Name::new("Joint1"),
            joint_1_position,
            BoneLength(
                joint_1_position
                    .translation
                    .distance(joint_2_position.translation),
            ),
            Mesh2d(meshes.add(
                Capsule2d::new(
                    5.0,
                    joint_1_position.translation.distance(
                        joint_2_position.translation
                    )
                )
            ),),
            MeshMaterial2d(
                materials.add(Color::hsl(200., 0.95, 0.7)),
            ),
            children![(
                Name::new("Joint2"),
                joint_2_position,
            )]
        )],
    ));

    commands.spawn((
        Name::new("IKRoot"),
        InverseKinematics,
        root_position
            .with_translation(Vec3::new(200., 0., 0.)),
        BoneLength(
            root_position
                .translation
                .distance(joint_1_position.translation),
        ),
        children![
            // bones
            (
                Name::new("Joint1"),
                joint_1_position,
                BoneLength(
                    joint_1_position.translation.distance(
                        joint_2_position.translation
                    ),
                ),
                children![(
                    Name::new("Joint2"),
                    joint_2_position,
                )]
            )
        ],
    ));

    // let root_position = Transform::default();
    // let joint_position = Transform::from_xyz(30., 30., 0.);
    // let joint_2_position =
    //     Transform::from_xyz(30., 30., 0.);

    // ooh wow does rustfmt not like this nesting lol
    // this outer closure is so that rustfmt doesn't
    // touch this flapjack stack of bones. There is
    // no other purpose for it.
    #[rustfmt::skip]
    let mut spawn_lots = || {
    // commands.spawn((
    //     Name::new("IKRoot"),
    //     InverseKinematics,
    //     root_position,
    //     BoneLength(joint_position.translation.length()),
    //     children![(
    //         Name::new("Joint1"),
    //         joint_position,
    //         BoneLength(joint_position.translation.length()),
    //         children![(
    //             Name::new("Joint2"),
    //             joint_position,
    //             BoneLength(joint_position.translation.length()),
    //             children![(
    //                 Name::new("Joint3"),
    //                 joint_position,
    //                 BoneLength(joint_position.translation.length()),
    //                 children![(
    //                     Name::new("Joint4"),
    //                     joint_position,
    //                     BoneLength(joint_position.translation.length()),
    //                     children![(
    //                         Name::new("Joint5"),
    //                         joint_position,
    //                         BoneLength(joint_position.translation.length()),
    //                         children![(
    //                             Name::new("Joint6"),
    //                             joint_2_position,
    //                         )]
    //                     )],
    //                 )],
    //             )],
    //         )],
    //     )],
    // ));
    };
    spawn_lots();
}

fn debug_transforms(
    query: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
) {
    for transform in &query {
        gizmos.axes_2d(*transform, 30.);
    }
}

const TOLERANCE: f32 = 1.;

fn update(
    ik_roots: Query<
        (Entity, &BoneLength),
        With<InverseKinematics>,
    >,
    children: Query<&Children>,
    bone_lengths: Query<&BoneLength>,
    mut gizmos: Gizmos,
    mut dotted_gizmos: Gizmos<DottedGizmos>,
    mouse_position: Option<Res<MousePosition>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    // if there's no mouse_position, just skip
    // everything the mouse_position is our
    // "target" so if we don't have one, there is
    // no target
    let Some(mouse_position) = mouse_position else {
        return;
    };

    // iterate over all ik bodies in the scene
    // using 'ik_bodies as a label in case we have to
    // abandon a specific ik root's processing
    'ik_bodies: for (root_entity, root_bone_length) in
        ik_roots.iter()
    {
        // dotted_gizmos.arrow_2d(
        //     root_transform.translation.xy(),
        //     mouse_position.0,
        //     PINK_400.with_alpha(0.4),
        // );

        // use `ChildOf` relationship to iter bones and
        // sum the length of all bones.
        // `iter_descendants` doesn't include the root
        // element, so we add the root bone length
        let total_length = root_bone_length.0
            + children
                .iter_descendants(root_entity)
                .filter_map(|entity| {
                    bone_lengths.get(entity).ok()
                })
                .map(|bone| bone.0)
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

        // if target isn't reachable, return
        //
        // if the `total_length` of the bones is less than
        // the distance required to reach the mouse, then
        // we can't make it to the target mouse location
        //
        // TODO: mouse_position is relative to world
        // center, and should be relative to IK
        // root
        let root_translation = global_transforms
            .get(root_entity)
            .unwrap()
            .translation()
            .xy();
        if total_length
            < (root_translation - mouse_position.0).length()
        {
            // mouse is out of reach!
            // orient all bones in straight line to mouse
            // direction
            let mouse_vector = (mouse_position.0
                - root_translation)
                .normalize();

            for (a, b) in std::iter::once(root_entity)
                .chain(
                    children.iter_descendants(root_entity),
                )
                .tuple_windows()
            {
                // get the bone length of the first node
                // and use it to figure out the position
                // entity b should be at.
                // let bone_a = bone_lengths.get(a).unwrap();
                // let bone_vector = mouse_vector * bone_a.0;
                // let mut transform =
                //     transforms.get_mut(b).unwrap();
                // transform.translation.x = bone_vector.x;
                // transform.translation.y = bone_vector.y;
                // transform.rotation = Quat::IDENTITY;

                // replicate code from rotation calculations
            }

            continue 'ik_bodies;
        }

        // We use this `Vec` to store the calculations
        // we make that mutate the `GlobalPosition`s.
        // After the loop ends, we take this `Vec` and
        // use the values to update the `Transform`
        // components
        let mut current_positions: Vec<_> = std::iter::once((
            root_translation,
            root_bone_length,
        ))
        .chain(children.iter_descendants(root_entity).map(|entity| (
            global_transforms.get(entity)
                .expect("bones should have GlobalTransform components")
                .translation().xy(),
            bone_lengths.get(entity).unwrap_or(&BoneLength(0.0))
        ))).collect();

        // end effector is the last joint without a child
        let end_effector = children
            .iter_descendants(root_entity)
            .last()
            .and_then(|entity| {
                global_transforms.get(entity).ok()
            })
            .expect("there should be a final bone")
            .translation()
            .xy();

        // `diff` is "how far off is the end joint from
        // the target?"
        let mut diff =
            end_effector.distance(mouse_position.0);

        // loop for forward/backward passes
        // keeps track of iteration count because
        // if the bones can't physically reach the point
        // the loop will never finish
        //
        // 10 iterations is an entirely arbitrary number
        // of maximum iterations.
        let mut iterations = 0;
        while diff > TOLERANCE && iterations < 10 {
            iterations += 1;

            // #########################################
            // #                                       #
            // #  Paper calls this the "Forward Pass"  #
            // #                                       #
            // #########################################
            //
            // which is an iteration from the end_effector
            // bone, to the root bone
            if let Some((pos, _)) =
                current_positions.last_mut()
            {
                pos.x = mouse_position.0.x;
                pos.y = mouse_position.0.y;
            } else {
                error!("bones list to have a bone");
                continue 'ik_bodies;
            }

            // options here are using `windows_mut` from
            // `lending_iterator` https://docs.rs/lending-iterator/latest/lending_iterator/#windows_mut
            // or using peekable.
            // We could also use indices, but I prefer
            // avoiding indices when possible
            let mut it = current_positions
                .iter_mut()
                .rev()
                .peekable();
            while let (Some(p2), Some(p1)) =
                (it.next(), it.peek_mut())
            {
                let vector = p2.0 - p1.0;
                p1.0 = p2.0 - vector.normalize() * p1.1.0;
            }

            // #########################################
            // #                                       #
            // # Paper calls this the "Backward Pass"  #
            // #                                       #
            // #########################################

            // which is an iteration from the root to
            // the end_effector
            if let Some((pos, _)) =
                current_positions.first_mut()
            {
                pos.x = root_translation.x;
                pos.y = root_translation.y;
            } else {
                error!("bones list to have a bone");
                continue 'ik_bodies;
            }

            // options here are using `windows_mut` from
            // `lending_iterator` https://docs.rs/lending-iterator/latest/lending_iterator/#windows_mut
            // or using peekable.
            // We could also use indices, but I prefer
            // avoiding indices when possible
            let mut it =
                current_positions.iter_mut().peekable();
            while let (Some((p1, p1_bone)), Some((p2, _))) =
                (it.next(), it.peek_mut())
            {
                let vector = *p1 - *p2;
                *p2 = *p1 - vector.normalize() * p1_bone.0;
            }

            // set diff and loop again
            diff = current_positions
                .last()
                .unwrap()
                .0
                .distance(mouse_position.0);
        }

        for (a, b) in
            current_positions.iter().tuple_windows()
        {
            dotted_gizmos.arrow_2d(
                a.0,
                b.0,
                PINK_400.with_alpha(0.4),
            );
        }

        info!(?current_positions);

        // Update Root node rotation
        let relative =
            current_positions[1].0 - current_positions[0].0;

        // set "root" to its proper rotation
        // let mut transform =
        //     transforms.get_mut(root_entity).unwrap();
        let angle = relative.to_angle();
        let current_rotation =
            Quat::from_axis_angle(Vec3::Z, angle);
        let mut transform =
            transforms.get_mut(root_entity).unwrap();
        transform.rotation = current_rotation;

        // Update all `Transform`s by taking global
        // positions and converting them to
        // relative measurements suitable
        // for `Transform`
        let it = current_positions.iter();
        for (
            (
                root_entity,
                (previous_node_global_position, _),
            ),
            (entity, (global_position, _)),
            (last_entity, (last_pos, _)),
        ) in std::iter::once(root_entity)
            .chain(children.iter_descendants(root_entity))
            .zip(it)
            .tuple_windows()
        {
            let relative = global_position
                - previous_node_global_position;

            // set "root" to its proper rotation
            // let mut transform =
            //     transforms.get_mut(root_entity).unwrap();
            let angle = relative.to_angle();
            let parent = Transform::from_xyz(
                previous_node_global_position.x,
                previous_node_global_position.y,
                0.,
            )
            .with_rotation(
                Quat::from_axis_angle(Vec3::Z, angle),
            );

            let current_node = Transform::from_xyz(
                global_position.x,
                global_position.y,
                0.,
            )
            .with_rotation(
                Quat::from_axis_angle(
                    Vec3::Z,
                    (last_pos - global_position).to_angle(),
                ),
            );
            let (scale, rotation, translation) =
                (parent.compute_affine().inverse()
                    * current_node.compute_affine())
                .to_scale_rotation_translation();

            let mut transform =
                transforms.get_mut(entity).unwrap();
            transform.scale = scale;
            transform.rotation = rotation;
            transform.translation = translation;
        }

        // Duplicate logic: REFACTOR NEEDED
        let Some((
            (last_position, last_bone),
            (next_to_last_position, next_to_last_bone),
        )) = current_positions
            .iter()
            .rev()
            .tuple_windows()
            .next()
        else {
            error!("unhandled tip!");
            continue 'ik_bodies;
        };

        let relative =
            last_position - next_to_last_position;

        // set "root" to its proper rotation
        // let mut transform =
        //     transforms.get_mut(root_entity).unwrap();
        let angle = relative.to_angle();
        let parent = Transform::from_xyz(
            next_to_last_position.x,
            next_to_last_position.y,
            0.,
        )
        .with_rotation(Quat::from_axis_angle(
            Vec3::Z,
            angle,
        ));

        let current_node = Transform::from_xyz(
            last_position.x,
            last_position.y,
            0.,
        )
        .with_rotation(Quat::from_axis_angle(
            Vec3::Z,
            // same as "next_to_last's" rotation
            (last_position - next_to_last_position)
                .to_angle(),
        ));
        let (scale, rotation, translation) =
            (parent.compute_affine().inverse()
                * current_node.compute_affine())
            .to_scale_rotation_translation();

        let entity = children
            .iter_descendants(root_entity)
            .last()
            .unwrap();
        let mut transform =
            transforms.get_mut(entity).unwrap();
        transform.scale = scale;
        transform.rotation = rotation;
        transform.translation = translation;
    }
}

#[derive(Resource)]
struct MousePosition(Vec2);

// a system that updates a `Resource` with the
// current world position of the mouse.
//
// We use the mouse world position to drive the
// IK target position
fn observe_mouse(
    schmove: On<Pointer<Move>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mouse_position: Option<ResMut<MousePosition>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = *camera_query;

    // Calculate a world position based on the
    // cursor's viewport position.
    let Ok(world_position) = camera.viewport_to_world_2d(
        camera_transform,
        schmove.pointer_location.position,
    ) else {
        return;
    };

    if let Some(mut mp) = mouse_position {
        // update the mouse position
        mp.0 = world_position;
    } else {
        // insert the Resource the first chance we get.
        // Could also init to 0 when building the app
        commands
            .insert_resource(MousePosition(world_position));
    }
}
