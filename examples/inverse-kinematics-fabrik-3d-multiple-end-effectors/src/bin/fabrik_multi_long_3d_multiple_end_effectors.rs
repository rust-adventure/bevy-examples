use bevy::{
    color::palettes::tailwind::*,
    log::{tracing_subscriber::field::MakeExt, BoxedFmtLayer},
    prelude::*,
};
use inverse_kinematics_fabrik_3d_multiple_end_effectors::{
    FabrikPlugin, IkRoot, InverseKinematicEndEffector,
};

// logging with the example name was getting a bit long, so this
// customizes it
fn fmt_layer(_app: &mut App) -> Option<BoxedFmtLayer> {
    Some(Box::new(
        bevy::log::tracing_subscriber::fmt::Layer::default()
            .without_time()
            .map_fmt_fields(MakeExt::debug_alt)
            .with_writer(std::io::stderr),
    ))
}

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_950.into()))
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                fmt_layer,

                ..default()
            }),
            FabrikPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, (update_target, debug_transforms))
        .run();
}

// A marker component for the sphere target
#[derive(Component)]
struct SphereTarget;

// A marker component for the sphere target
#[derive(Component)]
struct SphereTargetReceiver;

// A marker component for the sphere target
#[derive(Component)]
struct SphereTargetB;

