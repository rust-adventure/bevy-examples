use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::{
        mesh::VertexAttributeValues, render_resource::Face,
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("071f3c").unwrap(),
        ))
        // .insert_resource(AssetServerSettings {
        //     watch_for_changes: true,
        //     ..default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(MaterialPlugin::<
            candy_cane::StandardMaterial,
        >::default())
        .add_startup_system(setup)
        .add_system(change_color)
        .add_system(animate_light_direction)
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let mut mesh = Mesh::from(shape::UVSphere {
    //     radius: 1.0,
    //     ..default()
    // });
    // let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
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

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(
            candy_cane::StandardMaterial {
                base_color: Color::rgb(0.533, 0.533, 0.80),
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

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(
                -2.0, 10.0, 15.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Movable);

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
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
    mut materials: ResMut<
        Assets<candy_cane::StandardMaterial>,
    >,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        // material.1.base_color = Color::rgb(0.4,0.4,0.4);
        material.1.time =
            time.seconds_since_startup() as f32;
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
