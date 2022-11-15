//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use std::f32::consts::PI;

use bevy::{
    asset::AssetServerSettings,
    core_pipeline::clear_color::ClearColorConfig,
    math::Vec3Swizzles,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        mesh::Indices,
        render_resource::{
            AsBindGroup, Extent3d, PrimitiveTopology,
            ShaderRef, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
    },
    window::{WindowId, WindowResized},
};
use noise::{NoiseFn, Perlin};

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(
        Material2dPlugin::<PostProcessingMaterial>::default(
        ),
    )
    // .add_plugin(post_processing::PostProcessingPlugin)
    .add_startup_system(setup)
    .add_system(update_image_to_window_size)
    .add_system(main_camera_cube_rotator_system)
    // .add_system(update_material)
    .add_system(movement);

    app.run();
}

/// Marks the first camera cube (rendered to a texture.)
#[derive(Component)]
struct MainCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut images: ResMut<Assets<Image>>,
    mut post_processing_materials: ResMut<
        Assets<PostProcessingMaterial>,
    >,
) {
    asset_server.watch_for_changes().unwrap();

    let cube_count = 30;
    for z in 0..cube_count {
        for x in 0..cube_count {
            let perlin = Perlin::new();
            let val = perlin
                .get([x as f64 * 2.2, z as f64 * 2.2]);
            if val > 0.0 {
                let cube_handle =
                    meshes.add(Mesh::from(shape::Cube {
                        size: val as f32,
                    }));
                let cube_material_handle =
                    materials.add(StandardMaterial {
                        base_color: Color::rgb(
                            x as f32 / cube_count as f32,
                            0.5,
                            z as f32 / cube_count as f32,
                        ),
                        reflectance: 0.02,
                        unlit: false,
                        ..default()
                    });

                // The cube that will be rendered to the texture.
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: cube_handle,
                        material: cube_material_handle,
                        transform:
                            Transform::from_translation(
                                Vec3::new(
                                    x as f32
                                        - cube_count as f32
                                            / 2.0,
                                    val as f32 / 2.0,
                                    z as f32
                                        - cube_count as f32
                                            / 2.0,
                                ),
                            ),
                        ..default()
                    })
                    .insert(MainCube);
            }
        }
    }
    let plane_handle = meshes
        .add(Mesh::from(shape::Plane { size: 100.0 }));
    let plane_material_handle =
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.9, 0.9, 0.8),
            reflectance: 0.02,
            unlit: false,
            ..default()
        });
    commands.spawn_bundle(PbrBundle {
        mesh: plane_handle,
        material: plane_material_handle,
        // transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

    // Light
    // NOTE: Currently lights are ignoring render layers - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            0.0, 0.0, 10.0,
        )),
        ..default()
    });

    let window = windows.primary();
    let window_id = window.id();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..Default::default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..Default::default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_two = image.clone();

    let player_one_camera = images.add(image);
    let player_two_camera = images.add(image_two);

    let post_processing_material =
        post_processing_materials.add(
            PostProcessingMaterial {
                player_one_image: player_one_camera.clone(),
                player_two_image: player_two_camera.clone(),
                player_one_position: Vec2::new(0.0, 0.0),
                player_two_position: Vec2::new(0.0, 0.0),
            },
        );

    let player_one_handle =
        meshes.add(Mesh::from(shape::Capsule {
            radius: 0.2,
            depth: 1.0,
            ..default()
        }));
    let player_one_material_handle =
        materials.add(StandardMaterial {
            base_color: Color::RED,
            reflectance: 0.02,
            unlit: false,
            ..default()
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: player_one_handle,
            material: player_one_material_handle,
            transform: Transform::from_translation(
                Vec3::new(0.0, 0.5, 0.0),
            ),
            ..default()
        })
        .insert(Player(1))
        .insert(Character);

    let player_two_handle =
        meshes.add(Mesh::from(shape::Capsule {
            radius: 0.2,
            depth: 1.0,
            ..default()
        }));
    let player_two_material_handle =
        materials.add(StandardMaterial {
            base_color: Color::GREEN,
            reflectance: 0.02,
            unlit: false,
            ..default()
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: player_two_handle,
            material: player_two_material_handle,
            transform: Transform::from_translation(
                Vec3::new(0.0, 0.5, 0.0),
            ),
            ..default()
        })
        .insert(Player(2))
        .insert(Character);

    commands
        .spawn_bundle(Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(
                    player_one_camera.clone(),
                ),
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    Color::WHITE,
                ),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0.0, 3.0, 2.0),
            )
            .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(FitToWindowSize {
            image: player_one_camera.clone(),
            material: post_processing_material.clone(),
            window_id,
        })
        .insert(post_processing_material.clone())
        .insert(Player(1))
        .insert(PlayerCamera);

    commands
        .spawn_bundle(Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(
                    player_two_camera.clone(),
                ),
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    Color::WHITE,
                ),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0.0, 3.0, 2.0),
            )
            .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(FitToWindowSize {
            image: player_two_camera.clone(),
            material: post_processing_material.clone(),
            window_id,
        })
        .insert(post_processing_material.clone())
        .insert(Player(2))
        .insert(PlayerCamera);

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d fullscreen triangle.
    let post_processing_pass_layer = RenderLayers::layer(
        (RenderLayers::TOTAL_LAYERS - 1) as u8,
    );
    let half_extents = Vec2::new(
        size.width as f32 / 2f32,
        size.height as f32 / 2f32,
    );
    let mut triangle_mesh =
        Mesh::new(PrimitiveTopology::TriangleList);
    // NOTE: positions are actually not used because the vertex shader maps UV and clip space.
    triangle_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-half_extents.x, -half_extents.y, 0.0],
            [half_extents.x * 3f32, -half_extents.y, 0.0],
            [-half_extents.x, half_extents.y * 3f32, 0.0],
        ],
    );
    triangle_mesh
        .set_indices(Some(Indices::U32(vec![0, 1, 2])));
    triangle_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    );

    triangle_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[2.0, 0.0], [0.0, 2.0], [0.0, 0.0]],
    );
    let triangle_handle = meshes.add(triangle_mesh);

    // Post processing 2d fullscreen triangle, with material using the render texture done by the main camera, with a custom shader.
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: triangle_handle.into(),
            material: post_processing_material,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(post_processing_pass_layer);

    // The post-processing pass camera.
    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                priority: 10,
                ..Default::default()
            },
            ..Camera2dBundle::default()
        })
        .insert(post_processing_pass_layer);
}

