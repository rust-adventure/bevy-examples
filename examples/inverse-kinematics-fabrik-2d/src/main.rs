use bevy::{
    color::palettes::{css::MAGENTA, tailwind::*},
    prelude::*,
    window::PrimaryWindow,
};
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
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

#[derive(Debug, Component)]
struct EndEffector;

fn startup(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    window: Single<Entity, With<Window>>,
) {
    commands.entity(*window).observe(observe_mouse);

    let (config, _) =
        config_store.config_mut::<DottedGizmos>();
    config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 5.,
        line_scale: 10.,
    };

    commands.spawn(Camera2d::default());

    // let root_position = Transform::default();
    // let joint_1_position =
    //     Transform::from_xyz(100., 200., 0.);
    // let joint_2_position =
    //     Transform::from_xyz(100., 100., 0.);

    // commands.spawn((
    //     Name::new("IKRoot"),
    //     InverseKinematics,
    //     root_position,
    //     BoneLength(
    //         root_position
    //             .translation
    //             .distance(joint_1_position.translation),
    //     ),
    //     children![
    //         // bones
    //         (
    //             Name::new("Joint1"),
    //             joint_1_position,
    //             BoneLength(
    //                 joint_1_position.translation.distance(
    //                     joint_2_position.translation
    //                 ),
    //             ),
    //             children![(
    //                 Name::new("Joint2"),
    //                 EndEffector,
    //                 joint_2_position,
    //             )]
    //         )
    //     ],
    // ));

    let root_position = Transform::default();
    let joint_position = Transform::from_xyz(30., 30., 0.);
    let joint_2_position =
        Transform::from_xyz(30., 30., 0.);

    commands.spawn((
        Name::new("IKRoot"),
        InverseKinematics,
        root_position,
        BoneLength(joint_position.translation.length()),
        children![
            // bones
            (
                Name::new("Joint1"),
                joint_position,
                BoneLength(
                    joint_position.translation.length(),
                ),
                children![
                    // bones
                    (
                        Name::new("Joint1"),
                        joint_position,
                        BoneLength(
                            joint_position
                                .translation
                                .length(),
                        ),
                        children![
                            // bones
                            (
                                Name::new("Joint1"),
                                joint_position,
                                BoneLength(
                                    joint_position
                                        .translation
                                        .length(),
                                ),
                                children![
                                    // bones
                                    (
                                        Name::new("Joint1"),
                                        joint_position,
                                        BoneLength(
                                            joint_position
                                                .translation
                                                .length(),
                                        ),
                                        children![
            // bones
            (
                Name::new("Joint1"),
                joint_position,
                BoneLength(
                    joint_position.translation.length(),
                ),
                children![(
                    Name::new("Joint2"),
                    EndEffector,
                    joint_2_position,
                )]
            )
        ],
                                    )
                                ],
                            )
                        ],
                    )
                ],
            )
        ],
    ));
}

fn debug_transforms(
    query: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for transform in &query {
        gizmos.axes_2d(*transform, 30.);
    }
}

const TOLERANCE: f32 = 1.;

