//! An example showing how to save screenshots to
//! disk

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::screenshot::ScreenshotManager,
    },
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
    },
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::{
    AssetInspectorPlugin, ResourceInspectorPlugin,
    WorldInspectorPlugin,
};
use bevy_shader_utils::ShaderUtilsPlugin;

#[derive(Component)]
struct ExampleName;
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                ShaderUtilsPlugin,
                Material2dPlugin::<
                    ScreenshotPerlin2dMaterial,
                >::default(),
                MaterialPlugin::<
                    ScreenshotPerlin3dMaterial,
                >::default(),
                Material2dPlugin::<
                    ScreenshotSimplex2dMaterial,
                >::default(),
                MaterialPlugin::<ScreenshotSimplex3dMaterial>::default(),
                // Material2dPlugin::<ScreenshotFresnelMaterial>::default(),
                Material2dPlugin::<
                    ScreenshotVoronoiseMaterial,
                >::default(),
            ),
        )
        .add_plugins((
            // AssetInspectorPlugin::<
            //     ScreenshotPerlin2dMaterial,
            // >::default(),
            // AssetInspectorPlugin::<
            //     ScreenshotPerlin3dMaterial,
            // >::default(),
            // AssetInspectorPlugin::<
            //     ScreenshotSimplex2dMaterial,
            // >::default(),
            // AssetInspectorPlugin::<
            //     ScreenshotSimplex3dMaterial,
            // >::default(),
            // AssetInspectorPlugin::<
            //     ScreenshotFresnelMaterial,
            // >::default(),
            // AssetInspectorPlugin::<
            //     ScreenshotVoronoiseMaterial,
            // >::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                screenshot_on_spacebar,
                example_navigation,
            ),
        )
        .run();
}

#[derive(Resource)]
struct Examples(Vec<Example>);

struct Example {
    camera_type: CameraType,
    entity: Entity,
}

enum CameraType {
    TwoD,
    ThreeD,
}

fn example_navigation(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    examples: Option<Res<Examples>>,
    mut example_index: Local<u32>,
    names: Query<&Name>,
    mut name_text: Query<&mut Text, With<ExampleName>>,
    active_cameras: Query<Entity, With<ActiveCamera>>,
) {
    let Some(examples) = examples else { return };
    if input.just_pressed(KeyCode::Left) {
        match example_index.checked_sub(1) {
            Some(_) => {
                *example_index -= 1;
            }
            None => {
                *example_index =
                    (examples.0.len() - 1) as u32;
            }
        }
    }
    if input.just_pressed(KeyCode::Right) {
        *example_index += 1;
    }
    for example in examples.0.iter() {
        commands
            .entity(example.entity)
            .insert(Visibility::Hidden);
    }
    let entity_to_display =
        examples.0[*example_index as usize].entity;
    commands
        .entity(entity_to_display)
        .insert(Visibility::Visible);
    if let Ok(name) = names.get(entity_to_display) {
        for mut text in name_text.iter_mut() {
            text.sections[0].value = name.to_string();
        }
    }

    for entity in active_cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
    match examples.0[*example_index as usize].camera_type {
        CameraType::TwoD => {
            commands.spawn((
                Camera2dBundle::default(),
                ActiveCamera,
            ));
        }
        CameraType::ThreeD => {
            commands.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(
                        -2.0, 2.5, 5.0,
                    )
                    .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                ActiveCamera,
            ));
        }
    }
}

fn screenshot_on_spacebar(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    name_text: Query<&Text, With<ExampleName>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!(
            "./screenshots/{}.png",
            name_text.single().sections[0].value
        );
        screenshot_manager
            .save_screenshot_to_disk(
                main_window.single(),
                path,
            )
            .unwrap();
    }
}