// A marker component for the sphere target
#[derive(Component)]
struct SphereTargetBReceiver;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    // spawn a sphere target in.
    commands.spawn((
        SphereTargetB,
        Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    let root_position = Transform::default();
    let joint_position = Transform::from_xyz(1., 1., 0.);
    let joint_2_position = Transform::from_xyz(1., 1., 0.);

    let sphere_size = 0.2;
    let sphere = meshes.add(Sphere::new(sphere_size).mesh().uv(32, 18));
    // this is 2x the size of the sphere because we take a bit off both ends
    // of the "bone" meshes
    let sphere_gap = 2. * sphere_size;

    let cuboid = meshes.add(Cuboid::new(
        0.2,
        0.2,
        joint_position.translation.length() - sphere_gap,
    ));

    let joint_material = MeshMaterial3d(materials.add(Color::hsl(100., 0.95, 0.7)));
    let bone_material = MeshMaterial3d(materials.add(Color::hsl(300., 0.95, 0.7)));

    commands.spawn((
        Name::new("IKRoot"),
        IkRoot,
        root_position,
        Mesh3d(sphere.clone()),
        joint_material.clone(),
        children![
            (
                Mesh3d(cuboid.clone()),
                bone_material.clone(),
                Transform::from_xyz(0., 0., joint_position.translation.length() / 2.,) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
            ),
            (
                Name::new("Joint1"),
                joint_position,
                Mesh3d(sphere.clone()),
                joint_material.clone(),
                children![
                    (
                        Mesh3d(cuboid.clone()),
                        bone_material.clone(),
                        Transform::from_xyz(0., 0., joint_position.translation.length() / 2.,) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                    ),
                    (
                        Name::new("Joint2"),
                        joint_position,
                        Mesh3d(sphere.clone()),
                        joint_material.clone(),
                        children![
                            (
                                Mesh3d(cuboid.clone()),
                                bone_material.clone(),
                                Transform::from_xyz(
                                    0.,
                                    0.,
                                    joint_position.translation.length() / 2.,
                                ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                            ),
                            (
                                Name::new("Joint3"),
                                joint_position,
                                Mesh3d(sphere.clone()),
                                joint_material.clone(),
                                children![
                                    // (
                                    //     Mesh3d(cuboid.clone()),
                                    //     bone_material.clone(),
                                    //     Transform::from_xyz(
                                    //         0.,
                                    //         0.,
                                    //         joint_position.translation.length() / 2.,
                                    //     ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                                    // ),
                                    (
                                        Name::new("Joint4"),
                                        joint_position,
                                        Mesh3d(sphere.clone()),
                                        joint_material.clone(),
                                        children![
                                            (
                                                Mesh3d(cuboid.clone()),
                                                bone_material.clone(),
                                                Transform::from_xyz(
                                                    0.,
                                                    0.,
                                                    joint_position.translation.length() / 2.,
                                                ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                                            ),
                                            (
                                                Name::new("Joint5"),
                                                joint_position,
                                                Mesh3d(sphere.clone()),
                                                joint_material.clone(),
                                                children![
                                                    (
                                                        Mesh3d(cuboid.clone()),
                                                        bone_material.clone(),
                                                        Transform::from_xyz(
                                                            0.,
                                                            0.,
                                                            joint_position.translation.length()
                                                                / 2.,
                                                        ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                                                    ),
                                                    (
                                                        Name::new("Joint6"),
                                                        joint_2_position,
                                                        InverseKinematicEndEffector {
                                                            affected_bone_count: 6,
                                                            tolerance: 0.01,
                                                            target: Vec3::ZERO
                                                        },
                                                        SphereTargetReceiver
                                                    )
                                                ]
                                            )
                                        ],
                                    ),
                                    (
                                        Name::new("Joint4b"),
                                        joint_position,
                                        Mesh3d(sphere.clone()),
                                        joint_material.clone(),
                                        children![
                                            (
                                                Mesh3d(cuboid.clone()),
                                                bone_material.clone(),
                                                Transform::from_xyz(
                                                    0.,
                                                    0.,
                                                    joint_position.translation.length() / 2.,
                                                ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                                            ),
                                            (
                                                Name::new("Joint5b"),
                                                joint_position,
                                                Mesh3d(sphere.clone()),
                                                joint_material.clone(),
                                                children![
                                                    (
                                                        Mesh3d(cuboid.clone()),
                                                        bone_material.clone(),
                                                        Transform::from_xyz(
                                                            0.,
                                                            0.,
                                                            joint_position.translation.length()
                                                                / 2.,
                                                        ) // .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2))
                                                    ),
                                                    (
                                                        Name::new("Joint6b"),
                                                        joint_2_position,
                                                        InverseKinematicEndEffector {
                                                            affected_bone_count: 6,
                                                            tolerance: 0.01,
                                                            target: Vec3::ZERO
                                                        },
                                                        SphereTargetBReceiver
                                                    )
                                                ]
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

fn debug_transforms(query: Query<&GlobalTransform>, mut gizmos: Gizmos) {
    for transform in &query {
        // gizmos.axes(*transform, 1.);
    }
}

fn update_target(
    time: Res<Time>,
    mut sphere_target: Single<&mut Transform, (With<SphereTarget>, Without<SphereTargetB>)>,
    mut sphere_target_b: Single<&mut Transform, (With<SphereTargetB>, Without<SphereTarget>)>,
    mut end_effector: Single<
        &mut InverseKinematicEndEffector,
        (With<SphereTargetReceiver>, Without<SphereTargetBReceiver>),
    >,
    mut end_effector_b: Single<
        &mut InverseKinematicEndEffector,
        (With<SphereTargetBReceiver>, Without<SphereTargetReceiver>),
    >,
) {
    // move the sphere target around the scene
    sphere_target.translation = Vec3::new(
        time.elapsed_secs().sin() * 3.,
        time.elapsed_secs().cos() * 3. + 3.,
        time.elapsed_secs().cos() * 2. + 1.,
    );

    // Update the end_effector target to be the position
    // the sphere is at.
    end_effector.target = sphere_target.translation;

    // move the sphere target around the scene
    sphere_target_b.translation = Vec3::new(
        time.elapsed_secs().sin() * 3. + 5.,
        time.elapsed_secs().cos() * 3. + 3.,
        time.elapsed_secs().cos() * 2. + 1.,
    );

    // Update the end_effector target to be the position
    // the sphere is at.
    end_effector_b.target = sphere_target_b.translation;
}
