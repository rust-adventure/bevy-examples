//! Observers *can* be on the same entity as the target
//! which makes Trigger::observer and Trigger::target the
//! same entity
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(
            |trigger: Trigger<OnAdd, ExampleComponent>| {
                info!(
                    observer=?trigger.observer(),
                    target= ?trigger.target(),
                    event=?trigger.event(),
                    "on_add (global)",
                );
            },
        )
        .run();
}

#[derive(Debug, Component)]
struct ExampleComponent(i32);

fn startup(mut commands: Commands) {
    let mut observer = Observer::new(special);

    let id = commands.spawn_empty().observe(on_add).id();
    let id2 = commands.spawn_empty().observe(on_add).id();
    let id3 = commands.spawn_empty().observe(on_add).id();

    observer.watch_entity(id3);

    commands.entity(id3).insert(observer);

    commands.entity(id).insert(ExampleComponent(0));
    commands.entity(id2).insert(ExampleComponent(10));
    commands.entity(id3).insert(ExampleComponent(100));
}

fn on_add(trigger: Trigger<OnAdd, ExampleComponent>) {
    info!(
        observer=?trigger.observer(),
        target= ?trigger.target(),
        event=?trigger.event(),
        "on_add",
    );
}

fn special(trigger: Trigger<OnAdd, ExampleComponent>) {
    info!(
        observer=?trigger.observer(),
        target= ?trigger.target(),
        event=?trigger.event(),
        "special",
    );
}
