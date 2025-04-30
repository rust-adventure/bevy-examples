//! global vs local observers
//! custom events
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(|trigger: Trigger<MyEvent>| {
            info!(
                target= ?trigger.target(),
                event=?trigger.event(),
                "on_my_event (global)",
            );
        })
        .run();
}

fn startup(mut commands: Commands) {
    let id =
        commands.spawn_empty().observe(on_my_event).id();
    let id2 =
        commands.spawn_empty().observe(on_my_event).id();
    let id3 =
        commands.spawn_empty().observe(on_my_event).id();

    commands.trigger_targets(MyEvent { n: 0 }, id);

    commands.trigger_targets(
        MyEvent { n: 1 },
        vec![id, id2, id3],
    );

    commands.trigger(MyEvent { n: 2 });
}

#[derive(Debug, Event)]
struct MyEvent {
    n: u32,
}

fn on_my_event(trigger: Trigger<MyEvent>) {
    info!(
        target= ?trigger.target(),
        event=?trigger.event(),
        "on_my_event",
    );
}
