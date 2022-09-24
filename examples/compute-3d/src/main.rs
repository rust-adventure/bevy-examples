use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::{primitives::Aabb, render_resource::*},
    window::WindowDescriptor,
};
mod bevy_basic_camera;
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use bevy_shader_utils::ShaderUtilsPlugin;
use compute_3d::{
    compute::{
        CloudGeneratorComputePlugin, CloudGeneratorImage,
        SIZE,
    },
    fog::{FogPlugin, VolumetricImage},
    time::GpuTimePlugin,
    volumetric_single::{
        VolumetricMaterial, VolumetricMaterialPlugin,
    },
    // volumetric::{
    //     VolumetricMaterial, VolumetricMaterialPlugin,
    // },
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
        .init_resource::<FogImageSetup>()
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(GpuTimePlugin)
        .add_plugin(CloudGeneratorComputePlugin)
        .add_plugin(FogPlugin)
        .add_plugin(VolumetricMaterialPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

struct FogImageSetup;
impl FromWorld for FogImageSetup {
    fn from_world(world: &mut World) -> Self {
        let mut image = Image::new_fill(
            Extent3d {
                width: SIZE.0,
                height: SIZE.1,
                depth_or_array_layers: SIZE.2,
            },
            TextureDimension::D3,
            &[0, 0, 0, 255],
            TextureFormat::Rgba8Unorm,
        );

        image.texture_descriptor.usage =
            TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING;
        let image_handle = {
            let mut images = world
                .get_resource_mut::<Assets<Image>>()
                .unwrap();
            images.add(image)
        };
        world.insert_resource(CloudGeneratorImage(
            image_handle.clone(),
        ));
        world.insert_resource(VolumetricImage(
            image_handle.clone(),
        ));

        FogImageSetup
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<VolumetricMaterial>>,
    // images: ResMut<Assets<Image>>,
    volume_image: Res<VolumetricImage>,
) {
    // let image = generate_image(&mut commands, images);

    // cube
    let mesh = Mesh::from(shape::Cube { size: 1.0 });
    let aabb = mesh.compute_aabb().unwrap();
    info!(?aabb, "we're cheating. Make sure shader bounding box matches");
    commands.spawn_bundle((
        meshes.add(mesh),
        Transform::from_xyz(0.0, 0.0, 0.0),
        VolumetricMaterial {
            fog: volume_image.0.clone(),
        },
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        aabb,
    ));

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-20.0, 2.5, 5.0)
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