/// Rotates the cube rendered by the main camera
fn main_camera_cube_rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(0.55 * time.delta_seconds());
        // transform.rotate_z(0.15 * time.delta_seconds());
    }
}

#[derive(Component)]
struct PlayerCamera;

/// To support window resizing, this fits an image to a windows size.
#[derive(Component)]
struct FitToWindowSize {
    image: Handle<Image>,
    material: Handle<PostProcessingMaterial>,
    window_id: WindowId,
}

/// Update image size to fit window
fn update_image_to_window_size(
    windows: Res<Windows>,
    mut image_events: EventWriter<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    mut post_processing_materials: ResMut<
        Assets<PostProcessingMaterial>,
    >,
    mut resize_events: EventReader<WindowResized>,
    fit_to_window_size: Query<&FitToWindowSize>,
) {
    for resize_event in resize_events.iter() {
        for fit_to_window in fit_to_window_size.iter() {
            if resize_event.id == fit_to_window.window_id {
                let size = {
                    let window = windows.get(fit_to_window.window_id).expect("PostProcessingCamera is rendering to a window, but this window could not be found");
                    Extent3d {
                        width: window.physical_width(),
                        height: window.physical_height(),
                        ..Default::default()
                    }
                };
                let image = images.get_mut(&fit_to_window.image).expect(
                    "FitToWindowSize is referring to an Image, but this Image could not be found",
                );
                info!("resize to {:?}", size);
                image.resize(size);
                // Hack because of https://github.com/bevyengine/bevy/issues/5595
                image_events.send(AssetEvent::Modified {
                    handle: fit_to_window.image.clone(),
                });
                post_processing_materials
                    .get_mut(&fit_to_window.material);
            }
        }
    }
}

