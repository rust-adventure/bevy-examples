//! Loads and renders a glTF file as a scene.

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    pbr::wireframe::{
        Wireframe, WireframeConfig, WireframePlugin,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        mesh::{
            Indices, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages, WgpuFeatures,
        },
        settings::WgpuSettings,
        view::ViewDepthTexture,
    },
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;
use noise::{BasicMulti, NoiseFn};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        // .insert_resource(ClearColor(
        //     Color::hex("071f3c").unwrap(),
        // ))
        .insert_resource(ClearColor(
            Color::hex("590059").unwrap(),
        ))
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(BasicMulti::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(Material2dPlugin::<
            PostProcessingMaterial,
        >::default())
        .add_plugin(
            MaterialPlugin::<LandMaterial>::default(),
        )
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .add_system(movement)
        .add_system(change_position)
        .add_system(update_time_uniform)
        .run();
}

#[derive(Component)]
struct Ship;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LandMaterial>>,
    mut standard_materials: ResMut<
        Assets<StandardMaterial>,
    >,
    mut windows: ResMut<Windows>,
    mut post_processing_materials: ResMut<
        Assets<PostProcessingMaterial>,
    >,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary_mut().unwrap();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // commands
    //     .spawn_bundle(Camera3dBundle {
    //         transform: Transform::from_xyz(0.0, 1.5, 2.0)
    //             .looking_at(
    //                 Vec3::new(0.0, 1.5, 0.0),
    //                 Vec3::Y,
    //             ),
    //         ..default()
    //     })
    //     .insert(Movable);
    commands
        .spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    Color::WHITE,
                ),
                ..default()
            },
            camera: Camera {
                target: RenderTarget::Image(
                    image_handle.clone(),
                ),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 1.5, 2.0)
                .looking_at(
                    Vec3::new(0.0, 1.5, 0.0),
                    Vec3::Y,
                ),
            ..default()
        })
        .insert(Movable);

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer(
        (RenderLayers::TOTAL_LAYERS - 1) as u8,
    );

    let quad_handle = meshes.add(Mesh::from(
        shape::Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        )),
    ));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(
        PostProcessingMaterial {
            time: 0.,
            source_image: image_handle,
        },
    );

    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
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
        ..default()
    });

    // land
    let mut land = Mesh::from(Land {
        size: 1000.0,
        num_vertices: 1000,
    });
    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = land.attribute(Mesh::ATTRIBUTE_POSITION)
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
        land.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors,
        );
    }

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(land),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(LandMaterial {
            time: 0.,
            ship_position: Vec3::ZERO,
        }),
        // material: standard_materials.add(
        //     StandardMaterial {
        //         base_color: Color::WHITE,
        //         ..default()
        //     },
        // ),
        ..default()
    });
    // .insert(Wireframe);

    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server
                .load("craft/craft_miner.glb#Scene0"),
            transform: Transform::from_xyz(
                -2.0 as f32,
                1.0,
                0.0 as f32,
            )
            .with_scale(Vec3::splat(0.2)),
            // scene: asset_server
            //     .load("racecar/raceCarGreen.glb/#Scene0"),
            ..default()
        })
        .insert(Ship)
        .insert(Movable);

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        })
        .insert(post_processing_pass_layer);

    // The post-processing pass camera.
    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                priority: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        })
        .insert(post_processing_pass_layer);
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        With<DirectionalLight>,
    >,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32
                * std::f32::consts::TAU
                / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn update_time_uniform(
    mut materials: ResMut<Assets<PostProcessingMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time =
            time.seconds_since_startup() as f32;
    }
}

fn change_position(
    mut materials: ResMut<Assets<LandMaterial>>,
    mut ship: Query<&mut Transform, With<Ship>>,
    noise: Res<BasicMulti>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        let mut ship = ship.single_mut();
        material.1.ship_position = ship.translation;
        let new_x = noise.get([
            ship.translation.z as f64 * 0.02,
            time.seconds_since_startup() * 0.02,
        ]);
        let new_y = noise.get([
            ship.translation.z as f64 * 0.2,
            time.seconds_since_startup() * 0.2,
        ]);
        ship.translation.x = new_x as f32;
        ship.translation.y = new_y as f32 * 0.2 + 1.0;
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for LandMaterial {
    // fn fragment_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }
    fn vertex_shader() -> ShaderRef {
        "shaders/land_vertex_shader.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct LandMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    ship_position: Vec3,
}

#[derive(Debug, Copy, Clone)]
struct Land {
    size: f32,
    num_vertices: u32,
}

impl From<Land> for Mesh {
    fn from(plane: Land) -> Self {
        let extent = plane.size / 2.0;

        let jump = extent / plane.num_vertices as f32;

        let vertices = (0..=plane.num_vertices)
            .cartesian_product(0..=plane.num_vertices)
            .map(|(y, x)| {
                (
                    [
                        x as f32 * jump - 0.5 * extent,
                        0.0,
                        y as f32 * jump - 0.5 * extent,
                    ],
                    [0.0, 1.0, 0.0],
                    [
                        x as f32
                            / plane.num_vertices as f32,
                        y as f32
                            / plane.num_vertices as f32,
                    ],
                )
            })
            .collect::<Vec<_>>();

        let indices = Indices::U32(
            (0..=plane.num_vertices)
                .cartesian_product(0..=plane.num_vertices)
                .enumerate()
                .filter_map(|(index, (x, y))| {
                    if y >= plane.num_vertices {
                        None
                    } else if x >= plane.num_vertices {
                        None
                    } else {
                        Some([
                            [
                                index as u32,
                                index as u32
                                    + 1
                                    + 1
                                    + plane.num_vertices,
                                index as u32 + 1,
                            ],
                            [
                                index as u32,
                                index as u32
                                    + 1
                                    + plane.num_vertices,
                                index as u32
                                    + plane.num_vertices
                                    + 1
                                    + 1,
                            ],
                        ])
                    }
                })
                .flatten()
                .flatten()
                .collect::<Vec<_>>(),
        );
        // dbg!(&indices
        //     .iter()
        //     // .take(6)
        //     .collect::<Vec<_>>());
        // dbg!(&vertices
        //     .iter()
        //     .map(|(v, _, _)| v)
        //     .collect::<Vec<_>>());

        let positions: Vec<_> =
            vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> =
            vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> =
            vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh =
            Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions,
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

// fn debug_depth_texture(cameras: Query<&ViewDepthTexture>) {
//     for camera in cameras.iter() {
//         // dbg!(camera.depth_calculation);
//         dbg!(camera.texture.as_image_copy());
//     }
//     // ViewDepthTexture
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
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.z -= 1.0;
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

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct PostProcessingMaterial {
    #[uniform(0)]
    time: f32,
    /// In this example, this image will be the result of the main camera.
    #[texture(1)]
    #[sampler(2)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chromatic_aberration.wgsl".into()
    }
}
