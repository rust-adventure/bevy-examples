use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        mesh::VertexAttributeValues, render_resource::*,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                DecalsExtension,
            >,
        >::default())
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                Uv2Extension,
            >,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_uv2: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                Uv2Extension,
            >,
        >,
    >,
    mut materials_decals: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                DecalsExtension,
            >,
        >,
    >,
    asset_server: Res<AssetServer>,
) {
    let cuboid = Cuboid::new(1., 1., 1.).mesh().build();

    let Some(VertexAttributeValues::Float32x2(uvs)) =
        cuboid.attribute(Mesh::ATTRIBUTE_UV_0)
    else {
        return;
    };

    let uvs_b: Vec<[f32; 2]> =
        uvs.iter().map(|[x, y]| [x / 2., y / 2.]).collect();

    let cuboid = cuboid.with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_1,
        uvs_b,
    );

    let cuboid_handle = meshes.add(cuboid);
    commands.spawn((
        Mesh3d(cuboid_handle.clone()),
        MeshMaterial3d(materials_uv2.add(
            ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(
                        asset_server.load("uvchecker.png"),
                    ),
                    ..default()
                },
                extension: Uv2Extension {},
            },
        )),
        Transform::from_xyz(1.0, 0.5, 0.0),
        Rotate,
    ));

    commands.spawn((
        Mesh3d(cuboid_handle.clone()),
        MeshMaterial3d(materials_decals.add(
            ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(
                        asset_server.load("uvchecker.png"),
                    ),
                    ..default()
                },
                extension: DecalsExtension {
                    decals: Some(
                        asset_server.load("decals.png"),
                    ),
                },
            },
        )),
        Transform::from_xyz(-1.0, 0.5, 0.0),
        Rotate,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_things(
    mut q: Query<&mut Transform, With<Rotate>>,
    time: Res<Time>,
) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct DecalsExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[texture(100)]
    #[sampler(101)]
    decals: Option<Handle<Image>>,
}

impl MaterialExtension for DecalsExtension {
    fn fragment_shader() -> ShaderRef {
        "decals_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "decals_material.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct Uv2Extension {}

impl MaterialExtension for Uv2Extension {
    fn fragment_shader() -> ShaderRef {
        "uv2_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "uv2_material.wgsl".into()
    }
}
