use bevy::{
    asset::AssetServerSettings,
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        StandardMaterialKey, StandardMaterialUniform,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{
            MeshVertexBufferLayout, VertexAttributeValues,
        },
        primitives::Frustum,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, Face,
            RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError,
            TextureFormat,
        },
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("071f3c").unwrap(),
        ))
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(
            MaterialPlugin::<CustomMaterial>::default(),
        )
        .add_startup_system(setup)
        .add_system(change_color)
        .add_system(animate_light_direction)
        .run();
}

#[derive(Component)]
struct Cube;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut mesh = Mesh::from(shape::UVSphere {
        radius: 1.0,
        ..default()
    });
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

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(CustomMaterial {
            time: 0.,
            ..default()
        }),

        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
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

fn change_color(
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time =
            time.seconds_since_startup() as f32;
    }
}

// The Material trait is very configurable, but comes with sensible defaults for all methods.
// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/vertex_shader.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/standard_extension.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.sm.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if key.bind_group_data.normal_map {
            descriptor
                .fragment
                .as_mut()
                .unwrap()
                .shader_defs
                .push(String::from(
                    "STANDARDMATERIAL_NORMAL_MAP",
                ));
        }
        descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label =
                format!("custom_pbr_{}", *label).into();
        }
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
#[bind_group_data(CustomMaterialKey)]
#[uniform(0, CustomMaterialUniform)]
pub struct CustomMaterial {
    #[uniform(12)]
    time: f32,
    sm: StandardMaterial,
}

// Standard stuff for CustomMaterial

/// The GPU representation of the uniform data of a [`StandardMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct CustomMaterialUniform {
    pub time: f32,
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything
    /// in between.
    pub base_color: Vec4,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Vec4,
    /// Linear perceptual roughness, clamped to [0.089, 1.0] in the shader
    /// Defaults to minimum of 0.089
    pub roughness: f32,
    /// From [0.0, 1.0], dielectric to pure metallic
    pub metallic: f32,
    /// Specular intensity for non-metals on a linear scale of [0.0, 1.0]
    /// defaults to 0.5 which is mapped to 4% reflectance in the shader
    pub reflectance: f32,
    pub flags: u32,
    /// When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque,
    /// and any below means fully transparent.
    pub alpha_cutoff: f32,
}

impl AsBindGroupShaderType<CustomMaterialUniform>
    for CustomMaterial
{
    fn as_bind_group_shader_type(
        &self,
        images: &RenderAssets<Image>,
    ) -> CustomMaterialUniform {
        let sm_uniform: StandardMaterialUniform =
            self.sm.as_bind_group_shader_type(images);
        CustomMaterialUniform {
            time: self.time,
            base_color: sm_uniform.base_color,
            emissive: sm_uniform.emissive.into(),
            roughness: sm_uniform.roughness,
            metallic: sm_uniform.metallic,
            reflectance: sm_uniform.reflectance,
            flags: sm_uniform.flags,
            alpha_cutoff: sm_uniform.alpha_cutoff,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CustomMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
}

impl From<&CustomMaterial> for CustomMaterialKey {
    fn from(material: &CustomMaterial) -> Self {
        CustomMaterialKey {
            normal_map: material
                .sm
                .normal_map_texture
                .is_some(),
            cull_mode: material.sm.cull_mode,
        }
    }
}
