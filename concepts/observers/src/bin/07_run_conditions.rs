use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, trigger_event)
        .add_observer(jump_when_held.run_if(
            |input: Res<ButtonInput<KeyCode>>| {
                input.pressed(KeyCode::KeyW)
            },
        ))
        .add_observer(jump_not_held.run_if(
            |input: Res<ButtonInput<KeyCode>>| {
                !input.pressed(KeyCode::KeyW)
            },
        ))
        .run();
}

fn jump_when_held(jump: On<Jump>) -> Result {
    info!(
        event=?jump.event(),
        "on_jump, held",
    );
    Ok(())
}

fn jump_not_held(jump: On<Jump>) -> Result {
    info!(
        event=?jump.event(),
        "on_jump, not held",
    );
    Ok(())
}

fn trigger_event(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        info!("trigger");
        commands.trigger(Jump);
    }
}

#[derive(Debug, Event)]
struct Jump;