#[derive(Component)]
struct ActiveCamera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut screenshot_perlin2d_materials: ResMut<
        Assets<ScreenshotPerlin2dMaterial>,
    >,
    mut screenshot_perlin3d_materials: ResMut<
        Assets<ScreenshotPerlin3dMaterial>,
    >,
    mut screenshot_simplex2d_materials: ResMut<
        Assets<ScreenshotSimplex2dMaterial>,
    >,
    mut screenshot_simplex3d_materials: ResMut<
        Assets<ScreenshotSimplex3dMaterial>,
    >,
    // mut screenshot_fresnel_materials: ResMut<
    //     Assets<ScreenshotFresnelMaterial>,
    // >,
    mut screenshot_voronoise_materials: ResMut<
        Assets<ScreenshotVoronoiseMaterial>,
    >,
) {
    let mut entities = vec![];

    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material: screenshot_perlin2d_materials
                        .add(ScreenshotPerlin2dMaterial {
                            scale: 50.0,
                        }),
                    visibility: Visibility::Visible,
                    ..default()
                },
                Name::from("perlin-2d"),
            ))
            .id(),
    });

    commands
        .spawn((Camera2dBundle::default(), ActiveCamera));
    entities.push(Example {
        camera_type: CameraType::ThreeD,
        entity: 
        // cube
        commands
            .spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(
                        shape::Cube { size: 2.0 },
                    )),
                    material: screenshot_perlin3d_materials
                        .add(ScreenshotPerlin3dMaterial {
                            scale: 5.0,
                        }),
                    transform: Transform::from_xyz(
                        0.0, 0.5, 0.0,
                    ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("perlin-3d"),
            ))
            .id()});
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material:
                        screenshot_simplex2d_materials.add(
                            ScreenshotSimplex2dMaterial {
                                scale: 50.0,
                            },
                        ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("simplex-2d"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::ThreeD,
        entity: 
        // cube
        commands
            .spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(
                        shape::Cube { size: 2.0 },
                    )),
                    material:
                        screenshot_simplex3d_materials.add(
                            ScreenshotSimplex3dMaterial {
                                scale: 5.0,
                            },
                        ),
                    transform: Transform::from_xyz(
                        0.0, 0.5, 0.0,
                    ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("simplex-3d"),
            ))
            .id()});

    // entities.push(
    //     // cube
    //     commands
    //         .spawn(PbrBundle {
    //             mesh:
    // meshes.add(Mesh::from(shape::Cube {
    //                 size: 1.0,
    //             })),
    //             material:
    // screenshot_fresnel_materials               
    // .add(ScreenshotFresnelMaterial {}),
    //             transform: Transform::from_xyz(
    //                 0.0, 0.5, 0.0,
    //             ),
    //             visibility: Visibility::Hidden,
    //             ..default()
    //         })
    //         .id(),
    // );

    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material:
                        screenshot_voronoise_materials.add(
                            ScreenshotVoronoiseMaterial {
                                x: 1.0,
                                y: 0.0,
                                scale: 250.0,
                            },
                        ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material:
                        screenshot_voronoise_materials.add(
                            ScreenshotVoronoiseMaterial {
                                x: 1.0,
                                y: 1.0,
                                scale: 250.0,
                            },
                        ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material:
                        screenshot_voronoise_materials.add(
                            ScreenshotVoronoiseMaterial {
                                x: 0.0,
                                y: 0.0,
                                scale: 250.0,
                            },
                        ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(
                            shape::Quad::default(),
                        ))
                        .into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(4000.)),
                    material:
                        screenshot_voronoise_materials.add(
                            ScreenshotVoronoiseMaterial {
                                x: 0.0,
                                y: 1.0,
                                scale: 250.0,
                            },
                        ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::from("voronoise"),
            ))
            .id(),
    });

    commands.insert_resource(Examples(entities));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section(
            "Press <spacebar> to save a screenshot to disk",
            TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ExampleName,
    ));
}

// Set up materials
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "08063870-7da9-4b79-b9b7-6eeb904222ed"]
pub struct ScreenshotPerlin2dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material2d for ScreenshotPerlin2dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_perlin2d_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "a4704519-fd4f-4cb0-a96a-be86901ab101"]
pub struct ScreenshotPerlin3dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for ScreenshotPerlin3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_perlin3d_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "f2e98d09-230c-45a3-ba42-8b5da5642f36"]
pub struct ScreenshotSimplex2dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material2d for ScreenshotSimplex2dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_simplex2d_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "848b7711-3819-4525-bbba-91b474303778"]
pub struct ScreenshotSimplex3dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for ScreenshotSimplex3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_simplex3d_material.wgsl".into()
    }
}

// #[derive(AsBindGroup, TypeUuid,  Debug, Clone,
// Reflect)]
// #[uuid = "0c0bf6dc-492b-43d4-89b0-70f2c5ae2166"
// ] pub struct ScreenshotFresnelMaterial {}

// impl Material2d for ScreenshotFresnelMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/screenshot_fresnel_material.
// wgsl".into()     }
// }

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect)]
#[uuid = "3177e91c-c3db-4b62-bf9b-b2e50c81e3f4"]
pub struct ScreenshotVoronoiseMaterial {
    #[uniform(0)]
    x: f32,
    #[uniform(0)]
    y: f32,
    #[uniform(0)]
    scale: f32,
}

impl Material2d for ScreenshotVoronoiseMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_voronoise_material.wgsl".into()
    }
}
