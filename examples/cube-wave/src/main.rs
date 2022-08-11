use bevy::{
    asset::AssetServerSettings,
    pbr::{
        MaterialPipeline, MaterialPipelineKey,
        NotShadowCaster,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::Projection,
        mesh::{
            MeshVertexBufferLayout, VertexAttributeValues,
        },
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType,
            RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError,
        },
    },
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("e3eefc").unwrap(),
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
        .add_system(update_time_for_custom_material)
        .add_system(animate_light_direction)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });
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
            shadows_enabled: true,
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

    let cube_size = 0.2;
    let mut mesh =
        Mesh::from(shape::Cube { size: cube_size });
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
    };

    let mesh_handle = meshes.add(mesh);
    let num_squares = 15;
    let half = (num_squares - 1) / 2;
    for (z, x) in
        (0..num_squares).cartesian_product(0..num_squares)
    {
        let material =
            custom_materials.add(CustomMaterial {
                time: 0.0,
                offset: ((f32::abs((x - half) as f32)
                    + f32::abs((z - half) as f32))
                    as f32)
                    .abs(),
                color: Color::rgb(0.92, 0.90, 0.73),
            });
        println!(
            "{} {}",
            cube_size * x as f32 - num_squares as f32 / 2.0,
            cube_size * z as f32 - num_squares as f32 / 2.0
        );
        commands
            .spawn_bundle(MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    cube_size * x as f32
                        - num_squares as f32 * cube_size
                            / 2.0,
                    0.0,
                    cube_size * z as f32
                        - num_squares as f32 * cube_size
                            / 2.0,
                ),
                // visibility: Visibility::visible(),
                ..default()
            })
            .insert(NotShadowCaster);
    }

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(
            OrthographicProjection {
                // left: -num_squares as f32,
                // right: num_squares as f32,
                // bottom: 0.0,
                // top: 20.0,
                // near: todo!(),
                // far: todo!(),
                // window_origin: todo!(),
                // scaling_mode: todo!(),
                scale: 0.008,
                // depth_calculation: todo!(),
                ..Default::default()
            },
        ),
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

fn update_time_for_custom_material(
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time =
            time.seconds_since_startup() as f32;
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-42ab-8225-97e2a3f056e0"]
// #[bind_group_data(StandardMaterialKey)]
#[uniform(0, CustomMaterialUniform)]
pub struct CustomMaterial {
    time: f32,
    offset: f32,
    color: Color,
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

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("cubes_{}", *label).into();
        }
        Ok(())
    }
}

/// The GPU representation of the uniform data of a [`StandardMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct CustomMaterialUniform {
    pub time: f32,
    pub offset: f32,
    pub color: Vec4,
}

impl AsBindGroupShaderType<CustomMaterialUniform>
    for CustomMaterial
{
    fn as_bind_group_shader_type(
        &self,
        images: &RenderAssets<Image>,
    ) -> CustomMaterialUniform {
        CustomMaterialUniform {
            time: self.time,
            offset: self.offset,
            color: self.color.as_linear_rgba_f32().into(),
        }
    }
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub struct StandardMaterialKey {
//     normal_map: bool,
//     cull_mode: Option<Face>,
// }

// impl From<&StandardMaterial> for StandardMaterialKey {
//     fn from(material: &StandardMaterial) -> Self {
//         StandardMaterialKey {
//             normal_map: material
//                 .normal_map_texture
//                 .is_some(),
//             cull_mode: material.cull_mode,
//         }
//     }
// }
