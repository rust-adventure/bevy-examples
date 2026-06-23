//! global vs local observers
//! custom events
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(|added: On<Add, ExampleComponent>| {
            info!(
                target= ?added.entity,
                event=?added.event(),
                "on_add (global)",
            );
        })
        .run();
}

#[derive(Debug, Component)]
struct ExampleComponent(i32);

fn startup(mut commands: Commands) {
    let id = commands.spawn_empty().observe(on_add).id();
    let id2 = commands.spawn_empty().observe(on_add).id();
    let id3 = commands.spawn_empty().observe(on_add).id();

    commands.entity(id).insert(ExampleComponent(0));
    commands.entity(id2).insert(ExampleComponent(10));
    commands.entity(id3).insert(ExampleComponent(100));
}

fn on_add(added: On<Add, ExampleComponent>) {
    info!(
        target= ?added.entity,
        event=?added.event(),
        "on_add",
    );
}
