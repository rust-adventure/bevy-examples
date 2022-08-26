use bevy::{
    asset::AssetServerSettings,
    core_pipeline::core_3d::Transparent3d,
    ecs::system::{
        lifetimeless::{Read, SRes},
        SystemParamItem,
    },
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey,
        MeshUniform, SetMaterialBindGroup,
        SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::{
            ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_phase::{
            AddRenderCommand, DrawFunctions,
            EntityRenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline,
            TrackedRenderPass,
        },
        render_resource::*,
        renderer::{
            RenderContext, RenderDevice, RenderQueue,
        },
        view::ExtractedView,
        RenderApp, RenderStage,
    },
    window::WindowDescriptor,
};
mod bevy_basic_camera;
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use bevy_shader_utils::ShaderUtilsPlugin;
use compute_3d::{
    compute::{
        generate_image, CloudGeneratorComputePlugin,
    },
    time::GpuTimePlugin,
    volumetric::{
        VolumetricMaterial, VolumetricMaterialPlugin,
    },
};
use std::borrow::Cow;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(WindowDescriptor {
            // uncomment for unthrottled FPS
            // present_mode:
            // bevy::window::PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(GpuTimePlugin)
        // .add_plugin(UniformComponentPlugin::<
        //     VolumetricMaterial,
        // >::default())
        .add_plugin(CloudGeneratorComputePlugin)
        .add_plugin(VolumetricMaterialPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<VolumetricMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let image = generate_image(&mut commands, images);

    // cube
    commands.spawn_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        VolumetricMaterial { fog: image.clone() },
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        Movable,
    ));

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        });
}

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
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
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
