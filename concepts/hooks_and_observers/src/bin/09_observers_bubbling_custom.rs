use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (startup, trigger_event).chain(),
        )
        .add_observer(
            |trigger: Trigger<CatchFire>,
             names: Query<&Name>|
             -> Result {
                info!(
                    target=?trigger.target(),
                    event=?trigger.event(),
                    name=?names.get(trigger.target())?,
                    "on_my_event",
                );
                Ok(())
            },
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Chest"),
        related!(Inventory[
            Name::new("Money"),
            (Name::new("Paper"), Flamable),
            Name::new("Matchsticks"),
        ]),
    ));
}

fn trigger_event(
    mut commands: Commands,
    entity: Single<Entity, With<Flamable>>,
) {
    info!("trigger");
    commands.trigger_targets(CatchFire, *entity);
}

#[derive(Component)]
struct Flamable;

#[derive(Debug, Event)]
#[event(auto_propagate, traversal = &'static ItemOf)]
struct CatchFire;

#[derive(Debug, Component)]
#[relationship(relationship_target = Inventory)]
pub struct ItemOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ItemOf)]
pub struct Inventory(Vec<Entity>);
