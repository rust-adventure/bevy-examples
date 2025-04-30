use bevy::{ecs::world::OnDespawn, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(on_add)
        .add_observer(on_insert)
        .add_observer(on_replace)
        .add_observer(on_remove)
        .add_observer(on_despawn)
        .run();
}

fn startup(mut commands: Commands) {
    let id = commands.spawn(ExampleComponent(0)).id();

    commands.entity(id).insert(ExampleComponent(10));
    info!("");

    commands.entity(id).remove::<ExampleComponent>();
    commands.entity(id).remove::<ExampleComponent>();
    info!("");

    commands.entity(id).insert(ExampleComponent(100));
    info!("");

    commands.entity(id).despawn();
}

#[derive(Debug, Component)]
struct ExampleComponent(i32);

fn on_add(
    trigger: Trigger<OnAdd, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        target = ?trigger.target(),
        value = ?query.get(trigger.target()),
        "on_add",
    );
}

fn on_insert(
    trigger: Trigger<OnInsert, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        target = ?trigger.target(),
        value = ?query.get(trigger.target()),
        "on_insert",
    );
}
fn on_replace(
    trigger: Trigger<OnReplace, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        target = ?trigger.target(),
        value = ?query.get(trigger.target()),
        "on_replace",
    );
}
fn on_remove(
    trigger: Trigger<OnRemove, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        target = ?trigger.target(),
        value = ?query.get(trigger.target()),
        "on_remove",
    );
}
fn on_despawn(
    trigger: Trigger<OnDespawn, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        target = ?trigger.target(),
        value = ?query.get(trigger.target()),
        "on_despawn",
    );
}
