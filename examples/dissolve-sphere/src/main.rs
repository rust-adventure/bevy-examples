use bevy::{
    asset::AssetServerSettings,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
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
    asset_server.watch_for_changes().unwrap();

    // let mut flags = StandardMaterialFlags::NONE;

    // flags |= StandardMaterialFlags::DOUBLE_SIDED;
    // sphere
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_materials.add(CustomMaterial {
            // base_color: Color::BLACK,
            time: 0.,
            // flags,
            alpha_mode: AlphaMode::Blend,
            // cull_mode: None,
            // ..default()
        }),

        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // ground plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes
    //         .add(Mesh::from(shape::Plane { size: 10.0 })),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::WHITE,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });
    // // left wall
    // let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    // transform.rotate_z(std::f32::consts::FRAC_PI_2);
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(
    //         5.0, 0.15, 5.0,
    //     ))),
    //     transform,
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::INDIGO,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });
    // // back (right) wall
    // let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    // transform.rotate_x(std::f32::consts::FRAC_PI_2);
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(
    //         5.0, 0.15, 5.0,
    //     ))),
    //     transform,
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::INDIGO,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });

    // ambient light
    // commands.insert_resource(AmbientLight {
    //     color: Color::ORANGE_RED,
    //     brightness: 0.02,
    // });

    // red point light
    // commands
    //     .spawn_bundle(PointLightBundle {
    //         // transform: Transform::from_xyz(5.0, 8.0, 2.0),
    //         transform: Transform::from_xyz(1.0, 2.0, 0.0),
    //         point_light: PointLight {
    //             intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
    //             color: Color::RED,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn_bundle(PbrBundle {
    //             mesh: meshes.add(Mesh::from(
    //                 shape::UVSphere {
    //                     radius: 0.1,
    //                     ..default()
    //                 },
    //             )),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::RED,
    //                 emissive: Color::rgba_linear(
    //                     100.0, 0.0, 0.0, 0.0,
    //                 ),
    //                 ..default()
    //             }),
    //             ..default()
    //         });
    //     });

    // blue point light
    // commands
    //     .spawn_bundle(PointLightBundle {
    //         // transform: Transform::from_xyz(5.0, 8.0, 2.0),
    //         transform: Transform::from_xyz(0.0, 4.0, 0.0),
    //         point_light: PointLight {
    //             intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
    //             color: Color::BLUE,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn_bundle(PbrBundle {
    //             mesh: meshes.add(Mesh::from(
    //                 shape::UVSphere {
    //                     radius: 0.1,
    //                     ..default()
    //                 },
    //             )),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::BLUE,
    //                 emissive: Color::rgba_linear(
    //                     0.0, 0.0, 100.0, 0.0,
    //                 ),
    //                 ..default()
    //             }),
    //             ..default()
    //         });
    //     });

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
    fn fragment_shader() -> ShaderRef {
        "shaders/array_texture.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // if key.bind_group_data.normal_map {
        //     descriptor
        //         .fragment
        //         .as_mut()
        //         .unwrap()
        //         .shader_defs
        //         .push(String::from(
        //             "STANDARDMATERIAL_NORMAL_MAP",
        //         ));
        // }
        descriptor.primitive.cull_mode = None;
        // if let Some(label) = &mut descriptor.label {
        //     *label = format!("pbr_{}", *label).into();
        // }
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
// #[bind_group_data(CustomMaterialKey)]
// #[uniform(0, CustomMaterialUniform)]
pub struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    // #[uniform(1)]
    // flags: StandardMaterialFlags,
    alpha_mode: AlphaMode,
}

// Standard stuff for CustomMaterial

// NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
// bitflags::bitflags! {
//     #[repr(transparent)]
//     pub struct StandardMaterialFlags: u32 {
//         const BASE_COLOR_TEXTURE         = (1 << 0);
//         const EMISSIVE_TEXTURE           = (1 << 1);
//         const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
//         const OCCLUSION_TEXTURE          = (1 << 3);
//         const DOUBLE_SIDED               = (1 << 4);
//         const UNLIT                      = (1 << 5);
//         const ALPHA_MODE_OPAQUE          = (1 << 6);
//         const ALPHA_MODE_MASK            = (1 << 7);
//         const ALPHA_MODE_BLEND           = (1 << 8);
//         const TWO_COMPONENT_NORMAL_MAP   = (1 << 9);
//         const FLIP_NORMAL_MAP_Y          = (1 << 10);
//         const NONE                       = 0;
//         const UNINITIALIZED              = 0xFFFF;
//     }
// }

