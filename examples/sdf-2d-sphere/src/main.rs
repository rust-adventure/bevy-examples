use bevy::{
    asset::AssetServerSettings, prelude::*,
    sprite::MaterialMesh2dBundle,
};
use sdf_2d_sphere::materials::{
    SdfMaterial, SdfMaterialPlugin,
};

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SdfMaterialPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SdfMaterial>>,
) {
    // quad
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::default()))
            .into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(512.)),
        material: materials
            .add(SdfMaterial { color: Color::BLUE }),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}
