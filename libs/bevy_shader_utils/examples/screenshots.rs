// An example used to save screenshots of the materials to disk
use bevy::{
     prelude::*, render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::screenshot::{
            save_to_disk, Capturing, Screenshot,
        },
    }, sprite::{Material2d, Material2dPlugin}, window::SystemCursorIcon, winit::cursor::CursorIcon
};
// use bevy_inspector_egui::quick::{
//     AssetInspectorPlugin,
// ResourceInspectorPlugin,
//     WorldInspectorPlugin,
// };
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
        // .add_plugins((
        //     AssetInspectorPlugin::<
        //         ScreenshotPerlin2dMaterial,
        //     >::default().run_if(input_toggle_active(false, KeyCode::Digit1)),
        //     AssetInspectorPlugin::<
        //         ScreenshotPerlin3dMaterial,
        //     >::default().run_if(input_toggle_active(false, KeyCode::Digit2)),
        //     AssetInspectorPlugin::<
        //         ScreenshotSimplex2dMaterial,
        //     >::default().run_if(input_toggle_active(false, KeyCode::Digit3)),
        //     AssetInspectorPlugin::<
        //         ScreenshotSimplex3dMaterial,
        //     >::default().run_if(input_toggle_active(false, KeyCode::Digit4)),
        //     // AssetInspectorPlugin::<
        //     //     ScreenshotFresnelMaterial,
        //     // >::default(),
        //     AssetInspectorPlugin::<
        //         ScreenshotVoronoiseMaterial,
        //     >::default().run_if(input_toggle_active(false, KeyCode::Digit5)),
        // ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                screenshot_on_spacebar,
                screenshot_saving,
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
    input: Res<ButtonInput<KeyCode>>,
    examples: Option<Res<Examples>>,
    mut example_index: Local<u32>,
    names: Query<&Name>,
    mut name_text: Query<&mut Text, With<ExampleName>>,
    active_cameras: Query<Entity, With<ActiveCamera>>,
) {
    let Some(examples) = examples else { return };
    if input.just_pressed(KeyCode::ArrowLeft) {
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
    if input.just_pressed(KeyCode::ArrowRight) {
        *example_index += 1;
        if *example_index == examples.0.len() as u32 {
            *example_index = 0;
        }
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
            text.0 = name.to_string();
        }
    }

    for entity in active_cameras.iter() {
        commands.entity(entity).despawn();
    }
    match examples.0[*example_index as usize].camera_type {
        CameraType::TwoD => {
            commands
                .spawn((Camera2d::default(), ActiveCamera));
        }
        CameraType::ThreeD => {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(-2.0, 2.5, 5.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ActiveCamera,
            ));
        }
    }
}

fn screenshot_on_spacebar(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    name_text: Single<&Text, With<ExampleName>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!(
            "./screenshots/{}.png",
            name_text.0
        );

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    windows: Query<Entity, With<Window>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands.entity(window).insert(
                CursorIcon::from(
                    SystemCursorIcon::Progress,
                ),
            );
        }
        _ => {}
    }
}

#[derive(Component)]
struct ActiveCamera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
    println!("press digit to bring up inspector, arrow key to navigate");

    let mut entities = vec![];

    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_perlin2d_materials.add(
                        ScreenshotPerlin2dMaterial {
                            scale: 50.0,
                        },
                    ),
                ),
                Visibility::Visible,
                Name::from("perlin-2d"),
            ))
            .id(),
    });

    commands.spawn((Camera2d::default(), ActiveCamera));
    entities.push(Example {
        camera_type: CameraType::ThreeD,
        entity: 
        // cube
        commands
            .spawn((

                    Mesh3d(meshes.add(Cuboid{ half_size: Vec3::splat(1.) })),
                    MeshMaterial3d(screenshot_perlin3d_materials
                        .add(ScreenshotPerlin3dMaterial {
                            scale: 5.0,
                        })),
                    Transform::from_xyz(
                        0.0, 0.5, 0.0,
                    ),
                 Visibility::Hidden,
                Name::from("perlin-3d"),
            ))
            .id()});
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_simplex2d_materials.add(
                        ScreenshotSimplex2dMaterial {
                            scale: 50.0,
                        },
                    ),
                ),
                Visibility::Hidden,
                Name::from("simplex-2d"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::ThreeD,
        entity: 
        commands
            .spawn((
                Mesh3d(meshes.add(Cuboid{ half_size: Vec3::splat(1.) })),
                    MeshMaterial3d(
                        screenshot_simplex3d_materials.add(
                            ScreenshotSimplex3dMaterial {
                                scale: 5.0,
                            },
                        )),
                     Transform::from_xyz(
                        0.0, 0.5, 0.0,
                    ),
                     Visibility::Hidden,


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
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_voronoise_materials.add(
                        ScreenshotVoronoiseMaterial {
                            x: 1.0,
                            y: 0.0,
                            scale: 250.0,
                        },
                    ),
                ),
                Visibility::Hidden,
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_voronoise_materials.add(
                        ScreenshotVoronoiseMaterial {
                            x: 1.0,
                            y: 1.0,
                            scale: 250.0,
                        },
                    ),
                ),
                Visibility::Hidden,
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_voronoise_materials.add(
                        ScreenshotVoronoiseMaterial {
                            x: 0.0,
                            y: 0.0,
                            scale: 250.0,
                        },
                    ),
                ),
                Visibility::Hidden,
                Name::from("voronoise"),
            ))
            .id(),
    });
    entities.push(Example {
        camera_type: CameraType::TwoD,
        entity: commands
            .spawn((
                Mesh2d(
                    meshes.add(Rectangle::default()).into(),
                ),
                Transform::default()
                    .with_scale(Vec3::splat(4000.)),
                MeshMaterial2d(
                    screenshot_voronoise_materials.add(
                        ScreenshotVoronoiseMaterial {
                            x: 0.0,
                            y: 1.0,
                            scale: 250.0,
                        },
                    ),
                ),
                Visibility::Hidden,
                Name::from("voronoise"),
            ))
            .id(),
    });

    commands.insert_resource(Examples(entities));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Text::new(
            "Press <spacebar> to save a screenshot to disk",
        ),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new(""),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ExampleName,
    ));
}

// Set up materials
#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct ScreenshotPerlin2dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material2d for ScreenshotPerlin2dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_perlin2d_material.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct ScreenshotPerlin3dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for ScreenshotPerlin3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_perlin3d_material.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct ScreenshotSimplex2dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material2d for ScreenshotSimplex2dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_simplex2d_material.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct ScreenshotSimplex3dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for ScreenshotSimplex3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_simplex3d_material.wgsl".into()
    }
}

// #[derive(Asset, AsBindGroup,  Debug, Clone,
// Reflect)]
// ] pub struct ScreenshotFresnelMaterial {}

// impl Material2d for ScreenshotFresnelMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/screenshot_fresnel_material.
// wgsl".into()     }
// }

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
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
