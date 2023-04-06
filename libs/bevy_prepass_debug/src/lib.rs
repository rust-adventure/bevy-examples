use bevy::{
    asset::load_internal_asset, pbr::NotShadowCaster,
    prelude::*, reflect::TypeUuid,
    render::render_resource::*,
};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

const SHOW_PREPASS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(
        Shader::TYPE_UUID,
        3253086272234592509,
    );
/// Debug depth/normal/
/// In order to function, the [`PrepassDebug`] component should be attached to the camera entity.
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

        app.world
            .resource_mut::<Assets<ColorMaterial>>()
            .set_untracked(
                Handle::<ColorMaterial>::default(),
                ColorMaterial {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );

        app.init_resource::<PrepassSettings>()
            .register_type::<PrepassSettings>()
            .add_plugin(ResourceInspectorPlugin::<
                PrepassSettings,
            >::default())
            .add_plugin(MaterialPlugin::<
                PrepassOutputMaterial,
            > {
                // This material only needs to read the prepass textures,
                // but the meshes using it should not contribute to the prepass render, so we can disable it.
                prepass_enabled: false,
                ..default()
            })
            .add_startup_system(setup_prepass_debug)
            .add_system(toggle_prepass_view);
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
}

#[derive(Reflect, Default)]
enum Show {
    #[default]
    None,
    Depth,
    Normals,
    MotionVectors,
}
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
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

// This shader simply loads the prepass texture and outputs it directly
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "0af99895-b96e-4451-ba12-c6b1c1c52750"]
pub struct PrepassOutputMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for PrepassOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        SHOW_PREPASS_SHADER_HANDLE.typed().into()
    }

    // This needs to be transparent in order to show the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Every time you press space, it will cycle between transparent, depth and normals view
fn toggle_prepass_view(
    settings: Res<PrepassSettings>,
    material_handle: Query<&Handle<PrepassOutputMaterial>>,
    mut materials: ResMut<Assets<PrepassOutputMaterial>>,
) {
    if settings.is_changed() {
        let prepass_view = match settings.show {
            Show::None => 0,
            Show::Depth => 1,
            Show::Normals => 2,
            Show::MotionVectors => 3,
        };

        let handle = material_handle.single();
        let mat = materials.get_mut(handle).unwrap();
        mat.settings.show_depth =
            (prepass_view == 1) as u32;
        mat.settings.show_normals =
            (prepass_view == 2) as u32;
        mat.settings.show_motion_vectors =
            (prepass_view == 3) as u32;
    }
}
