use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind::*, prelude::*};
use inverse_kinematics_fabrik_3d::{
    process_inverse_kinematics, BoneLength, DottedGizmos, InverseKinematicEndEffector,
    MousePosition,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .insert_resource(MousePosition(Vec3::new(1., 2., -1.)))
        .init_gizmo_group::<DottedGizmos>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (update_target, debug_transforms, process_inverse_kinematics),
        )
        .run();
}

#[derive(Component)]
struct Target;

fn startup(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (config, _) = config_store.config_mut::<DottedGizmos>();
    config.line.style = GizmoLineStyle::Dashed {
        gap_scale: 5.,
        line_scale: 10.,
    };

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // spawn a sphere target in.
    commands.spawn((
        Target,
        Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // let root_position = Transform::default();
    // let joint_1_position = Transform::from_xyz(3., 5., 0.);
    // let joint_2_position = Transform::from_xyz(3., 2., 0.);

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
    //             Mesh3d(meshes.add(Cuboid::new(
    //                 0.3,
    //                 0.3,
    //                 root_position.translation.distance(
    //                     joint_1_position.translation
    //                 ),
    //             )),),
    //             MeshMaterial3d(
    //                 materials
    //                     .add(Color::hsl(200., 0.95, 0.7)),
    //             ),
    //             Transform::from_xyz(
    //                 root_position.translation.distance(
    //                     joint_1_position.translation
    //                 ) / 2.,
    //                 0.,
    //                 0.
    //             )
    //             .with_rotation(
    //                 Quat::from_axis_angle(
    //                     Vec3::Z,
    //                     FRAC_PI_2
    //                 )
    //             )
    //         ),
    //         (
    //             Name::new("Joint1"),
    //             joint_1_position,
    //             BoneLength(
    //                 joint_1_position.translation.distance(
    //                     joint_2_position.translation
    //                 ),
    //             ),
    //             Visibility::Inherited,
    //             children![(
    //                 InverseKinematicEndEffector {
    //                     affected_bone_count: 2
    //                 },
    //                 Name::new("Joint2"),
    //                 joint_2_position,
    //             )]
    //         )
    //     ],
    // ));

    // commands.spawn((
    //     Name::new("IKRoot"),
    //     InverseKinematics,
    //     root_position
    //         .with_translation(Vec3::new(200., 0.,
    // 0.)),     BoneLength(
    //         root_position
    //             .translation
    //
    // .distance(joint_1_position.translation),
    //     ),
    //     children![
    //         // bones
    //         (
    //             Name::new("Joint1"),
    //             joint_1_position,
    //             BoneLength(
    //
    // joint_1_position.translation.distance(
    //
    // joint_2_position.translation
    // ),             ),
    //             children![(
    //                 Name::new("Joint2"),
    //                 joint_2_position,
    //             )]
    //         )
    //     ],
    // ));

    let root_position = Transform::default();
    let joint_position = Transform::from_xyz(1., 1., 0.);
    let joint_2_position = Transform::from_xyz(1., 1., 0.);

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
        children![(
            Name::new("Joint1"),
            joint_position,
            BoneLength(joint_position.translation.length()),
            children![(
                Name::new("Joint2"),
                joint_position,
                BoneLength(joint_position.translation.length()),
                children![(
                    Name::new("Joint3"),
                    joint_position,
                    BoneLength(joint_position.translation.length()),
                    children![(
                        Name::new("Joint4"),
                        joint_position,
                        BoneLength(joint_position.translation.length()),
                        children![(
                            Name::new("Joint5"),
                            joint_position,
                            BoneLength(joint_position.translation.length()),
                            children![(
                                Name::new("Joint6"),
                                joint_2_position,
                                InverseKinematicEndEffector{ affected_bone_count: 6 }
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
        gizmos.axes(*transform, 1.);
    }
}

fn update_target(
    mut position: ResMut<MousePosition>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Target>>,
) {
    position.0.x = time.elapsed_secs().sin() * 3.;
    position.0.y = time.elapsed_secs().cos() * 3. + 3.;
    position.0.z = time.elapsed_secs().cos() * 2. + 1.;

    for mut sphere in &mut query {
        sphere.translation.x = position.0.x;
        sphere.translation.y = position.0.y;
        sphere.translation.z = position.0.z;
    }
}

// a system that updates a `Resource` with the
// current world position of the mouse.
//
// We use the mouse world position to drive the
// // IK target position
// fn observe_mouse(
//     schmove: On<Pointer<Move>>,
//     camera_query: Single<(&Camera, &GlobalTransform)>,
//     mouse_position: Option<ResMut<MousePosition>>,
//     mut commands: Commands,
// ) {
//     let (camera, camera_transform) = *camera_query;

//     // Calculate a world position based on the
//     // cursor's viewport position.
//     let Ok(world_position) = camera.viewport_to_world_2d(
//         camera_transform,
//         schmove.pointer_location.position,
//     ) else {
//         return;
//     };

//     if let Some(mut mp) = mouse_position {
//         // update the mouse position
//         mp.0 = world_position;
//     } else {
//         // insert the Resource the first chance we get.
//         // Could also init to 0 when building the app
//         commands
//             .insert_resource(MousePosition(world_position));
//     }
// }
