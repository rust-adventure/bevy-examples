//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
};
use post_processing::PostProcessingCamera;
mod post_processing;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(post_processing::PostProcessingPlugin)
        .add_startup_system(setup)
        .add_system(main_camera_cube_rotator_system);

    app.run();
}

/// Marks the first camera cube (rendered to a texture.)
#[derive(Component)]
struct MainCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    asset_server.watch_for_changes().unwrap();

    let cube_handle =
        meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle =
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            reflectance: 0.02,
            unlit: false,
            ..default()
        });

    // The cube that will be rendered to the texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(
                Vec3::new(0.0, 0.0, 1.0),
            ),
            ..default()
        })
        .insert(MainCube);

    // Light
    // NOTE: Currently lights are ignoring render layers - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            0.0, 0.0, 10.0,
        )),
        ..default()
    });

    // Main camera, first to render
    commands
        .spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    Color::WHITE,
                ),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0.0, 0.0, 15.0),
            )
            .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(PostProcessingCamera);
}

/// Rotates the cube rendered by the main camera
fn main_camera_cube_rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}
