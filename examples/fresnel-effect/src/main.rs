//! A shader and a material that uses it.

use bevy::{
    ecs::system::Command,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
mod bevy_basic_camera;

use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("071f3c").unwrap(),
        ))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(CameraControllerPlugin)
        .add_plugin(MaterialPlugin::<
            NormalVisualizerMaterial,
        >::default())
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .run();
}

fn setup(
    mut commands: Commands,
    mut custom_materials: ResMut<
        Assets<NormalVisualizerMaterial>,
    >,
) {
    let normals = vec![
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
    ];
    for (i, selection) in normals.iter().enumerate() {
        commands.add(SpawnSphere {
            transform: Transform::from_xyz(
                4.0 * i as f32 - 4.0,
                2.0,
                0.0,
            ),
            material: custom_materials.add(
                NormalVisualizerMaterial {
                    selection: *selection,
                },
            ),
        });

        let mut second_selection = selection.clone();
        second_selection.w = 1.0;
        commands.add(SpawnSphere {
            transform: Transform::from_xyz(
                4.0 * i as f32 - 4.0,
                -2.0,
                0.0,
            ),
            material: custom_materials.add(
                NormalVisualizerMaterial {
                    selection: second_selection,
                },
            ),
        });
    }

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        },
    ));

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
            shadows_enabled: false,
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

impl Material for NormalVisualizerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/normal_visualizer.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NormalVisualizerMaterial {
    #[uniform(0)]
    selection: Vec4,
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

pub struct SpawnSphere {
    pub transform: Transform,
    pub material: Handle<NormalVisualizerMaterial>,
}

impl Command for SpawnSphere {
    fn write(self, world: &mut World) {
        let (cube, sphere) = {
            let mut meshes = world
                .get_resource_mut::<Assets<Mesh>>()
                .unwrap();
            (
                meshes.add(Mesh::from(shape::Cube {
                    size: 0.1,
                })),
                meshes.add(Mesh::from(shape::UVSphere {
                    radius: 1.0,
                    ..default()
                })),
            )
        };

        let (red, green, blue) = {
            let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
            (
                materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    ..default()
                }),
            )
        };

        // Sphere
        world.spawn(MaterialMeshBundle {
            mesh: sphere,
            transform: self.transform,
            material: self.material,
            ..default()
        });
        // .with_children(|builder| {
        //     // X
        //     builder.spawn().insert_bundle(
        //         MaterialMeshBundle {
        //             mesh: cube.clone(),
        //             transform: Transform::from_xyz(
        //                 2.0, 0.0, 0.0,
        //             ),
        //             material: red,
        //             ..default()
        //         },
        //     );

        //     // Y
        //     builder.spawn().insert_bundle(
        //         MaterialMeshBundle {
        //             mesh: cube.clone(),
        //             transform: Transform::from_xyz(
        //                 0.0, 2.0, 0.0,
        //             ),
        //             material: green,
        //             ..default()
        //         },
        //     );

        //     // Z
        //     builder.spawn().insert_bundle(
        //         MaterialMeshBundle {
        //             mesh: cube,
        //             transform: Transform::from_xyz(
        //                 0.0, 0.0, 2.0,
        //             ),
        //             material: blue,
        //             ..default()
        //         },
        //     );
        // });
    }
}
