use bevy::{
    asset::embedded_asset,
    color::palettes::tailwind::*,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

/// To use the utility functions, add the
/// plugin to your app.
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_blockout::BlockoutPlugin;
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         BlockoutPlugin,
///     ));
/// ```
///
/// then use the material extension
///
/// ```ignore
/// commands.spawn((
///     Mesh3d(
///         meshes.add(Mesh::from(
///             Plane3d::default()
///                 .mesh()
///                 .size(40., 40.)
///                 .subdivisions(10),
///         )),
///     ),
///     Transform::from_xyz(0.0, 0.0, 0.0),
///     MeshMaterial3d(materials.add(ExtendedMaterial {
///         base: StandardMaterial {
///             base_color: SLATE_400.into(),
///             ..default()
///         },
///         extension: BlockoutMaterialExt::default(),
///     })),
/// ));
/// ```
pub struct BlockoutPlugin;

impl Plugin for BlockoutPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "materials/blockout.wgsl");

        app.register_type::<UseBlockoutMaterial>()
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<
                    StandardMaterial,
                    BlockoutMaterialExt,
                >,
            >::default());
    }
}

/// A Component that is mostly intended for use when
/// using Reflection data in places like Blender.
/// (perhaps using skein or another blender integration)
///
/// It will configure the Blockout material using the
/// pre-defined StandardMaterial from the glTF file.
#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_add_use_blockout_material)]
pub struct UseBlockoutMaterial;

///
/// The on_add hook that will run when the component is
/// added when spawning the glTF scene.
fn on_add_use_blockout_material(
    mut world: DeferredWorld,
    HookContext { entity, .. }: HookContext,
) {
    let material_handle = world
        .get::<MeshMaterial3d<StandardMaterial>>(entity)
        .unwrap();

    let base = world
        .get_resource::<Assets<StandardMaterial>>()
        .expect("couldn't get std material assets")
        .get(material_handle)
        .expect("no existing StandardMaterial for entity")
        .clone();

    let blockout_material = world
        .get_resource_mut::<Assets<
            ExtendedMaterial<
                StandardMaterial,
                BlockoutMaterialExt,
            >,
        >>()
        .expect("couldn't get blockout material assets")
        .add(ExtendedMaterial {
            base,
            extension: BlockoutMaterialExt::default(),
        });

    world
        .commands()
        .entity(entity)
        .remove::<MeshMaterial3d<StandardMaterial>>()
        .insert(MeshMaterial3d(blockout_material));
}

/// warning: not all of these are relevant at the moment
/// This is just an early release.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct BlockoutMaterialExt {
    #[uniform(100)]
    pub line_color: LinearRgba,
    #[uniform(100)]
    pub color: LinearRgba,
    #[uniform(100)]
    pub cell_multiplier: Vec2,
    #[uniform(100)]
    pub line_size: Vec2,
}

impl Default for BlockoutMaterialExt {
    fn default() -> Self {
        Self {
            line_color: Color::WHITE.into(), //SLATE_50.into(),
            color: SLATE_400.into(),
            cell_multiplier: Vec2::splat(10.),
            line_size: Vec2::splat(0.1),
        }
    }
}

impl MaterialExtension for BlockoutMaterialExt {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_blockout/materials/blockout.wgsl"
            .into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_blockout/materials/blockout.wgsl"
            .into()
    }
}
