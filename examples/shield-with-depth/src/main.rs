//! A shader and a material that uses it.

use std::f32::consts::FRAC_PI_2;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        prepass::{DepthPrepass, NormalPrepass},
    },
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        NotShadowCaster,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{
            MeshVertexBufferLayout, VertexAttributeValues,
        },
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor,
            ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
};

use bevy_asset_loader::prelude::*;
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("071f3c").unwrap(),
        ))
        .insert_resource(AmbientLight {
            color: Color::ORANGE_RED,
            brightness: 0.02,
        })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(CameraControllerPlugin)
        .add_state::<MyStates>()
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial> {
            prepass_enabled: false,
            ..default()
        }) // Debug
        .add_plugin(
            MaterialPlugin::<PrepassOutputMaterial> {
                // This material only needs to read the prepass textures,
                // but the meshes using it should not contribute to the prepass render, so we can disable it.
                prepass_enabled: false,
                ..default()
            },
        )
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(
            MyStates::AssetLoading,
        )
        .add_system(
            setup.in_schedule(OnEnter(MyStates::Next)),
        )
        .add_system(
            toggle_prepass_view
                .in_set(OnUpdate(MyStates::Next)),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    assets: Res<MyAssets>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut depth_materials: ResMut<
        Assets<PrepassOutputMaterial>,
    >,
) {
    // Debug
    // A quad that shows the outputs of the prepass
    // To make it easy, we just draw a big quad right in front of the camera. For a real application, this isn't ideal.
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(
                shape::Quad::new(Vec2::new(20.0, 20.0))
                    .into(),
            ),
            material: depth_materials.add(
                PrepassOutputMaterial {
                    settings: ShowPrepassSettings::default(
                    ),
                },
            ),
            transform: Transform::from_xyz(
                -0.75, 1.25, 3.0,
            )
            .looking_at(
                Vec3::new(2.0, -2.5, -5.0),
                Vec3::Y,
            ),
            ..default()
        },
        NotShadowCaster,
    ));
    // end Debug
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-2.0, 3., 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // To enable the prepass you need to add the components associated with the ones you need
        // This will write the depth buffer to a texture that you can use in the main pass
        DepthPrepass,
        // This will generate a texture containing world normals (with normal maps applied)
        NormalPrepass,
        CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        },
        BloomSettings::default(),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes
            .add(shape::Plane::from_size(10.0).into()),
        material: std_materials
            .add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.3, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes
            .add(shape::Plane::from_size(5.0).into()),
        material: std_materials
            .add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.3, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(
                FRAC_PI_2,
            )),

        ..default()
    });

    let transform = Transform::from_xyz(0.0, 0.5, 0.0);
    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: assets.hex_sphere.clone(),
            transform: transform,
            material: materials.add(CustomMaterial {
                color: Color::BLUE,
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn(SceneBundle {
        scene: assets.ferris.clone(),
        transform: transform.clone().with_rotation(
            Quat::from_rotation_y(-FRAC_PI_2),
        ),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("shield_{}", *label).into();
        }
        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(
        path = "models/hex-sphere-5-subdivisions.glb#Mesh0/Primitive0"
    )]
    hex_sphere: Handle<Mesh>,
    #[asset(path = "models/ferris3d_v1.0.glb#Scene0")]
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

// Debug

#[derive(Debug, Clone, Default, ShaderType)]
struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    padding_1: u32,
    padding_2: u32,
}

// This shader simply loads the prepass texture and outputs it directly
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "0af99895-b96e-4451-bc12-c6b1c1c52750"]
pub struct PrepassOutputMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for PrepassOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/show_prepass.wgsl".into()
    }

    // This needs to be transparent in order to show the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Every time you press space, it will cycle between transparent, depth and normals view
fn toggle_prepass_view(
    keycode: Res<Input<KeyCode>>,
    material_handle: Query<&Handle<PrepassOutputMaterial>>,
    mut materials: ResMut<Assets<PrepassOutputMaterial>>,
) {
    if keycode.just_pressed(KeyCode::Space) {
        let handle = material_handle.single();
        let mat = materials.get_mut(handle).unwrap();
        if mat.settings.show_depth == 1 {
            dbg!("normal");
            mat.settings.show_depth = 0;
            mat.settings.show_normals = 1;
        } else if mat.settings.show_normals == 1 {
            dbg!("transparent");
            mat.settings.show_depth = 0;
            mat.settings.show_normals = 0;
        } else {
            dbg!("depth");
            mat.settings.show_depth = 1;
            mat.settings.show_normals = 0;
        }

        // let mut text = text.single_mut();
        // text.sections[0].value =
        //     format!("Prepass Output: {out_text}\n");
    }
}
