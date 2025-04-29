use bevy::{
    color::palettes::tailwind::*,
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins((
            DefaultPlugins,
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_lil_cube,
                log_item_of.run_if(input_just_pressed(
                    KeyCode::Space,
                )),
                remove_item_of.run_if(input_just_pressed(
                    KeyCode::KeyD,
                )),
            ),
        )
        .run();
}

#[derive(Debug, Component)]
#[relationship(relationship_target = Inventory)]
pub struct ItemOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ItemOf)]
pub struct Inventory(Vec<Entity>);

#[derive(Component)]
struct LilCube;

#[derive(Component)]
struct DisplayImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        LilCube,
        Name::new("LilCube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(SKY_400))),
        Transform::from_xyz(-2.0, 0.0, 0.0),
        children![
            (
                Name::new("RightEye"),
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(-0.3, 0.2, 0.5),
            ),
            (
                Name::new("LeftEye"),
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.3, 0.2, 0.5),
            ),
        ],
        Inventory::spawn((
            Spawn((
                Name::new("Bars"),
                DisplayImage(
                    asset_server
                        .load("emotes/emote_bars.png"),
                ),
            )),
            Spawn((
                Name::new("Cloud"),
                DisplayImage(
                    asset_server
                        .load("emotes/emote_cloud.png"),
                ),
            )),
            Spawn((
                Name::new("Cross"),
                DisplayImage(
                    asset_server
                        .load("emotes/emote_cross.png"),
                ),
            )),
        )),
    ));

    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.0, 2.0, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn move_lil_cube(
    mut query: Query<&mut Transform, With<LilCube>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.translation.x =
            time.elapsed_secs().sin() * 3.;
    }
}

fn log_item_of(
    query: Query<(&ItemOf, &Name)>,
    names: Query<&Name>,
) -> Result {
    for (item_of, name) in &query {
        info!(
            ?name,
            ?item_of,
            parent_name = ?names.get(item_of.0)?
        );
    }
    Ok(())
}

fn remove_item_of(
    mut commands: Commands,
    query: Query<(Entity, &Name, &DisplayImage, &ItemOf)>,
    transforms: Query<&GlobalTransform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result {
    let (entity, name, image, item_of) = query
        .iter()
        .next()
        .ok_or("no more items in inventory")?;

    let transform = transforms.get(item_of.0)?;
    commands.entity(entity).remove::<ItemOf>().insert((
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image.0.clone()),
            ..default()
        })),
        transform.compute_transform(),
    ));
    info!(?name, "removed item_of");

    Ok(())
}
