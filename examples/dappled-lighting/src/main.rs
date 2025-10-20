use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::tailwind::*,
    image::ImageLoaderSettings,
    light::DirectionalLightTexture,
    pbr::decal,
    prelude::*,
    render::{
        render_resource::{AsBindGroup, TextureFormat},
        renderer::{RenderAdapter, RenderDevice},
    },
    shader::ShaderRef,
    sprite_render::Material2dPlugin,
};
use light_consts::lux::AMBIENT_DAYLIGHT;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            Material2dPlugin::<LightTextureMaterial>::default(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
    render_adapter: Res<RenderAdapter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_light_texture: ResMut<
        Assets<LightTextureMaterial>,
    >,
    mut images: ResMut<Assets<Image>>,
) {
    // Error out if clustered decals (and so light
    // textures) aren't supported on the current
    // platform.
    if !decal::clustered::clustered_decals_are_usable(
        &render_device,
        &render_adapter,
    ) {
        error!(
            "Light textures aren't usable on this platform."
        );
        commands.write_message(AppExit::error());
    }

    // This is the texture that will be rendered to.
    let image = Image::new_target_texture(
        512,
        512,
        TextureFormat::bevy_default(),
    );
    // the image we'll use
    let image_handle = images.add(image);
    let light_texture_layer = RenderLayers::layer(1);

    // spawn a floor
    commands.spawn((
        Mesh3d(
            meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::splat(100.),
            )),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN_400.into(),
            ..default()
        })),
        Transform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2., 20., 15.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: ORANGE_900.into(),
            ..default()
        })),
        Transform::default(),
    ));

    commands.spawn(Camera3d::default()).insert(
        Transform::from_xyz(15.0, 15., 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    );

    commands.spawn((
        Transform::from_xyz(-10., 5.0, 10.)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::splat(2.)),
        DirectionalLight {
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        DirectionalLightTexture {
            image: image_handle.clone(),
            // image: asset_server
            //     .load("caustic_directional_texture.png"),
            tiled: true,
        },
        Visibility::Visible,
    ));

    commands.spawn((
        Mesh2d(
            meshes.add(Rectangle::from_size(Vec2::splat(
                512.,
            ))),
        ),
        MeshMaterial2d(materials_light_texture.add(
            LightTextureMaterial {
                base_texture: Some(asset_server.load_with_settings(
                    "pattern_42.png",
                    |settings: &mut ImageLoaderSettings| {
                        let  sampler = settings.sampler.get_or_init_descriptor();
                        sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    }
                )),
                layer_1_texture: Some(asset_server.load_with_settings(
                    "pattern_77.png",
                    |settings: &mut ImageLoaderSettings| {
                        let  sampler = settings.sampler.get_or_init_descriptor();
                        sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    }
                )),
                layer_2_texture: Some(asset_server.load_with_settings(
                    "pattern_82.png",
                    |settings: &mut ImageLoaderSettings| {
                        let  sampler = settings.sampler.get_or_init_descriptor();
                        sampler.address_mode_u = bevy::image::ImageAddressMode::Repeat;
                        sampler.address_mode_v = bevy::image::ImageAddressMode::Repeat;
                    }
                )),
            },
        )),
        Transform::from_translation(Vec3::new(
            0.0, 0.0, 1.0,
        )),
        light_texture_layer.clone(),
    ));

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: image_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            0.0, 0.0, 0.0,
        ))
        .looking_at(Vec3::ZERO, Vec3::Y),
        light_texture_layer,
    ));
}

// The material that will be used to generate the
// light texture image
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LightTextureMaterial {
    #[texture(1)]
    #[sampler(2)]
    base_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    layer_1_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    layer_2_texture: Option<Handle<Image>>,
}

impl bevy::sprite_render::Material2d
    for LightTextureMaterial
{
    fn fragment_shader() -> ShaderRef {
        "light_texture.wgsl".into()
    }
}
