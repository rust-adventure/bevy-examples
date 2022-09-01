use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{
            AsBindGroup, PrimitiveTopology, ShaderRef,
        },
    },
};
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("590059").unwrap(),
        ))
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(
            MaterialPlugin::<ParticlesMaterial>::default(),
        )
        .add_startup_system(setup)
        .add_system(update_time_for_particles_material)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticlesMaterial>>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(2.5, 2.5, 2.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        });

    let mut particles =
        Mesh::from(Particles { num_particles: 100 });
    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = particles.attribute(Mesh::ATTRIBUTE_POSITION)
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
        particles.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors,
        );
    }

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(particles),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials
            .add(ParticlesMaterial { time: 0.0 }),
        ..default()
    });
}

impl Material for ParticlesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/particles.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct ParticlesMaterial {
    #[uniform(0)]
    time: f32,
}

#[derive(Debug, Copy, Clone)]
struct Particles {
    num_particles: u32,
}

impl From<Particles> for Mesh {
    fn from(particles: Particles) -> Self {
        let extent = 1.0 / 2.0;

        let jump = extent / particles.num_particles as f32;

        let vertices = (0..=particles.num_particles)
            .cartesian_product(0..=particles.num_particles)
            .cartesian_product(0..=particles.num_particles)
            .map(|((z, y), x)| {
                (
                    [
                        x as f32 * jump - 0.5 * extent,
                        y as f32 * jump - 0.5 * extent,
                        z as f32 * jump - 0.5 * extent,
                    ],
                    [0.0, 1.0, 0.0],
                )
            })
            .collect::<Vec<_>>();

        let positions: Vec<_> =
            vertices.iter().map(|(p, _)| *p).collect();
        let normals: Vec<_> =
            vertices.iter().map(|(_, n)| *n).collect();

        let mut mesh =
            Mesh::new(PrimitiveTopology::PointList);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions,
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        );
        mesh
    }
}

fn update_time_for_particles_material(
    mut materials: ResMut<Assets<ParticlesMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time =
            time.seconds_since_startup() as f32;
    }
}
