use bevy::{
    color::palettes::tailwind::*,
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor,
    },
    shader::ShaderRef,
};
use image::{
    DynamicImage, ImageBuffer, imageops::FilterType,
};

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        // .insert_resource(AmbientLight {
        //     brightness: 1_000.,
        //     ..default()
        // })
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<DpDyMaterial>::default(),
        ))
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, rotate)
        .add_systems(Update, build_mips)
        .run()
}

#[derive(Resource)]
struct ManualMips {
    x128: Handle<Image>,
    x256: Handle<Image>,
    x512: Handle<Image>,
    x1024: Handle<Image>,
    x2048: Handle<Image>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut dpdx_materials: ResMut<Assets<DpDyMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 15.)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let manual_mips = ManualMips {
        x128: asset_server.load("128.png"),
        x256: asset_server.load("256.png"),
        x512: asset_server.load("512.png"),
        x1024: asset_server.load("1024.png"),
        x2048: asset_server.load("2048.png"),
    };

    commands.spawn((
        Mesh3d(
            meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::splat(5.),
            )),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(
                manual_mips.x2048.clone(),
            ),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(-6.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(
            meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::splat(5.),
            )),
        ),
        MeshMaterial3d(dpdx_materials.add(DpDyMaterial {})),
        Transform::from_xyz(6.0, 0.0, 0.0),
    ));

    commands.insert_resource(manual_mips);
}

fn rotate(
    mut transforms: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut transforms {
        transform.rotate_x(0.01);
    }
}

fn build_mips(
    mut commands: Commands,
    manual_mips: Res<ManualMips>,
    mut assets: ResMut<Assets<Image>>,
    mut processed: Local<bool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if *processed {
        info_once!("processed");
        return;
    }
    let Some(x128) = assets.get(&manual_mips.x128) else {
        warn!("manual_mip x128 doesn't exist");
        return;
    };
    let Some(x256) = assets.get(&manual_mips.x256) else {
        warn!("manual_mip x256 doesn't exist");
        return;
    };
    let Some(x512) = assets.get(&manual_mips.x512) else {
        warn!("manual_mip x512 doesn't exist");
        return;
    };
    let Some(x1024) = assets.get(&manual_mips.x1024) else {
        warn!("manual_mip x1024 doesn't exist");
        return;
    };

    let mip_128 = build_mip(x128)
        .expect("failed to build_mip for 128");
    let mip_256 = build_mip(x256)
        .expect("failed to build_mip for 256");
    let mip_512 = build_mip(x512)
        .expect("failed to build_mip for 512");
    let mip_1024 = build_mip(x1024)
        .expect("failed to build_mip for 1024");

    let Some(x2048) = assets.get_mut(&manual_mips.x2048)
    else {
        return;
    };
    info!("ready");
    *processed = true;
    let mip_2048 = build_mip(x2048)
        .expect("failed to build_mip for 2048");

    let mut new_image = mip_2048.as_bytes().to_vec();
    new_image.append(&mut mip_1024.as_bytes().to_vec());
    new_image.append(&mut mip_512.as_bytes().to_vec());
    new_image.append(&mut mip_256.as_bytes().to_vec());
    new_image.append(&mut mip_128.as_bytes().to_vec());

    x2048.texture_descriptor.mip_level_count = 5;
    x2048.data = Some(new_image);

    // commands.spawn((
    //     Mesh3d(
    //         meshes.add(Plane3d::new(
    //             Vec3::Y,
    //             Vec2::splat(10.),
    //         )),
    //     ),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color_texture: Some(
    //             manual_mips.x2048.clone(),
    //         ),
    //         unlit: true,
    //         ..default()
    //     })),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(5., 5., 5.))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color_texture: Some(
    //             manual_mips.x2048.clone(),
    //         ),
    //         unlit: true,
    //         ..default()
    //     })),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));
}
fn build_mip(image: &Image) -> Option<DynamicImage> {
    info!(
        has_data = image.data.is_some(),
        "processing"
    );
    let Some(data) = &image.data else { return None };
    info!(
        format = ?image.texture_descriptor.format,
        "image data",
    );

    let Some(dynamic_image) = ImageBuffer::from_raw(
        image.texture_descriptor.size.width,
        image.texture_descriptor.size.height,
        data.clone(),
    )
    .map(DynamicImage::ImageRgba8) else {
        return None;
    };
    Some(dynamic_image)
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DpDyMaterial {}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for DpDyMaterial {
    fn fragment_shader() -> ShaderRef {
        "dpdy_material.wgsl".into()
    }
    fn specialize(
            _pipeline: &MaterialPipeline,
            descriptor: &mut RenderPipelineDescriptor,
            _layout: &MeshVertexBufferLayoutRef,
            _key: MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError>{
        // don't cull backfaces
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
