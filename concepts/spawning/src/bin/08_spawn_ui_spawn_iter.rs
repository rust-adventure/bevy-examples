use bevy::{
    color::palettes::tailwind::*,
    ecs::{
        relationship::RelatedSpawner,
        spawn::{SpawnIter, SpawnWith},
    },
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
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..default()
        },
        Children::spawn((
            SpawnWith(
                |parent: &mut RelatedSpawner<ChildOf>| {
                    parent
                        .spawn(button("New Game"))
                        .observe(
                            |_: On<Pointer<Click>>| {
                                info!("New Game");
                            },
                        );
                    parent
                        .spawn(button("Options"))
                        .observe(
                            |_: On<Pointer<Click>>| {
                                info!("Options");
                            },
                        );
                    parent.spawn(button("Quit")).observe(
                        |_: On<Pointer<Click>>| {
                            info!("Quit");
                        },
                    );
                },
            ),
            Spawn((
                Node {
                    width: Val::Px(200.),
                    justify_content:
                        JustifyContent::SpaceBetween,
                    ..default()
                },
                Children::spawn(SpawnIter(
                    (0..9).into_iter().map(|index| {
                        Text::new(index.to_string())
                    }),
                )),
            )),
        )),
    ));

    commands.spawn(Camera2d::default());
}

fn button<T: Into<String>>(text: T) -> impl Bundle {
    (
        Button,
        BackgroundColor(SKY_700.into()),
        Node {
            padding: UiRect::all(Val::Px(5.)),
            width: Val::Px(200.),
            ..default()
        },
        children![(
            Text::new(text),
            TextColor(SLATE_50.into())
        )],
    )
}
