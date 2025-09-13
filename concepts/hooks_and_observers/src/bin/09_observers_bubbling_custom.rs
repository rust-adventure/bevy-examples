use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (startup, trigger_event).chain(),
        )
        .add_observer(
            |catch_fire: On<CatchFire>,
             names: Query<&Name>|
             -> Result {
                info!(
                    event=?catch_fire.event(),
                    name=?names.get(catch_fire.entity)?,
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
    commands.trigger(CatchFire { entity: *entity });
}

#[derive(Component)]
struct Flamable;

#[derive(Debug, EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static ItemOf)]
struct CatchFire {
    entity: Entity,
}

#[derive(Debug, Component)]
#[relationship(relationship_target = Inventory)]
pub struct ItemOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ItemOf)]
pub struct Inventory(Vec<Entity>);
