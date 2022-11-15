//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{
            AddressMode, AsBindGroup, Extent3d,
            SamplerDescriptor, ShaderRef, TextureDimension,
            TextureFormat,
        },
        texture::ImageSampler,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            MaterialPlugin::<FloorMaterial>::default(),
        )
        .add_startup_system(repeat_floor)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FloorMaterial>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let texture = UvChecker::new();

    // cube
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes
            .add(Mesh::from(shape::Plane { size: 3.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(FloorMaterial {
            color: Color::BLUE,
            repeat: Vec4::new(3.0, 3.0, 1.0, 1.0),
            color_texture: Some(images.add(texture.image)),
            // color_texture: Some(
            //     asset_server
            //         .load("CustomUVChecker_byValle_1K.png"),
            // ),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn repeat_floor(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(texture) =
                    textures.get_mut(handle)
                {
                    if let bevy::render::texture::ImageSampler::Descriptor(sampler) = &mut texture.sampler_descriptor {
                        sampler.address_mode_u = AddressMode::Repeat;
                        sampler.address_mode_v = AddressMode::Repeat;
                        sampler.address_mode_w = AddressMode::Repeat;
                  }
                }
            }
            _ => (),
        }
    }
}

impl Material for FloorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/floor_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45cb-8225-97e2a3f056e0"]
pub struct FloorMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    repeat: Vec4,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

pub struct UvChecker {
    image: Image,
}

impl UvChecker {
    fn new() -> Self {
        let pixel = [0, 0, 0, 255];
        let mut image = Image::new_fill(
            Extent3d {
                width: 1024,
                height: 1024,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &pixel,
            TextureFormat::Rgba8Unorm,
        );
        let xy = image.size();
        for (i, current_pixel) in image
            .data
            .chunks_exact_mut(pixel.len())
            .enumerate()
        {
            // dbg!(xy);
            let r = (i as f32 % xy.x) / xy.x;
            let g = (i as f32 / xy.y).ceil() / xy.y;
            // dbg!(r, g);
            let box_r = (r * 10.0).floor() / 10.0;
            let box_g = (g * 10.0).floor() / 10.0;
            current_pixel.copy_from_slice(&[
                (box_r * 255.0) as u8,
                (box_g * 255.0) as u8,
                0,
                255,
            ]);
        }

        let mut descriptor =
            ImageSampler::linear_descriptor();
        descriptor.address_mode_u = AddressMode::Repeat;
        descriptor.address_mode_v = AddressMode::Repeat;
        descriptor.address_mode_w = AddressMode::Repeat;

        let sampler = ImageSampler::Descriptor(descriptor);

        image.sampler_descriptor = sampler;

        UvChecker { image }
    }
}
