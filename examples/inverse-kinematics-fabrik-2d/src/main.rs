use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind::*, prelude::*};
use inverse_kinematics_fabrik_2d::{
    process_inverse_kinematics, BoneLength, DottedGizmos, InverseKinematicEndEffector,
    MousePosition,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .init_gizmo_group::<DottedGizmos>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (debug_transforms, process_inverse_kinematics))
        .run();
}

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

    let (config, _) = config_store.config_mut::<DottedGizmos>();
    config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 5.,
        line_scale: 10.,
    };

    commands.spawn(Camera2d);

    // let root_position = Transform::default();
    // let joint_1_position = Transform::from_xyz(100., 200., 0.);
    // let joint_2_position = Transform::from_xyz(100., 100., 0.);

    // commands.spawn((
    //     Name::new("IKRoot"),
    //     root_position,
    //     BoneLength(
    //         root_position
    //             .translation
    //             .distance(joint_1_position.translation),
    //     ),
    //     Visibility::Inherited,
    //     children![
    //         (
    //             Mesh2d(
    //                 meshes.add(Capsule2d::new(
    //                     5.0,
    //                     root_position
    //                         .translation
    //                         .distance(joint_1_position.translation),
    //                 )),
    //             ),
    //             MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7)),),
    //             Transform::from_xyz(
    //                 root_position
    //                     .translation
    //                     .distance(joint_1_position.translation)
    //                     / 2.,
    //                 0.,
    //                 0.
    //             )
    //             .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
    //         ),
    //         (
    //             Name::new("Joint1"),
    //             joint_1_position,
    //             BoneLength(
    //                 joint_1_position
    //                     .translation
    //                     .distance(joint_2_position.translation),
    //             ),
    //             Visibility::Inherited,
    //             children![
    //                 (
    //                     Mesh2d(
    //                         meshes.add(Capsule2d::new(
    //                             5.0,
    //                             joint_1_position
    //                                 .translation
    //                                 .distance(joint_2_position.translation)
    //                         )),
    //                     ),
    //                     MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7)),),
    //                     Transform::from_xyz(
    //                         joint_1_position
    //                             .translation
    //                             .distance(joint_2_position.translation)
    //                             / 2.,
    //                         0.,
    //                         0.
    //                     )
    //                     .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
    //                 ),
    //                 (
    //                     Name::new("Joint2"),
    //                     InverseKinematicEndEffector {
    //                         affected_bone_count: 2
    //                     },
    //                     joint_2_position,
    //                 )
    //             ]
    //         )
    //     ],
    // ));

    // commands.spawn((
    //     Name::new("IKRoot"),
    //     root_position.with_translation(Vec3::new(200., 0., 0.)),
    //     BoneLength(
    //         root_position
    //             .translation
    //             .distance(joint_1_position.translation),
    //     ),
    //     children![(
    //         Name::new("Joint1"),
    //         joint_1_position,
    //         BoneLength(
    //             joint_1_position
    //                 .translation
    //                 .distance(joint_2_position.translation),
    //         ),
    //         children![(
    //             Name::new("Joint2"),
    //             InverseKinematicEndEffector {
    //                 affected_bone_count: 2
    //             },
    //             joint_2_position,
    //         )]
    //     )],
    // ));

    let root_position = Transform::default();
    let joint_position = Transform::from_xyz(30., 30., 0.);
    let joint_2_position = Transform::from_xyz(30., 30., 0.);

    // ooh wow does rustfmt not like this nesting lol
    // this outer closure is so that rustfmt doesn't
    // touch this flapjack stack of bones. There is
    // no other purpose for it.
    #[rustfmt::skip]
    let mut spawn_lots = || {
    commands.spawn((
        Name::new("IKRoot"),
        root_position,
        BoneLength(joint_position.translation.length()),
        Visibility::Inherited,
        children![(
            Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
            MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
            Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                .with_rotation(
                    Quat::from_axis_angle(
                        Vec3::Z,
                        FRAC_PI_2
                    )
                ),
            ),(
            Name::new("Joint1"),
            joint_position,
            BoneLength(joint_position.translation.length()),
            Visibility::Inherited,
            children![(
                Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
                MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
                Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                    .with_rotation(
                        Quat::from_axis_angle(
                            Vec3::Z,
                            FRAC_PI_2
                        )
                    ),
                ),(
                Name::new("Joint2"),
                joint_position,
                BoneLength(joint_position.translation.length()),
                Visibility::Inherited,
                children![(
                    Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
                    MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
                    Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                        .with_rotation(
                            Quat::from_axis_angle(
                                Vec3::Z,
                                FRAC_PI_2
                            )
                        ),
                    ),(
                    Name::new("Joint3"),
                    joint_position,
                    BoneLength(joint_position.translation.length()),
                    Visibility::Inherited,
                    children![(
                        Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
                        MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
                        Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                            .with_rotation(
                                Quat::from_axis_angle(
                                    Vec3::Z,
                                    FRAC_PI_2
                                )
                            ),
                        ),(
                        Name::new("Joint4"),
                        joint_position,
                        BoneLength(joint_position.translation.length()),
                        Visibility::Inherited,
                        children![(
                            Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
                            MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
                            Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                                .with_rotation(
                                    Quat::from_axis_angle(
                                        Vec3::Z,
                                        FRAC_PI_2
                                    )
                                ),
                            ),(
                            Name::new("Joint5"),
                            joint_position,
                            BoneLength(joint_position.translation.length()),
                            Visibility::Inherited,
                            children![(
                                Mesh2d(meshes.add(Capsule2d::new(5.0,joint_position.translation.length()))),
                                MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7))),
                                Transform::from_xyz(joint_position.translation.length() / 2., 0., 0.)
                                    .with_rotation(
                                        Quat::from_axis_angle(
                                            Vec3::Z,
                                            FRAC_PI_2
                                        )
                                    ),
                                ),(
                                InverseKinematicEndEffector {
                                    affected_bone_count: 6
                                },
                                Name::new("Joint6"),
                                joint_2_position,
                            )]
                        )],
                    )],
                )],
            )],
        )],
    ));
    };
    spawn_lots();
}

fn debug_transforms(query: Query<&GlobalTransform>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.axes_2d(*transform, 30.);
    }
}

/// a system that updates a `Resource` with the
/// current world position of the mouse.
///
/// We use the mouse world position to drive the
/// IK target position
pub fn observe_mouse(
    schmove: On<Pointer<Move>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mouse_position: Option<ResMut<MousePosition>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = *camera_query;

    // Calculate a world position based on the
    // cursor's viewport position.
    let Ok(world_position) =
        camera.viewport_to_world_2d(camera_transform, schmove.pointer_location.position)
    else {
        return;
    };

    if let Some(mut mp) = mouse_position {
        // update the mouse position
        mp.0 = world_position;
    } else {
        // insert the Resource the first chance we get.
        // Could also init to 0 when building the app
        commands.insert_resource(MousePosition(world_position));
    }
}
