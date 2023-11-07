use bevy::{
    core_pipeline::{
        fxaa::Fxaa,
        prepass::{
            DepthPrepass, MotionVectorPrepass,
            NormalPrepass,
        },
    },
    pbr::{ExtendedMaterial, OpaqueRendererMethod},
    prelude::*,
    render::mesh::VertexAttributeValues,
};

use bevy_prepass_debug::PrepassDebugPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use dissolve_sphere_standard_material_extension::DissolveExtension;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.02,
        })
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(
            Color::hex("1fa9f4").unwrap(),
        ))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
            ShaderUtilsPlugin,
            PrepassDebugPlugin,
            MaterialPlugin::<
                ExtendedMaterial<
                    StandardMaterial,
                    DissolveExtension,
                >,
            >::default(),
        ))
        .add_systems(Startup, setup)
        // .add_system(change_color)
        .add_systems(
            Update,
            (animate_light_direction, movement),
        )
        .run();
}

#[derive(Component)]
struct Cube;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut dissolve_materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                DissolveExtension,
            >,
        >,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mesh = Mesh::from(shape::UVSphere {
        radius: 1.0,
        ..default()
    });
    // let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
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
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: dissolve_materials
            .add(ExtendedMaterial {
            base: StandardMaterial {
                // base_color: Color::rgb(0.533, 0.533, 0.80),
                base_color: Color::WHITE,
                // base_color: Color::YELLOW,
                base_color_texture: Some(
                    asset_server.load(
                        "concrete/sekjcawb_2K_Albedo.jpg",
                    ),
                ),
                normal_map_texture: Some(
                    asset_server.load(
                        "concrete/sekjcawb_2K_Normal.jpg",
                    ),
                ),
                double_sided: true,
                cull_mode: None,
                // can be used in forward or deferred mode.
                opaque_render_method:
                    OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                ..default()
            },
            extension: DissolveExtension {
                // quantize_steps: 3,
            },
        }),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Movable,
        DepthPrepass,
        NormalPrepass,
        MotionVectorPrepass,
        Fxaa::default(),
    ));
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 10.0,
            ..default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });
    // // left wall
    // let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    // transform.rotate_z(std::f32::consts::FRAC_PI_2);
    // commands.spawn(PbrBundle {
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
    // commands.spawn(PbrBundle {
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

    // red point light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(1.0, 2.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(
                    shape::UVSphere {
                        radius: 0.1,
                        ..default()
                    },
                )),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(
                        100.0, 0.0, 0.0, 0.0,
                    ),
                    ..default()
                }),
                ..default()
            });
        });

    // blue point light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(
                    shape::UVSphere {
                        radius: 0.1,
                        ..default()
                    },
                )),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    emissive: Color::rgba_linear(
                        0.0, 0.0, 100.0, 0.0,
                    ),
                    ..default()
                }),
                ..default()
            });
        });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 20.0, 10.0),
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

// fn change_color(
//     mut materials: ResMut<Assets<dissolve_sphere_standard_material_extension::StandardMaterial>>,
//     time: Res<Time>,
// ) {
//     for material in materials.iter_mut() {
//         // material.1.base_color = Color::rgb(0.4,0.4,0.4);
//         material.1.time = time.elapsed_seconds();
//     }
// }

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
