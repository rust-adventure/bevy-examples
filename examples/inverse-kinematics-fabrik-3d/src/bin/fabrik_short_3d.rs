use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind::*, prelude::*};
use inverse_kinematics_fabrik_3d::{
    BoneLength, DottedGizmos, FabrikPlugin, InverseKinematicEndEffector,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .add_plugins((DefaultPlugins, FabrikPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, (update_target, debug_transforms))
        .run();
}

// A marker component for the sphere target
#[derive(Component)]
struct SphereTarget;

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
        SphereTarget,
        Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    let root_position = Transform::default();
    let joint_1_position = Transform::from_xyz(3., 5., 0.);
    let joint_2_position = Transform::from_xyz(3., 2., 0.);

    let sphere = meshes.add(Sphere::new(0.2).mesh().uv(32, 18));

    let joint_material = MeshMaterial3d(materials.add(Color::hsl(100., 0.95, 0.7)));

    commands.spawn((
        Name::new("IKRoot"),
        root_position,
        BoneLength(
            root_position
                .translation
                .distance(joint_1_position.translation),
        ),
        Mesh3d(sphere.clone()),
        joint_material.clone(),
        children![
            (
                Mesh3d(
                    meshes.add(Cuboid::new(
                        0.2,
                        0.2,
                        root_position
                            .translation
                            .distance(joint_1_position.translation)
                            - 0.4,
                    ))
                ),
                MeshMaterial3d(materials.add(Color::hsl(300., 0.95, 0.7))),
                Transform::from_xyz(
                    0.,
                    0.,
                    root_position
                        .translation
                        .distance(joint_1_position.translation)
                        / 2.,
                )
            ),
            (
                Name::new("Joint1"),
                joint_1_position,
                BoneLength(
                    joint_1_position
                        .translation
                        .distance(joint_2_position.translation),
                ),
                Mesh3d(sphere.clone()),
                joint_material.clone(),
                children![
                    (
                        Mesh3d(
                            meshes.add(Cuboid::new(
                                0.2,
                                0.2,
                                joint_1_position
                                    .translation
                                    .distance(joint_2_position.translation)
                                    - 0.4,
                            )),
                        ),
                        MeshMaterial3d(materials.add(Color::hsl(360., 0.95, 0.7))),
                        Transform::from_xyz(
                            0.,
                            0.,
                            joint_1_position
                                .translation
                                .distance(joint_2_position.translation)
                                / 2.,
                        )
                    ),
                    (
                        InverseKinematicEndEffector {
                            affected_bone_count: 2,
                            tolerance: 0.01,
                            target: Vec3::Z * 1000.
                        },
                        Name::new("Joint2"),
                        joint_2_position,
                        Mesh3d(sphere.clone()),
                        joint_material.clone(),
                    )
                ]
            )
        ],
    ));
}

fn debug_transforms(query: Query<&GlobalTransform>, mut gizmos: Gizmos) {
    for transform in &query {
        gizmos.axes(*transform, 1.);
    }
}

fn update_target(
    time: Res<Time>,
    mut sphere_target: Single<&mut Transform, With<SphereTarget>>,
    mut end_effectors: Query<&mut InverseKinematicEndEffector>,
) {
    // move the sphere target around the scene
    sphere_target.translation = Vec3::new(
        time.elapsed_secs().sin() * 3.,
        time.elapsed_secs().cos() * 3. + 3.,
        time.elapsed_secs().cos() * 2. + 1.,
    );

    for mut end_effector in &mut end_effectors {
        // Update the end_effector target to be the position
        // the sphere is at.
        end_effector.target = sphere_target.translation;
    }
}
