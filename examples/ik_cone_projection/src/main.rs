use std::f32::consts::FRAC_PI_8;

use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    color::palettes::tailwind::*,
    ecs::system::command::init_resource,
    prelude::*,
};
use itertools::Itertools;

const CIRCLE_GIZMO_RESOLUTION: u32 = 100;

fn main() {
    App::new()
        .init_gizmo_group::<Gizmos3d>()
        .init_gizmo_group::<Gizmos2d>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, (update, update_cameras))
        .init_resource::<Joints>()
        .add_observer(on_drag)
        .run();
}

#[derive(Resource, Default)]
struct Joints(Vec<Entity>);

#[derive(Default, Reflect, GizmoConfigGroup)]
struct Gizmos3d {}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct Gizmos2d {}

fn startup(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_2d: ResMut<Assets<ColorMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<Gizmos3d>();
    config.render_layers = RenderLayers::layer(0);
    config.line = GizmoLineConfig {
        width: 10.,
        // perspective: true,
        ..default()
    };

    let (config, _) = config_store.config_mut::<Gizmos2d>();
    config.render_layers = RenderLayers::layer(1);
    config.line = GizmoLineConfig {
        width: 10.,
        ..default()
    };

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let window = window.resolution.physical_size();
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(
                SKY_800.into(),
            ),
            viewport: Some(Viewport {
                physical_size: UVec2::new(
                    window.x / 2,
                    window.y,
                ),
                ..default()
            }),
            ..default()
        },
        RenderLayers::layer(0),
        Transform::from_xyz(0., 2., 10.)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 2,
            // clear_color: ClearColorConfig::Custom(
            //     BLUE_800.into(),
            // ),
            viewport: Some(Viewport {
                physical_position: UVec2::new(
                    window.x / 2,
                    0,
                ),
                physical_size: UVec2::new(
                    window.x / 2,
                    window.y,
                ),
                ..default()
            }),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode:
                bevy::camera::ScalingMode::FixedHorizontal {
                    viewport_width: 10.,
                },
            // scale: (),
            ..OrthographicProjection::default_2d()
        }),
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Mesh2d(
            meshes.add(Rectangle::from_size(Vec2::splat(
                2000.,
            ))),
        ),
        MeshMaterial2d(
            materials_2d.add(Color::from(BLUE_800)),
        ),
        Transform::default(),
        RenderLayers::layer(1),
    ));

    let sphere =
        meshes.add(Sphere::new(0.2).mesh().uv(32, 18));

    let joint_material = MeshMaterial3d(
        materials.add(Color::hsl(100., 0.95, 0.7)),
    );

    let x = commands
        .spawn((
            Mesh3d(sphere.clone()),
            joint_material.clone(),
            Transform::from_xyz(0., 0., 2.),
        ))
        .id();
    let y = commands
        .spawn((
            Mesh3d(sphere.clone()),
            joint_material.clone(),
            Transform::from_xyz(0., 1., 0.),
        ))
        .id();
    let z = commands
        .spawn((
            Mesh3d(sphere.clone()),
            joint_material.clone(),
            Transform::from_xyz(1., 2., 0.),
        ))
        .id();
    commands.insert_resource(Joints(vec![x, y, z]));

    commands.spawn((
        Mesh2d(
            meshes.add(Rectangle::from_size(Vec2::splat(
                2000.,
            ))),
        ),
        MeshMaterial2d(
            materials_2d.add(Color::from(SKY_900)),
        ),
        Transform::default(),
    ));
}

