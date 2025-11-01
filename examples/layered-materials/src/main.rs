use bevy::{
    asset::{load_internal_asset, uuid_handle},
    color::palettes::tailwind::SLATE_950,
    image::ImageLoaderSettings,
    math::Affine2,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

const PBR_FRAGMENT_REPLACEMENT: Handle<Shader> =
    uuid_handle!("980ee5ac-3d4a-4593-841f-6b46f02abcb3");

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                LayeredExtension,
            >,
        >::default())
        .add_systems(Startup, setup);

    load_internal_asset!(
        app,
        PBR_FRAGMENT_REPLACEMENT,
        "../assets/pbr_fragment.wgsl",
        Shader::from_wgsl
    );

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_layered: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                LayeredExtension,
            >,
        >,
    >,
    mut asset_server: Res<AssetServer>,
) {
    let circle = Circle::new(10.0)
        .mesh()
        .build()
        .with_generated_tangents()
        .unwrap();
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(circle)),
        MeshMaterial3d(materials_layered.add(ExtendedMaterial{
            base: StandardMaterial{
                // parallax_depth_scale is notably texture-size dependent
                parallax_depth_scale: 0.00,
                // uv_transform: Affine2::from_scale(Vec2::splat(30.)),
                ..default()
            },
            extension: LayeredExtension {
            base_color_texture: asset_server.load_with_settings(
                "processed_array_kram/base_color.ktx2",
                |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = true;
                    let sampler = settings.sampler.get_or_init_descriptor();
                    sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                }
            ),
            // metallic_roughness_texture: asset_server.load_with_settings(
            //     "processed_array/array_metallic_roughness.ktx2",
            //     |settings: &mut ImageLoaderSettings| {
            //         let sampler = settings.sampler.get_or_init_descriptor();
            //         sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
            //         sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
            //         sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
            //     }
            // ),
            normal_map_texture: asset_server.load_with_settings(
                "processed_array_kram/normal_map.ktx2",
                |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false;
                    let sampler = settings.sampler.get_or_init_descriptor();
                    sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                }
            ),
            depth_map: asset_server.load_with_settings(
                "processed_array_kram/depth_map.ktx2",
                |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false;
                    let sampler = settings.sampler.get_or_init_descriptor();
                    sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                }
            ),
            ..default()
        }})),
        Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2,
        )),
    ));

    // loads in one of the single-texture ktx files
    // commands.spawn((
    //     Mesh3d(meshes.add(circle)),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color_texture: Some(
    //             asset_server.load(
    //                 "processed_single/base_color.ktx2",
    //             ),
    //         ),
    //         // metallic_roughness_texture: Some(asset_server.load(
    //         //     "processed_single/metallic_roughness.ktx2"
    //         // )),
    //         normal_map_texture: Some(
    //             asset_server.load(
    //                 "processed_single/normal_map.ktx2",
    //             ),
    //         ),
    //         depth_map: Some(
    //             asset_server.load(
    //                 "processed_single/depth_map.ktx2",
    //             ),
    //         ),
    //         // parallax_depth_scale is notably texture-size dependent
    //         parallax_depth_scale: 0.02,
    //         ..default()
    //     })),
    //     Transform::from_rotation(Quat::from_rotation_x(
    //         -std::f32::consts::FRAC_PI_2,
    //     )),
    // ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(
            materials.add(Color::srgb_u8(124, 144, 255)),
        ),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// A StandardMaterial extension to pass in texture array images
// We *should* basically copy the pieces of the StandardMaterial pipeline
// instead, but piecemealing the StandardMaterial to only use what we need
// requires significantly more code, and this is just an example
#[derive(
    Asset, AsBindGroup, Reflect, Debug, Clone, Default,
)]
struct LayeredExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[texture(100, dimension = "2d_array")]
    #[sampler(101)]
    base_color_texture: Handle<Image>,
    // #[texture(102, dimension = "2d_array")]
    // #[sampler(103)]
    // metallic_roughness_texture: Handle<Image>,
    #[texture(104, dimension = "2d_array")]
    #[sampler(105)]
    normal_map_texture: Handle<Image>,
    #[texture(106, dimension = "2d_array")]
    #[sampler(107)]
    depth_map: Handle<Image>,
}

impl MaterialExtension for LayeredExtension {
    fn fragment_shader() -> ShaderRef {
        "layered.wgsl".into()
    }

    fn specialize(
        pipeline: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> std::result::Result<(), bevy::render::render_resource::SpecializedMeshPipelineError>{
        if let Some(fragment) = descriptor.fragment.as_mut()
        {
            let shader_defs = &mut fragment.shader_defs;

            for shader_def in
                ["STANDARD_MATERIAL_NORMAL_MAP"]
            {
                shader_defs.push(shader_def.into());
            }
        }
        Ok(())
    }
}
