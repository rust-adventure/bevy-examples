use bevy::{
    color::palettes::{css::RED, tailwind::GREEN_700},
    pbr::ExtendedMaterial,
    prelude::*,
    render::storage::ShaderStorageBuffer,
};
use bevy_shader_utils::ShaderUtilsPlugin;
use candy_cane::stripe::{
    CandyCaneMaterial, Stripe, StripeMaterialPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("66b1c3").unwrap().into(),
        ))
        .add_plugins((
            DefaultPlugins,
            ShaderUtilsPlugin,
            StripeMaterialPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                CandyCaneMaterial,
            >,
        >,
    >,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let mesh = Mesh::from(Capsule3d {
        radius: 1.0,
        half_length: 5.0,
    });

    // first candy cane
    let data = vec![Stripe {
        frequency: 10.0,
        minimum_value: 0.4,
        power_value: 100.0,
        offset: 0.0,
        color: RED.into(),
    }];
    let stripes_buffer =
        buffers.add(ShaderStorageBuffer::from(data));

    commands.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        Transform::from_xyz(-5.0, 0.5, 0.0),
        MeshMaterial3d(custom_materials.add(
            ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::WHITE,
                    ..default()
                },
                extension: CandyCaneMaterial {
                    stripes_buffer,
                },
            },
        )),
    ));

    // second candy cane
    let data = vec![
        Stripe {
            frequency: 10.0,
            minimum_value: 0.2,
            power_value: 100.0,
            offset: 0.,
            color: RED.into(),
        },
        Stripe {
            frequency: 10.0,
            minimum_value: 0.0,
            power_value: 300.0,
            offset: 1.2,
            color: GREEN_700.into(),
        },
        Stripe {
            frequency: 10.0,
            minimum_value: 0.0,
            power_value: 300.0,
            offset: 1.5,
            color: GREEN_700.into(),
        },
        Stripe {
            frequency: 10.0,
            minimum_value: 0.0,
            power_value: 300.0,
            offset: 1.8,
            color: GREEN_700.into(),
        },
    ];
    let stripes_buffer =
        buffers.add(ShaderStorageBuffer::from(data));

    commands.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(custom_materials.add(
            ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::WHITE,
                    clearcoat: 2.,
                    perceptual_roughness: 0.,
                    ..default()
                },
                extension: CandyCaneMaterial {
                    stripes_buffer,
                },
            },
        )),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // cane 3
    let data = vec![
        Stripe {
            frequency: 5.0,
            minimum_value: 0.2,
            power_value: 100.0,
            offset: 0.,
            color: GREEN_700.into(),
        },
        Stripe {
            frequency: 5.0,
            minimum_value: 0.1,
            power_value: 400.0,
            offset: 1.5,
            color: Srgba::hex("b1b100").unwrap().into(),
        },
    ];
    let stripes_buffer =
        buffers.add(ShaderStorageBuffer::from(data));

    commands.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(custom_materials.add(
            ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::WHITE,
                    ..default()
                },
                extension: CandyCaneMaterial {
                    stripes_buffer,
                },
            },
        )),
        Transform::from_xyz(5.0, 0.5, 0.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 10.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Movable,
    ));

    // ambient light
    commands.insert_resource(AmbientLight { ..default() });

    // directional 'sun' light
    commands.spawn((
        DirectionalLight::default(),
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_4,
            ),
            ..default()
        },
    ));
}

#[derive(Component)]
struct Movable;
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        transform.translation +=
            time.delta_secs() * 2.0 * direction;

        if input.pressed(KeyCode::KeyR) {
            transform.rotate_around(
                Vec3::from((0.0, 0.5, 0.0)),
                Quat::from_rotation_y(
                    time.delta_secs()
                        * 2.0
                        * std::f32::consts::FRAC_PI_8,
                ),
            );
        }
    }
}