fn update(
    joints: Res<Joints>,
    query: Query<&GlobalTransform>,
    mut gizmos_3d: Gizmos<Gizmos3d>,
    mut gizmos_2d: Gizmos<Gizmos2d>,
) {
    let positions: Vec<Vec3> = joints
        .0
        .iter()
        .map(|entity| {
            query.get(*entity).unwrap().translation()
        })
        .collect();
    // dbg!(&positions);
    for (ja, jb) in positions.iter().tuple_windows() {
        gizmos_3d.line(*ja, *jb, RED_400);
    }

    let bone_length = positions[1].distance(positions[0]);

    // the vector
    let grandchild_vector = positions[1] - positions[2];
    let vector = positions[1] - positions[0];

    // // 3.2: find the projection O of the target t on line L1
    let o = vector.project_onto(grandchild_vector);
    gizmos_3d.line(
        positions[1],
        positions[1] - o,
        BLUE_400,
    );

    // // 3.3: find the distance between O and the relevant joint
    let s = o.length();
    // // 3.4: Map the target (rotate and translate) in such a way that O
    // //   is now located at the axis origin and oriented according to
    // //   the x and y-axis ) Now it is a 2D simplified problem
    // // 3.5/3.6: cone so all sectors are the same

    // arbitrary limit
    let qj = s * FRAC_PI_8.tan();

    gizmos_3d
        .primitive_3d(
            &Cone::new(s * FRAC_PI_8, s),
            // Isometry3d::default(),
            Isometry3d {
                // rotation: global_transform.rotation()
                //     * Quat::from_rotation_x(FRAC_PI_2),
                rotation: Quat::from_rotation_arc(
                    Vec3::NEG_Y,
                    (-o).normalize(),
                ),
                translation: (positions[1]
                    - s / 2. * o.normalize())
                .into(),
                ..default()
            },
            YELLOW_400.with_alpha(0.2),
        )
        .resolution(40);

    gizmos_3d
        .circle(
            Isometry3d {
                rotation: Quat::from_rotation_arc(
                    Vec3::NEG_Z,
                    (-o).normalize(),
                ),
                translation: (positions[1] - o).into(),
                ..default()
            },
            qj,
            Color::WHITE,
        )
        .resolution(CIRCLE_GIZMO_RESOLUTION);

    // the cone slice location, which is the origin
    gizmos_2d
        .circle_2d(Isometry2d::default(), 0.01, BLUE_400)
        .resolution(CIRCLE_GIZMO_RESOLUTION);
    gizmos_2d
        .circle_2d(Isometry2d::default(), qj, SLATE_50)
        .resolution(CIRCLE_GIZMO_RESOLUTION);

    // goal is to build the rotation matrix the rotates
    // `projection_target_plane_normal` onto `bevy_xy_plane_normal`,
    // which can then be applied to the target points
    let projection_target_plane_normal = o.normalize();
    let bevy_xy_plane_normal = Vec3::Z;

    // rotation axis
    let axis = projection_target_plane_normal
        .cross(bevy_xy_plane_normal);
    // rotation angle
    let angle = projection_target_plane_normal
        .angle_between(bevy_xy_plane_normal);
    // rotation matrix
    let mat = Mat3::from_axis_angle(axis, angle);
    // the global position of the vector projection
    let projected_endpoint_from_origin = positions[1] - o;
    // the target position, rotated and translated to exist
    // on Bevy's 2d, xy, plane with o as the origin
    let target_position_on_2d_plane = mat
        * (positions[0] - projected_endpoint_from_origin);

    // 2d target position, before mapping to cone slice
    gizmos_2d
        .circle_2d(
            Isometry2d::from_translation(
                target_position_on_2d_plane.xy(),
            ),
            0.1,
            RED_400,
        )
        .resolution(CIRCLE_GIZMO_RESOLUTION);

    // // 3.8: Check whether the target is within the conic section or not
    if target_position_on_2d_plane.xy().length() > qj {
        // The target position, moved to exist on the cone slice
        let new_target_position_2d =
            (target_position_on_2d_plane.xy()).normalize()
                * qj;
        gizmos_2d
            .circle_2d(
                Isometry2d::from_translation(
                    new_target_position_2d,
                ),
                0.1,
                GREEN_400,
            )
            .resolution(CIRCLE_GIZMO_RESOLUTION);

        // the vector the target took to move to the circle
        gizmos_2d.arrow_2d(
            target_position_on_2d_plane.xy(),
            new_target_position_2d,
            GREEN_400,
        );

        // new_3d_position of target, generated by inverting
        // the rotation and translation we applied before
        let position_before_bone_length = mat.inverse()
            * new_target_position_2d.extend(0.)
            + projected_endpoint_from_origin;

        // joint position before applying the bone_length
        gizmos_3d
            .circle(
                Isometry3d::from_translation(
                    position_before_bone_length,
                ),
                0.1,
                GREEN_400,
            )
            .resolution(CIRCLE_GIZMO_RESOLUTION);

        // movement of the target joint, in 3d.
        // Represents the same movement as the 2d green vector
        gizmos_3d.arrow(
            positions[0],
            position_before_bone_length,
            GREEN_400,
        );

        // extend bone_length vector to show final position
        let direction = (position_before_bone_length
            - positions[1])
            .normalize();
        gizmos_3d.line(
            positions[1],
            positions[1] + direction * bone_length,
            INDIGO_400,
        );

        gizmos_3d
            .circle(
                Isometry3d::from_translation(
                    positions[1] + direction * bone_length,
                ),
                0.1,
                INDIGO_400,
            )
            .resolution(CIRCLE_GIZMO_RESOLUTION);
    }
}

fn update_cameras(
    mut query: Query<(
        &mut Camera,
        Has<Camera3d>,
        Has<Camera2d>,
    )>,
    window: Single<&Window>,
) {
    for (mut camera, _is_3d, is_2d) in &mut query {
        let window = window.resolution.physical_size();
        let cam = camera.viewport.as_mut().unwrap();

        cam.physical_size =
            UVec2::new(window.x / 2, window.y);
        if is_2d {
            cam.physical_position.x = window.x / 2;
        }
    }
}

fn on_drag(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
) {
    let Ok(mut transform) = transforms.get_mut(drag.entity)
    else {
        return;
    };
    transform.translation.x += drag.delta.x * 0.01;
    transform.translation.y -= drag.delta.y * 0.01;
}
