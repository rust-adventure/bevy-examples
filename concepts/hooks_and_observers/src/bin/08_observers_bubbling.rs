use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (startup, trigger_event).chain(),
        )
        .add_observer(
            |my_event: On<MyEvent>, names: Query<&Name>| {
                info!(
                    event=?my_event.event(),
                    name=?names.get(my_event.entity),
                    "on_my_event",
                );
            },
        )
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
    commands.trigger(MyEvent {
        n: 0,
        entity: *entity,
    });
}

#[derive(Component)]
struct Marker;

#[derive(Debug, EntityEvent)]
#[entity_event(propagate, auto_propagate)]
struct MyEvent {
    n: usize,
    entity: Entity,
}
