use bevy::{
    color::palettes::tailwind::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SLATE_950.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10.),
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            parent
                .spawn(button("New Game"))
                .observe(|_: On<Pointer<Click>>| {
                    info!("New Game");
                });
            parent
                .spawn(button("Options"))
                .observe(|_: On<Pointer<Click>>| {
                    info!("Options");
                });
            parent
                .spawn(button("Quit"))
                .observe(|_: On<Pointer<Click>>| {
                    info!("Quit");
                });
        })),
    ));

    commands.spawn(Camera2d::default());
}

fn button<T: Into<String>>(text: T) -> impl Bundle {
    (
        Button,
        BackgroundColor(SKY_700.into()),
        Node {
            padding: px(5.).all(),
            width: px(200.),
            ..default()
        },
        children![(Text::new(text), TextColor(SLATE_50.into()))],
    )
}
