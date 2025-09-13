use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    let id = commands.spawn(ExampleHooksComponent(0)).id();

    commands.entity(id).insert(ExampleHooksComponent(10));
    info!("");

    commands.entity(id).remove::<ExampleHooksComponent>();
    commands.entity(id).remove::<ExampleHooksComponent>();
    info!("");

    commands.entity(id).insert(ExampleHooksComponent(100));
    info!("");

    commands.entity(id).despawn();
}

#[derive(Debug, Component)]
#[component(on_add = log_on_add)]
#[component(on_insert = log_on_insert)]
#[component(on_replace = log_on_replace)]
#[component(on_remove = log_on_remove)]
#[component(on_despawn = log_on_despawn)]
struct ExampleHooksComponent(i32);

fn log_on_add(
    world: DeferredWorld,
    HookContext { entity, caller, .. }: HookContext,
) {
    let component_value =
        world.get::<ExampleHooksComponent>(entity).unwrap();
    info!(
        ?entity,
        line = caller
            .map(|loc| loc.line())
            .unwrap_or_default(),
        ?component_value,
        "on_add"
    );
}
fn log_on_insert(
    world: DeferredWorld,
    HookContext { entity, caller, .. }: HookContext,
) {
    let component_value =
        world.get::<ExampleHooksComponent>(entity).unwrap();
    info!(
        ?entity,
        line = caller
            .map(|loc| loc.line())
            .unwrap_or_default(),
        ?component_value,
        "on_insert"
    );
}
fn log_on_replace(
    world: DeferredWorld,
    HookContext { entity, caller, .. }: HookContext,
) {
    let component_value =
        world.get::<ExampleHooksComponent>(entity).unwrap();
    info!(
        ?entity,
        line = caller
            .map(|loc| loc.line())
            .unwrap_or_default(),
        ?component_value,
        "on_replace"
    );
}
fn log_on_remove(
    world: DeferredWorld,
    HookContext { entity, caller, .. }: HookContext,
) {
    let component_value =
        world.get::<ExampleHooksComponent>(entity).unwrap();
    info!(
        ?entity,
        line = caller
            .map(|loc| loc.line())
            .unwrap_or_default(),
        ?component_value,
        "on_remove"
    );
}

fn log_on_despawn(
    world: DeferredWorld,
    HookContext { entity, caller, .. }: HookContext,
) {
    let component_value =
        world.get::<ExampleHooksComponent>(entity).unwrap();
    info!(
        ?entity,
        line = caller
            .map(|loc| loc.line())
            .unwrap_or_default(),
        ?component_value,
        "on_despawn"
    );
}
