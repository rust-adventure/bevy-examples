//! Loads and renders a glTF file as a scene.

use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;
use noise::{BasicMulti, NoiseFn};
use spacecraft_noiseland::post_process::{
    PostProcessPlugin, PostProcessSettings,
};

#[derive(Resource)]
struct MyNoise(BasicMulti);

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
            ..default()
        })
        // .insert_resource(ClearColor(
        //     Srgba::hex("071f3c").unwrap().into(),
        // ))
        .insert_resource(ClearColor(
            Srgba::hex("590059").unwrap().into(),
        ))
        .insert_resource(MyNoise(BasicMulti::new()))
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            ShaderUtilsPlugin,
            MaterialPlugin::<LandMaterial>::default(),
            PostProcessPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_light_direction,
                movement,
                change_position,
            ),
        )
        .run();
}

#[derive(Component)]
struct Ship;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LandMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 2.0)
            .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
        PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
        Movable,
    ));

    commands.spawn(DirectionalLight::default());

    // land
    let mut land = Mesh::from(Land {
        size: Vec2::splat(1000.0),
        subdivisions: 1000,
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

    commands.spawn((
        Mesh3d(meshes.add(land)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        MeshMaterial3d(materials.add(LandMaterial {
            ship_position: Vec3::ZERO,
        })),
    ));
    // .insert(Wireframe);

    commands.spawn((
        SceneRoot(
            asset_server
                .load("craft/craft_miner.glb#Scene0"),
        ),
        Transform::from_xyz(-2.0 as f32, 1.0, 0.0 as f32)
            .with_scale(Vec3::splat(0.2)),
        Ship,
        Movable,
    ));
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
            time.elapsed_secs() as f32
                * std::f32::consts::TAU
                / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn change_position(
    mut materials: ResMut<Assets<LandMaterial>>,
    mut ship: Single<&mut Transform, With<Ship>>,
    noise: Res<MyNoise>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.ship_position = ship.translation;
        let new_x = noise.0.get([
            ship.translation.z as f64 * 0.02,
            time.elapsed_secs_f64() * 0.02,
        ]);
        let new_y = noise.0.get([
            ship.translation.z as f64 * 0.2,
            time.elapsed_secs_f64() * 0.2,
        ]);
        ship.translation.x = new_x as f32;
        ship.translation.y = new_y as f32 * 0.2 + 1.0;
    }
}
impl Material for LandMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/land_vertex_shader.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct LandMaterial {
    #[uniform(0)]
    ship_position: Vec3,
}

#[derive(Debug, Copy, Clone)]
struct Land {
    size: Vec2,
    subdivisions: u32,
}

impl From<Land> for Mesh {
    fn from(plane: Land) -> Self {
        Mesh::from(
            Plane3d::default()
                .mesh()
                .size(plane.size.x, plane.size.y)
                .subdivisions(plane.subdivisions),
        )
    }
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
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
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