fn update(
    ik_roots: Query<(
        Entity,
        &BoneLength,
        &InverseKinematics,
    )>,
    end_effectors: Query<(), With<EndEffector>>,
    ik_bones: Query<
        &BoneLength,
        Without<InverseKinematics>,
    >,
    children: Query<&Children>,
    parents: Query<&ChildOf>,
    bone_lengths: Query<&BoneLength>,
    // target: mouse
    mut gizmos: Gizmos,
    mut dotted_gizmos: Gizmos<DottedGizmos>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mouse_position: Option<Res<MousePosition>>,
    mut commands: Commands,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    let Some(mouse_position) = mouse_position else {
        return;
    };

    for (root_entity, root_bone_length, ik) in
        ik_roots.iter()
    {
        // dotted_gizmos.arrow_2d(
        //     root_transform.translation.xy(),
        //     mouse_position.0,
        //     PINK_400.with_alpha(0.4),
        // );

        // loop start
        // use childof relationship to iter bones
        let total_length = children
            .iter_descendants(root_entity)
            .filter_map(|entity| {
                bone_lengths.get(entity).ok()
            })
            .map(|bone| bone.0)
            .sum::<f32>()
            + root_bone_length.0;

        // info!(?total_length);

        gizmos.circle_2d(
            Isometry2d::IDENTITY,
            total_length,
            SLATE_400,
        );

        // if target isn't reachable, return
        // if the total_length of the bones is less than
        // the distance required to reach the mouse, then
        // we can't make it to the target
        //
        // TODO: mouse_position is relative to world center,
        // and should be relative to IK root
        if total_length < mouse_position.0.length() {
            // warn!("mouse is out of reach!");
            return;
        }
        // temporarily cache global positions for mutation
        let mut current_positions = vec![
            (
            global_transforms.get(root_entity).expect("bones should have GlobalTransform components").translation().xy(),
            root_bone_length,
            )
        ];
        for entity in children.iter_descendants(root_entity)
        {
            current_positions.push(
                (
                    global_transforms.get(entity).expect("bones should have GlobalTransform components").translation().xy(),
                    bone_lengths.get(entity).unwrap_or(&BoneLength(0.0))
                ));
        }

        // let current_root_position = transforms
        //     .get(root_entity)
        //     .unwrap()
        //     .translation
        //     .xy();

        let end_effector_entity = children
            .iter_descendants(root_entity)
            .find(|entity| {
                end_effectors.get(*entity).is_ok()
            })
            .expect("ik chains need an EndEffector joint");
        let end_effector = global_transforms
            .get(end_effector_entity)
            .unwrap()
            .translation()
            .xy();

        let mut diff =
            end_effector.distance(mouse_position.0);
        let mut i = 0;
        loop {
            if diff < TOLERANCE {
                break;
            }
            if i > 10 {
                info!("break because of i");
                break;
            }
            i += 1;

            // forward/backward pass

            // Backward Pass
            {
                let (pos, _) = current_positions
                    .last_mut()
                    .expect("bones list to have a bone");

                pos.x = mouse_position.0.x;
                pos.y = mouse_position.0.y;
            }

            for (idx_p2, idx_p1) in (0..current_positions
                .len())
                .into_iter()
                .rev()
                .tuple_windows()
            {
                let vector = current_positions[idx_p2].0
                    - current_positions[idx_p1].0;
                current_positions[idx_p1].0 =
                    current_positions[idx_p2].0
                        - vector.normalize()
                            * current_positions[idx_p1].1.0;
            }
            // Forward Pass
            {
                let (pos, _) = current_positions
                    .first_mut()
                    .expect("bones list to have a bone");

                let root_translation = global_transforms
                    .get(root_entity)
                    .unwrap()
                    .translation()
                    .xy();

                pos.x = root_translation.x;
                pos.y = root_translation.y;
            }

            for (idx_p1, idx_p2) in (0..current_positions
                .len())
                .into_iter()
                .tuple_windows()
            {
                let vector = current_positions[idx_p1].0
                    - current_positions[idx_p2].0;
                current_positions[idx_p2].0 =
                    current_positions[idx_p1].0
                        - vector.normalize()
                            * current_positions[idx_p1].1.0;
            }

            // set diff
            // distance, not normalized distance
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

        let mut it = current_positions.into_iter();
        let _ = it.next().unwrap().0;

        // commands.entity(root_entity).insert();

        let mut current_root_position = global_transforms
            .get(root_entity)
            .unwrap()
            .translation()
            .xy();
        for (entity, (global_position, _)) in
            children.iter_descendants(root_entity).zip(it)
        {
            let relative =
                global_position - current_root_position;
            // current_root_position = global_position;
            let mut transform =
                transforms.get_mut(entity).unwrap();
            transform.translation.x = relative.x;
            transform.translation.y = relative.y;
        }
        // for  in current_positions
        // {
        // }
    }
}

#[derive(Resource)]
struct MousePosition(Vec2);

fn observe_mouse(
    schmove: On<Pointer<Move>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mouse_position: Option<ResMut<MousePosition>>,
    mut commands: Commands,
) {
    let cursor_position = schmove.pointer_location.position;
    let (camera, camera_transform) = *camera_query;
    // Calculate a world position based on the cursor's position.
    let Ok(world_position) = camera.viewport_to_world_2d(
        camera_transform,
        cursor_position,
    ) else {
        return;
    };

    if let Some(mut mp) = mouse_position {
        mp.0 = world_position;
    } else {
        commands
            .insert_resource(MousePosition(world_position));
    }
}
