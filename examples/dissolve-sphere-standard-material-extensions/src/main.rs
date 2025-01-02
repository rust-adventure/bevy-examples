use bevy::{
    color::palettes::tailwind::{BLUE_400, RED_400},
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
        .insert_resource(ClearColor(
            Srgba::hex("1fa9f4").unwrap().into(),
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
    let mut mesh = Sphere::default().mesh().uv(32, 18);
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

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        MeshMaterial3d(dissolve_materials
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
        })),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Movable,
        DepthPrepass,
        NormalPrepass,
        MotionVectorPrepass,
        Fxaa::default(),
    ));
    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(
            Plane3d::default().mesh().size(10., 10.),
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    // red point light
    commands
        .spawn((
            Transform::from_xyz(1.0, 2.0, 0.0),
            PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: RED_400.into(),
                shadows_enabled: true,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(
                    meshes
                        .add(Sphere { radius: 0.1 }.mesh()),
                ),
                MeshMaterial3d(materials.add(
                    StandardMaterial {
                        base_color: RED_400.into(),
                        emissive: LinearRgba::new(
                            100., 0., 0., 0.,
                        ),
                        ..default()
                    },
                )),
            ));
        });

    // blue point light
    commands
        .spawn((
            Transform::from_xyz(0.0, 4.0, 0.0),
            PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: BLUE_400.into(),
                shadows_enabled: true,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(
                    meshes
                        .add(Sphere { radius: 0.1 }.mesh()),
                ),
                MeshMaterial3d(materials.add(
                    StandardMaterial {
                        base_color: BLUE_400.into(),
                        emissive: LinearRgba::new(
                            0.0, 0.0, 100.0, 0.0,
                        ),
                        ..default()
                    },
                )),
            ));
        });

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(10.0, 20.0, 10.0),
            rotation: Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_4,
            ),
            ..default()
        },
    ));
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        With<DirectionalLight>,
    >,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.5);
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
    }
}
