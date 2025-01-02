use bevy::{
    color::palettes::css::{BLUE, ORANGE_RED},
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        NotShadowCaster,
    },
    prelude::*,
    render::{
        mesh::{
            MeshVertexBufferLayoutRef,
            VertexAttributeValues,
        },
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use bevy_asset_loader::prelude::*;
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("071f3c").unwrap().into(),
        ))
        .add_plugins((
            DefaultPlugins,
            ShaderUtilsPlugin,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(
            MyStates::AssetLoading,
        )
        .add_systems(OnEnter(MyStates::Next), setup)
        .add_systems(Update, animate_light_direction)
        .run();
}

#[derive(Component)]
struct Inserted;

/// set up a simple 3D scene
fn setup(
    assets: Res<MyAssets>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 0.02,
    });
    const HALF_SIZE: f32 = 10.0;
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

    // ground plane

    let mut plane_mesh = Mesh::from(
        Plane3d::default().mesh().size(100., 100.),
    );
    plane_mesh.generate_tangents().unwrap();
    commands.spawn((
        Mesh3d(meshes.add(plane_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
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
            ..default()
        })),
        Transform::from_xyz(0.0, -0.3, 0.0),
    ));

    let mesh = meshes.get_mut(&assets.hex_sphere).unwrap();
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

    let custom_material =
        custom_materials.add(CustomMaterial {
            color: BLUE.into(),
            alpha_mode: AlphaMode::Blend,
        });
    let num_ferris = 20;
    for (z, x) in
        (0..num_ferris).cartesian_product(0..num_ferris)
    {
        let subject_transform = Transform::from_xyz(
            -x as f32 * 2.5 + 5.0,
            0.0,
            -z as f32 * 2.5 + 2.5,
        );
        commands.queue(SpawnShieldedFerris {
            transform: subject_transform,
            shield: assets.hex_sphere.clone(),
            ferris: assets.ferris.clone(),
            shield_material: MeshMaterial3d(
                custom_material.clone(),
            ),
        });
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.5, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/vertex_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("shield_{}", *label).into();
        }
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(1)]
    color: LinearRgba,
    alpha_mode: AlphaMode,
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

pub struct SpawnShieldedFerris {
    pub transform: Transform,
    pub shield: Handle<Mesh>,
    pub ferris: Handle<Scene>,
    pub shield_material: MeshMaterial3d<CustomMaterial>,
}

impl Command for SpawnShieldedFerris {
    fn apply(self, world: &mut World) {
        world.spawn((
            Mesh3d(self.shield),
            self.shield_material,
            self.transform.clone(),
            Visibility::Visible,
            NotShadowCaster,
        ));

        world.spawn((
            SceneRoot(self.ferris),
            self.transform.clone(),
        ));
    }
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(
        path = "hex-sphere-5-subdivisions.glb#Mesh0/Primitive0"
    )]
    hex_sphere: Handle<Mesh>,
    #[asset(path = "ferris3d_v1.0.glb#Scene0")]
    ferris: Handle<Scene>,
}

#[derive(
    Default, Clone, Eq, PartialEq, Debug, Hash, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