// /// The GPU representation of the uniform data of a [`StandardMaterial`].
// #[derive(Clone, Default, ShaderType)]
// pub struct CustomMaterialUniform {
//     // /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything
//     // /// in between.
//     // pub base_color: Vec4,
//     // // Use a color for user friendliness even though we technically don't use the alpha channel
//     // // Might be used in the future for exposure correction in HDR
//     // pub emissive: Vec4,
//     // /// Linear perceptual roughness, clamped to [0.089, 1.0] in the shader
//     // /// Defaults to minimum of 0.089
//     // pub roughness: f32,
//     // /// From [0.0, 1.0], dielectric to pure metallic
//     // pub metallic: f32,
//     // /// Specular intensity for non-metals on a linear scale of [0.0, 1.0]
//     // /// defaults to 0.5 which is mapped to 4% reflectance in the shader
//     // pub reflectance: f32,
//     pub flags: u32,
//     pub time: f32, // /// When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque,
//                    // /// and any below means fully transparent.
//                    // pub alpha_cutoff: f32,
// }

// impl AsBindGroupShaderType<CustomMaterialUniform>
//     for CustomMaterial
// {
//     fn as_bind_group_shader_type(
//         &self,
//         images: &RenderAssets<Image>,
//     ) -> CustomMaterialUniform {
//         let mut flags = StandardMaterialFlags::NONE;
//         // if self.base_color_texture.is_some() {
//         //     flags |=
//         //         StandardMaterialFlags::BASE_COLOR_TEXTURE;
//         // }
//         // if self.emissive_texture.is_some() {
//         //     flags |=
//         //         StandardMaterialFlags::EMISSIVE_TEXTURE;
//         // }
//         // if self.metallic_roughness_texture.is_some() {
//         //     flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
//         // }
//         // if self.occlusion_texture.is_some() {
//         //     flags |=
//         //         StandardMaterialFlags::OCCLUSION_TEXTURE;
//         // }
//         // if self.double_sided {
//         // flags |= StandardMaterialFlags::DOUBLE_SIDED;
//         // }
//         // if self.unlit {
//         //     flags |= StandardMaterialFlags::UNLIT;
//         // }
//         // let has_normal_map =
//         //     self.normal_map_texture.is_some();
//         // if has_normal_map {
//         //     match images
//         //         .get(
//         //             self.normal_map_texture
//         //                 .as_ref()
//         //                 .unwrap(),
//         //         )
//         //         .unwrap()
//         //         .texture_format
//         //     {
//         //         // All 2-component unorm formats
//         //         TextureFormat::Rg8Unorm
//         //         | TextureFormat::Rg16Unorm
//         //         | TextureFormat::Bc5RgUnorm
//         //         | TextureFormat::EacRg11Unorm => {
//         //             flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
//         //         }
//         //         _ => {}
//         //     }
//         //     if self.flip_normal_map_y {
//         //         flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;
//         //     }
//         // }
//         // NOTE: 0.5 is from the glTF default - do we want this?
//         let mut alpha_cutoff = 0.5;
//         match self.alpha_mode {
//             AlphaMode::Opaque => {
//                 flags |=
//                     StandardMaterialFlags::ALPHA_MODE_OPAQUE
//             }
//             AlphaMode::Mask(c) => {
//                 alpha_cutoff = c;
//                 flags |=
//                     StandardMaterialFlags::ALPHA_MODE_MASK;
//             }
//             AlphaMode::Blend => {
//                 flags |=
//                     StandardMaterialFlags::ALPHA_MODE_BLEND
//             }
//         };

//         // StandardMaterialUniform {
//         //     base_color: self
//         //         .base_color
//         //         .as_linear_rgba_f32()
//         //         .into(),
//         //     emissive: self.emissive.into(),
//         //     roughness: self.perceptual_roughness,
//         //     metallic: self.metallic,
//         //     reflectance: self.reflectance,
//         //     flags: flags.bits(),
//         //     alpha_cutoff,
//         // }

//         CustomMaterialUniform {
//             time: self.time,
//             flags: flags.bits(),
//         }
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub struct CustomMaterialKey {
//     // normal_map: bool,
//     cull_mode: Option<Face>,
// }

// impl From<&CustomMaterial> for CustomMaterialKey {
//     fn from(material: &CustomMaterial) -> Self {
//         CustomMaterialKey {
//             // normal_map: material
//             //     .normal_map_texture
//             //     .is_some(),
//             cull_mode: None,
//         }
//     }
// }
