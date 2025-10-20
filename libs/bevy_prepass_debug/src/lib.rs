// The code in this file is literally the Bevy
// prepass example copy/pasted and slightly
// modified to exist as a plugin.
// https://github.com/bevyengine/bevy/blob/b208388af95ecd753e4710f40baf2e913bc85c17/examples/shader/shader_prepass.rs
use bevy::{
    asset::{load_internal_asset, uuid_handle},
    light::NotShadowCaster,
    math::vec2,
    prelude::*,
    reflect::TypePath,
    render::render_resource::*,
    shader::ShaderRef,
};
// use bevy_inspector_egui::{
//     prelude::*, quick::ResourceInspectorPlugin,
// };

const SHOW_PREPASS_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("3ce3ca25-a0bd-4e4f-a239-b96564809547");
/// Debug depth/normal/
/// In order to function, the [`PrepassDebug`]
/// component should be attached to the camera
/// entity.
#[derive(Default)]
pub struct PrepassDebugPlugin;

impl Plugin for PrepassDebugPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHOW_PREPASS_SHADER_HANDLE,
            "../assets/shaders/show_prepass.wgsl",
            Shader::from_wgsl
        );

        app.init_resource::<PrepassSettings>()
            .register_type::<PrepassSettings>()
            // .add_plugins((ResourceInspectorPlugin::<
            //     PrepassSettings,
            // >::default(),))
            .add_plugins(MaterialPlugin::<
                PrepassOutputMaterial,
            > {
                // This material only needs to read the
                // prepass textures, but the
                // meshes using it should not contribute to
                // the prepass render, so we can disable it.
                prepass_enabled: false,
                ..default()
            })
            .add_systems(Startup, setup_prepass_debug)
            .add_systems(Update, toggle_prepass_view);
    }
}

fn setup_prepass_debug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut depth_materials: ResMut<
        Assets<PrepassOutputMaterial>,
    >,
) {
    // A quad that shows the outputs of the prepass
    // To make it easy, we just draw a big quad right
    // in front of the camera. For a real application,
    // this isn't ideal.
    commands.spawn((
        Mesh3d(
            meshes
                .add(Rectangle::from_size(vec2(20., 20.))),
        ),
        MeshMaterial3d(depth_materials.add(
            PrepassOutputMaterial {
                settings: ShowPrepassSettings {
                    show_depth: 1,
                    ..default()
                },
            },
        )),
        Transform::from_xyz(-0.75, 1.25, 3.0).looking_at(
            Vec3::new(2.0, -2.5, -5.0),
            Vec3::Y,
        ),
        NotShadowCaster,
    ));
}

#[derive(Reflect, Default, Debug)]
enum Show {
    #[default]
    None,
    Depth,
    Normals,
    MotionVectors,
}
// #[derive(Reflect, Resource, Default,
// InspectorOptions)] #[reflect(Resource,
// InspectorOptions)]
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct PrepassSettings {
    show: Show,
    padding_1: u32,
    padding_2: u32,
}

#[derive(Debug, Clone, Default, ShaderType)]
struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    show_motion_vectors: u32,
    padding_1: u32,
    padding_2: u32,
}

// This shader simply loads the prepass texture
// and outputs it directly
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PrepassOutputMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for PrepassOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        SHOW_PREPASS_SHADER_HANDLE.into()
    }

    // This needs to be transparent in order to show
    // the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn toggle_prepass_view(
    keycode: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<PrepassSettings>,
    material_handle: Single<
        &MeshMaterial3d<PrepassOutputMaterial>,
    >,
    mut materials: ResMut<Assets<PrepassOutputMaterial>>,
) {
    if keycode.just_pressed(KeyCode::Space) {
        let next_view = match settings.show {
            Show::None => Show::Depth,
            Show::Depth => Show::Normals,
            Show::Normals => Show::MotionVectors,
            Show::MotionVectors => Show::None,
        };
        settings.show = next_view;
    }
    if settings.is_changed() {
        let prepass_view = match settings.show {
            Show::None => 0,
            Show::Depth => 1,
            Show::Normals => 2,
            Show::MotionVectors => 3,
        };

        let mat =
            materials.get_mut(&material_handle.0).unwrap();
        mat.settings.show_depth =
            (prepass_view == 1) as u32;
        mat.settings.show_normals =
            (prepass_view == 2) as u32;
        mat.settings.show_motion_vectors =
            (prepass_view == 3) as u32;
    }
}
