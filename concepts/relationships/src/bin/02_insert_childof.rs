use bevy::{
    color::palettes::tailwind::*,
    input::common_conditions::input_just_pressed,
    prelude::*,
};
// use bevy_inspector_egui::{
//     bevy_egui::EguiPlugin,
// quick::WorldInspectorPlugin, };

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins((
            DefaultPlugins,
            // EguiPlugin {
            //     enable_multipass_for_primary_context:
            // true, },
            // WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_lil_cube,
                log_child_of.run_if(input_just_pressed(
                    KeyCode::Space,
                )),
                remove_child_of.run_if(input_just_pressed(
                    KeyCode::KeyD,
                )),
            ),
        )
        .run();
}

#[derive(Component)]
struct LilCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lil_cube = commands
        .spawn((
            LilCube,
            Name::new("LilCube"),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(
                materials.add(Color::from(SKY_400)),
            ),
            Transform::from_xyz(-2.0, 0.0, 0.0),
        ))
        .id();

    commands.spawn((
        Name::new("RightEye"),
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(-0.3, 0.2, 0.5),
        ChildOf(lil_cube),
    ));

    commands.spawn((
        Name::new("LeftEye"),
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.3, 0.2, 0.5),
        ChildOf(lil_cube),
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
        transform.translation.x = time.elapsed_secs().sin();
    }
}

fn log_child_of(
    query: Query<(&ChildOf, &Name)>,
    names: Query<&Name>,
) -> Result {
    for (child_of, name) in &query {
        info!(
            ?name,
            ?child_of,
            parent_name = ?names.get(child_of.parent())?
        );
    }
    Ok(())
}

fn remove_child_of(
    mut commands: Commands,
    query: Query<(Entity, &Name), With<ChildOf>>,
) -> Result {
    for (entity, name) in &query {
        commands.entity(entity).remove::<ChildOf>();
        info!(?name, "removed child_of");
    }
    Ok(())
}