// fn update_material(
//     characters: Query<
//         (&Player, &Transform),
//         (With<Character>, With<Player>),
//     >,
//     mut cameras: Query<
//         (
//             &Player,
//             &mut Transform,
//             &Handle<PostProcessingMaterial>,
//         ),
//         Without<Character>,
//     >,
//     mut materials: ResMut<Assets<PostProcessingMaterial>>,
// ) {
//     for (player, mut camera_transform, handle) in
//         &mut cameras
//     {
//         let mut mat = materials.get_mut(handle).unwrap();

//         for (p, transform) in &characters {
//             match p.0 {
//                 1 => {
//                     mat.player_one_position = Vec2::new(
//                         transform.translation.x,
//                         transform.translation.z,
//                     );
//                 }
//                 2 => {
//                     mat.player_two_position = Vec2::new(
//                         transform.translation.x,
//                         transform.translation.z,
//                     );
//                 }
//                 _ => panic!("ehhhhh"),
//             }
//         }
//     }
// }

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    player_one_image: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    player_two_image: Handle<Image>,

    #[uniform(4)]
    player_one_position: Vec2,
    #[uniform(5)]
    player_two_position: Vec2,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_chromatic_aberration.wgsl"
            .into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/screen_vertex.wgsl".into()
    }
}

#[derive(Component)]
struct Player(u32);

#[derive(Component)]
struct Character;

fn movement(
    mut players: Query<
        (&mut Transform, &Player),
        With<Character>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<
        (
            &Player,
            &mut Transform,
            &Handle<PostProcessingMaterial>,
        ),
        (Without<Character>, With<PlayerCamera>),
    >,
    mut materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    let mut player_one_translation = Vec3::ZERO;
    let mut player_two_translation = Vec3::ZERO;

    for (mut transform, player) in &mut players {
        match player.0 {
            1 => {
                if keyboard_input.pressed(KeyCode::W) {
                    transform.translation.z +=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::A) {
                    transform.translation.x -=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::S) {
                    transform.translation.z -=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::D) {
                    transform.translation.x +=
                        0.2 * time.delta_seconds();
                }
                player_one_translation =
                    transform.translation.clone();
            }
            2 => {
                if keyboard_input.pressed(KeyCode::I) {
                    transform.translation.z +=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::J) {
                    transform.translation.x -=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::K) {
                    transform.translation.z -=
                        0.2 * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::L) {
                    transform.translation.x +=
                        0.2 * time.delta_seconds();
                }
                player_two_translation =
                    transform.translation.clone();
            }
            _ => panic!("no such player"),
        }
        // trans
    }

    for (player, mut camera_transform, handle) in
        &mut cameras
    {
        let mut mat = materials.get_mut(handle).unwrap();

        let xz1 = player_one_translation.xz();
        let xz2 = player_two_translation.xz();
        let distance_to_midpoint = xz1.distance(xz2) / 2.0;

        let distance_from_screen_center = 2.0
            * ((distance_to_midpoint / 3.8).atan() / PI);

        match player.0 {
            1 => {
                let xz = xz1
                    + (xz1 - xz2).normalize()
                        * distance_from_screen_center;
                mat.player_one_position = (xz1 - xz2)
                    .normalize()
                    * distance_from_screen_center;
                camera_transform.translation.x = xz.x;
                camera_transform.translation.z = xz.y + 2.0;
                dbg!(xz);
            }
            2 => {
                let xz = xz2
                    + (xz2 - xz1).normalize()
                        * distance_from_screen_center;
                mat.player_two_position = (xz2 - xz1)
                    .normalize()
                    * distance_from_screen_center;
                camera_transform.translation.x = xz.x;
                camera_transform.translation.z = xz.y + 2.0;
                dbg!(xz);
            }
            _ => panic!("nope"),
        }
    }
}
