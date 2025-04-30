use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (startup, trigger_event).chain(),
        )
        .add_observer(|trigger: Trigger<MyEvent>| {
            info!(
                target=?trigger.target(),
                event=?trigger.event(),
                "on_my_event",
            );
        })
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Root"),
        children![(
            Name::new("First"),
            children![(
                Name::new("Second"),
                children![(Name::new("Third"), Marker)]
            )]
        )],
    ));
}

fn trigger_event(
    mut commands: Commands,
    entity: Single<Entity, With<Marker>>,
) {
    info!("trigger");
    commands.trigger_targets(MyEvent { n: 0 }, *entity);
}

#[derive(Component)]
struct Marker;

#[derive(Debug, Event)]
#[event(auto_propagate, traversal = &'static ChildOf)]
struct MyEvent {
    n: usize,
}
