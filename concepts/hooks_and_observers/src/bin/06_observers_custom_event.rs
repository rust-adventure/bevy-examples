//! custom events
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(
            |my_global_event: On<MyGlobalEvent>| {
                info!(
                    event=?my_global_event.event(),
                    "on_my_event (global)",
                );
            },
        )
        .run();
}

fn startup(mut commands: Commands) {
    let id =
        commands.spawn_empty().observe(on_my_event).id();
    commands.trigger(MyEvent { n: 0, entity: id });

    commands.trigger(MyGlobalEvent { n: 15 });
}

#[derive(Debug, Event)]
struct MyGlobalEvent {
    n: u32,
}

#[derive(Debug, EntityEvent)]
struct MyEvent {
    n: u32,
    entity: Entity,
}

fn on_my_event(my_event: On<MyEvent>) {
    info!(
        event=?my_event.event(),
        "on_my_event",
    );
}
