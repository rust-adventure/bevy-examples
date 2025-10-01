use bevy::{color::palettes::tailwind::*, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_lil_cube, update_inventory_display),
        )
        .run();
}

#[derive(Component)]
#[relationship(relationship_target = Inventory)]
pub struct ItemOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ItemOf)]
pub struct Inventory(Vec<Entity>);

#[derive(Component)]
struct LilCube;

#[derive(Component)]
struct DisplayImage(Handle<Image>);

#[derive(Component)]
struct ItemIndex(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        LilCube,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(SKY_400))),
        Transform::from_xyz(-2.0, 0.0, 0.0),
        children![
            (
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(-0.3, 0.2, 0.5),
            ),
            (
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.3, 0.2, 0.5),
            ),
        ],
        related!(
            Inventory[(
                Name::new("Bars"),
                DisplayImage(
                    asset_server.load("emotes/emote_bars.png")
                )
            ),
            (
                Name::new("Cloud"),
                DisplayImage(
                    asset_server.load("emotes/emote_cloud.png")
                )
            ),
            (
                Name::new("Cross"),
                DisplayImage(
                    asset_server.load("emotes/emote_cross.png")
                )
            )]
        ),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![item_slot(0), item_slot(1), item_slot(2)],
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

fn item_slot(index: u32) -> impl Bundle {
    (
        Node::default(),
        BackgroundColor(SLATE_400.into()),
        children![(
            Node {
                width: Val::Px(50.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Px(2.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(SLATE_700.into()),
            children![(
                Node {
                    height: Val::Px(30.),
                    width: Val::Px(30.),
                    ..default()
                },
                ImageNode::default(),
                ItemIndex(index),
            )],
        )],
    )
}

fn move_lil_cube(
    mut query: Query<&mut Transform, With<LilCube>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.translation.x = time.elapsed_secs().sin();
    }
}

fn update_inventory_display(
    inventory: Single<&Inventory, With<LilCube>>,
    items: Query<(&Name, &DisplayImage)>,
    mut item_displays: Query<(&mut ImageNode, &ItemIndex)>,
) {
    for (inventory_slot_id, entity) in
        inventory.into_inner().iter().enumerate()
    {
        let Ok(item) = items.get(entity) else {
            continue;
        };

        let Some((mut node, _)) =
            item_displays.iter_mut().find(|(_, index)| {
                inventory_slot_id == index.0 as usize
            })
        else {
            continue;
        };

        node.image = item.1.0.clone();
    }
}
