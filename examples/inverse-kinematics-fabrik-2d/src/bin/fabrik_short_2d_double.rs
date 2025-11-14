use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind::*, prelude::*};
use inverse_kinematics_fabrik_2d::{BoneLength, FabrikPlugin, InverseKinematicEndEffector};

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .add_plugins((DefaultPlugins, FabrikPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, debug_transforms)
        .run();
}

fn startup(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(*window).observe(observe_mouse);

    commands.spawn(Camera2d);

    let root_position = Transform::default();
    let joint_1_position = Transform::from_xyz(100., 200., 0.);
    let joint_2_position = Transform::from_xyz(100., 100., 0.);

    commands.spawn((
        Name::new("IKRoot"),
        root_position,
        BoneLength(
            root_position
                .translation
                .distance(joint_1_position.translation),
        ),
        Visibility::Inherited,
        children![
            (
                Mesh2d(
                    meshes.add(Capsule2d::new(
                        5.0,
                        root_position
                            .translation
                            .distance(joint_1_position.translation),
                    )),
                ),
                MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7)),),
                Transform::from_xyz(
                    root_position
                        .translation
                        .distance(joint_1_position.translation)
                        / 2.,
                    0.,
                    0.
                )
                .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
            ),
            (
                Name::new("Joint1"),
                joint_1_position,
                BoneLength(
                    joint_1_position
                        .translation
                        .distance(joint_2_position.translation),
                ),
                Visibility::Inherited,
                children![
                    (
                        Mesh2d(
                            meshes.add(Capsule2d::new(
                                5.0,
                                joint_1_position
                                    .translation
                                    .distance(joint_2_position.translation)
                            )),
                        ),
                        MeshMaterial2d(materials.add(Color::hsl(200., 0.95, 0.7)),),
                        Transform::from_xyz(
                            joint_1_position
                                .translation
                                .distance(joint_2_position.translation)
                                / 2.,
                            0.,
                            0.
                        )
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                    ),
                    (
                        Name::new("Joint2"),
                        InverseKinematicEndEffector {
                            affected_bone_count: 2,
                            tolerance: 1.,
                            // default target to "far forward"
                            target: Vec2::X * 1000.
                        },
                        joint_2_position,
                    )
                ]
            )
        ],
    ));

    commands.spawn((
        Name::new("IKRoot"),
        root_position.with_translation(Vec3::new(200., 0., 0.)),
        BoneLength(
            root_position
                .translation
                .distance(joint_1_position.translation),
        ),
        children![(
            Name::new("Joint1"),
            joint_1_position,
            BoneLength(
                joint_1_position
                    .translation
                    .distance(joint_2_position.translation),
            ),
            children![(
                Name::new("Joint2"),
                InverseKinematicEndEffector {
                    affected_bone_count: 2,
                    tolerance: 1.0,
                    // default target to "far forward"
                    target: Vec2::X * 1000.
                },
                joint_2_position,
            )]
        )],
    ));
}

fn debug_transforms(query: Query<&GlobalTransform>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.axes_2d(*transform, 30.);
    }
}

/// We use the mouse world position to drive the
/// IK target position
pub fn observe_mouse(
    schmove: On<Pointer<Move>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut end_effectors: Query<&mut InverseKinematicEndEffector>,
) {
    let (camera, camera_transform) = *camera_query;

    // Calculate a world position based on the
    // cursor's viewport position.
    let Ok(world_position) =
        camera.viewport_to_world_2d(camera_transform, schmove.pointer_location.position)
    else {
        return;
    };

    for mut end_effector in &mut end_effectors {
        // update the mouse position
        end_effector.target = world_position;
    }
}
