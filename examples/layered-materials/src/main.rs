use bevy::{
    color::palettes::tailwind::SLATE_950, image::ImageLoaderSettings, prelude::*,
    render::render_resource::AsBindGroup, shader::ShaderRef,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<LayeredMaterial>::default())
        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_layered: ResMut<Assets<LayeredMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let circle = Circle::new(10.0)
        .mesh()
        .build()
        .with_generated_tangents()
        .unwrap();
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(circle)),
        MeshMaterial3d(materials_layered.add(LayeredMaterial {
            base_color_texture: asset_server.load_with_settings(
                "processed_array_kram/base_color.ktx2",
                |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = true;
                    let sampler = settings.sampler.get_or_init_descriptor();
                    sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                },
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
                },
            ),
            depth_map: asset_server.load_with_settings(
                "processed_array_kram/depth_map.ktx2",
                |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false;
                    let sampler = settings.sampler.get_or_init_descriptor();
                    sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    sampler.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                },
            ),
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
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
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
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
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Debug, Default, Clone, Reflect, Asset, AsBindGroup)]
#[bindless(index_table(range(0..6)))]
struct LayeredMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    base_color_texture: Handle<Image>,
    #[texture(2, dimension = "2d_array")]
    #[sampler(3)]
    normal_map_texture: Handle<Image>,
    #[texture(4, dimension = "2d_array")]
    #[sampler(5)]
    depth_map: Handle<Image>,
}

impl Material for LayeredMaterial {
    fn fragment_shader() -> ShaderRef {
        "layered.wgsl".into()
    }
}
