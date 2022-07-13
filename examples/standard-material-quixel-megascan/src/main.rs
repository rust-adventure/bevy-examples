use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("071f3c").unwrap(),
        ))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(change_color)
        .add_system(animate_light_direction)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Cube {
    id: Handle<Image>,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut custom_materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let normal_map_id: Handle<bevy::prelude::Image> =
        asset_server
            .load("concrete/sekjcawb_2K_Normal.jpg");
    let id: Handle<bevy::prelude::Image> = asset_server
        .load("concrete/sekjcawb_2K_Roughness.jpg");

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            base_color_texture: Some(
                asset_server.load(
                    "concrete/sekjcawb_2K_Albedo.jpg",
                ),
            ),
            normal_map_texture: Some(normal_map_id.clone()),
            // double_sided: true,
            // cull_mode: None,
            // alpha_mode: AlphaMode::Blend,
            // perceptual_roughness: 10.0,
            // time: 0.,
            ..default()
        }),
        // material: materials.add(StandardMaterial {
        //     base_color: Color::BLUE,
        //     alpha_mode: AlphaMode::Blend,
        //     ..default()
        // }),
        ..default()
    });
    // .insert(Cube { id: normal_map_id });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Movable);
    // // ground plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes
    //         .add(Mesh::from(shape::Plane { size: 10.0 })),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::WHITE,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });
    // // left wall
    // let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    // transform.rotate_z(std::f32::consts::FRAC_PI_2);
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(
    //         5.0, 0.15, 5.0,
    //     ))),
    //     transform,
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::INDIGO,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });
    // // // back (right) wall
    // let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    // transform.rotate_x(std::f32::consts::FRAC_PI_2);
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(
    //         5.0, 0.15, 5.0,
    //     ))),
    //     transform,
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::INDIGO,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });

    // ambient light
    // commands.insert_resource(AmbientLight {
    //     color: Color::ORANGE_RED,
    //     brightness: 0.02,
    // });

    // red point light
    // commands
    //     .spawn_bundle(PointLightBundle {
    //         // transform: Transform::from_xyz(5.0, 8.0, 2.0),
    //         transform: Transform::from_xyz(1.0, 2.0, 0.0),
    //         point_light: PointLight {
    //             intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
    //             color: Color::RED,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn_bundle(PbrBundle {
    //             mesh: meshes.add(Mesh::from(
    //                 shape::UVSphere {
    //                     radius: 0.1,
    //                     ..default()
    //                 },
    //             )),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::RED,
    //                 emissive: Color::rgba_linear(
    //                     100.0, 0.0, 0.0, 0.0,
    //                 ),
    //                 ..default()
    //             }),
    //             ..default()
    //         });
    //     });

    // blue point light
    // commands
    //     .spawn_bundle(PointLightBundle {
    //         // transform: Transform::from_xyz(5.0, 8.0, 2.0),
    //         transform: Transform::from_xyz(0.0, 4.0, 0.0),
    //         point_light: PointLight {
    //             intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
    //             color: Color::BLUE,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn_bundle(PbrBundle {
    //             mesh: meshes.add(Mesh::from(
    //                 shape::UVSphere {
    //                     radius: 0.1,
    //                     ..default()
    //                 },
    //             )),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::BLUE,
    //                 emissive: Color::rgba_linear(
    //                     0.0, 0.0, 100.0, 0.0,
    //                 ),
    //                 ..default()
    //             }),
    //             ..default()
    //         });
    //     });

    // directional 'sun' light
    const HALF_SIZE: f32 = 20.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_4,
            ),
            ..default()
        },
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        With<DirectionalLight>,
    >,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn change_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        // material.1.base_color = Color::rgb(0.4,0.4,0.4);
        // material.1.time =
        //     time.seconds_since_startup() as f32;
    }
}

#[derive(Component)]
struct Movable;
fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        transform.translation +=
            time.delta_seconds() * 2.0 * direction;
    }
}
