use bevy::{
    prelude::*, render::mesh::VertexAttributeValues,
};
use bevy_shader_utils::ShaderUtilsPlugin;
use candy_cane::Stripe;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("66b1c3").unwrap(),
        ))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(MaterialPlugin::<
            candy_cane::StandardMaterial,
        >::default())
        .add_startup_system(setup)
        .add_system(change_color)
        // .add_system(animate_light_direction)
        .add_system(movement)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<
        Assets<candy_cane::StandardMaterial>,
    >,
) {
    let mut mesh = Mesh::from(shape::Capsule {
        radius: 1.0,
        depth: 10.0,
        ..default()
    });
    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| {
                [
                    (1. - *r) / 2.,
                    (1. - *g) / 2.,
                    (1. - *b) / 2.,
                    1.,
                ]
            })
            .collect();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors,
        );
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh.clone()),
        transform: Transform::from_xyz(-5.0, 0.5, 0.0),
        material: custom_materials.add(
            candy_cane::StandardMaterial {
                stripe_one: Stripe {
                    frequency: 10.0,
                    minimum_value: 0.4,
                    power_value: 100.0,
                    should_use: 1.0,
                },
                base_color: Color::rgb(1.0, 1.0, 1.0),
                // base_color: Color::YELLOW,
                double_sided: true,
                cull_mode: None,
                // alpha_mode: AlphaMode::Blend,
                // perceptual_roughness: 10.0,
                // time: 0.,
                ..default()
            },
        ),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh.clone()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(
            candy_cane::StandardMaterial {
                stripe_one: Stripe {
                    frequency: 10.0,
                    minimum_value: 0.2,
                    power_value: 100.0,
                    should_use: 1.0,
                },
                stripe_two: Stripe {
                    frequency: 10.0,
                    minimum_value: 0.0,
                    power_value: 300.0,
                    should_use: 1.2,
                },
                stripe_three: Stripe {
                    frequency: 10.0,
                    minimum_value: 0.0,
                    power_value: 300.0,
                    should_use: 1.5,
                },
                stripe_four: Stripe {
                    frequency: 10.0,
                    minimum_value: 0.0,
                    power_value: 300.0,
                    should_use: 1.8,
                },
                stripe_color_two: Color::DARK_GREEN,
                stripe_color_three: Color::DARK_GREEN,
                stripe_color_four: Color::DARK_GREEN,
                base_color: Color::rgb(1.0, 1.0, 1.0),
                // base_color: Color::YELLOW,
                double_sided: true,
                cull_mode: None,
                // alpha_mode: AlphaMode::Blend,
                // perceptual_roughness: 10.0,
                // time: 0.,
                ..default()
            },
        ),
        // material: materials.add(StandardMaterial {
        //     base_color: Color::BLUE,
        //     alpha_mode: AlphaMode::Blend,
        //     ..default()
        // }),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh.clone()),
        transform: Transform::from_xyz(5.0, 0.5, 0.0),
        material: custom_materials.add(
            candy_cane::StandardMaterial {
                stripe_one: Stripe {
                    frequency: 5.0,
                    minimum_value: 0.2,
                    power_value: 100.0,
                    should_use: 1.0,
                },
                stripe_two: Stripe {
                    frequency: 5.0,
                    minimum_value: 0.1,
                    power_value: 400.0,
                    should_use: 1.5,
                },
                stripe_three: Stripe {
                    frequency: 20.0,
                    minimum_value: 0.0,
                    power_value: 100.0,
                    should_use: 0.0,
                },
                stripe_four: Stripe {
                    frequency: 20.0,
                    minimum_value: 0.0,
                    power_value: 100.0,
                    should_use: 0.0,
                },
                stripe_five: Stripe {
                    frequency: 20.0,
                    minimum_value: 0.0,
                    power_value: 100.0,
                    should_use: 0.0,
                },
                stripe_color_one: Color::DARK_GREEN,
                stripe_color_two: Color::hex("b1b100")
                    .unwrap(),
                base_color: Color::rgb(1.0, 1.0, 1.0),
                // base_color: Color::YELLOW,
                double_sided: true,
                cull_mode: None,
                // alpha_mode: AlphaMode::Blend,
                // perceptual_roughness: 10.0,
                // time: 0.,
                ..default()
            },
        ),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                -2.0, 10.0, 15.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Movable,
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
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

fn change_color(
    mut materials: ResMut<
        Assets<candy_cane::StandardMaterial>,
    >,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
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

        if input.pressed(KeyCode::R) {
            transform.rotate_around(
                Vec3::from((0.0, 0.5, 0.0)),
                Quat::from_rotation_y(
                    time.delta_seconds()
                        * 2.0
                        * std::f32::consts::FRAC_PI_8,
                ),
            );
        }
    }
}
