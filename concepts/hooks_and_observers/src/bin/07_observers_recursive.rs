use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_observer(on_my_event)
        .run();
}

fn startup(mut commands: Commands) {
    commands.trigger(MyEvent { spawn_n_times: 4 });
    commands.trigger(MyEvent { spawn_n_times: 2 });
}

#[derive(Debug, Event)]
struct MyEvent {
    spawn_n_times: usize,
}

fn on_my_event(
    my_event: On<MyEvent>,
    mut commands: Commands,
) {
    info!(
        event=?my_event.event(),
        "on_my_event",
    );
    let Some(rest) =
        my_event.event().spawn_n_times.checked_sub(1)
    else {
        info!("done recursing");
        return;
    };
    commands.queue(MyCommand(rest));
    commands.trigger(MyEvent {
        spawn_n_times: rest,
    });
}

struct MyCommand(usize);

impl Command for MyCommand {
    fn apply(self, world: &mut World) -> () {
        info!(int=?self.0)
    }
}
