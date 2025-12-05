use bevy::{
    color::palettes::tailwind::*,
    image::{ImageFilterMode, ImageLoaderSettings},
    math::Affine2,
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, rotate)
        .run()
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 15.)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::Y,
            Vec2::new(30., 90.),
        ))),
        MeshMaterial3d(materials.add(
            StandardMaterial {
                base_color_texture:
                    Some(
                        asset_server.load_with_settings(
                            // Bevy has an issue where these settings can get "shared"
                            // so to avoid that we load a clean second exact copy from
                            // a different file
                            "floor_graph_base_color_uncompressed_2.ktx2",
                            |settings: &mut ImageLoaderSettings| {
                                let  descriptor = settings.sampler.get_or_init_descriptor();
                                descriptor.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                                descriptor.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                                descriptor.address_mode_w = bevy::image::ImageAddressMode::Repeat;

                            }
                        ),
                    ),
                unlit: true,
                cull_mode: None,
                uv_transform: Affine2::from_scale(Vec2::new(30., 90.)),
                ..default()
            },
        )),
        Transform::from_xyz(-30.1, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(0.1)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::Y,
            Vec2::new(30., 90.),
        ))),
        MeshMaterial3d(materials.add(
            StandardMaterial {
                base_color_texture:
                    Some(
                        asset_server.load_with_settings(
                            "floor_graph_base_color_uncompressed.ktx2",
                            |settings: &mut ImageLoaderSettings| {
                                let descriptor = settings.sampler.get_or_init_descriptor();
                                descriptor.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                                descriptor.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                                descriptor.address_mode_w = bevy::image::ImageAddressMode::Repeat;
                                let sampler = settings.sampler.get_or_init_descriptor();
                                sampler.anisotropy_clamp = 16;
                                sampler.min_filter = ImageFilterMode::Linear;
                                sampler.mag_filter = ImageFilterMode::Linear;
                                sampler.mipmap_filter = ImageFilterMode::Linear;
                            }
                        ),
                    ),
                unlit: true,
                cull_mode: None,
                uv_transform: Affine2::from_scale(Vec2::new(30., 90.)),
                ..default()
            },
        )),
        Transform::from_xyz(30.1, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(0.1)),
    ));
}

fn rotate(
    mut transforms: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut transforms {
        // transform.rotate_x(0.01);
    }
}
