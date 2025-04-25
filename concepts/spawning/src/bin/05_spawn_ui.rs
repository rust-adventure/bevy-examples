use bevy::{color::palettes::tailwind::*, prelude::*};

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
        children![
            (
                Button,
                BackgroundColor(SKY_700.into()),
                Node {
                    padding: UiRect::all(Val::Px(5.)),
                    width: Val::Px(200.),
                    ..default()
                },
                children![(
                    Text::new("New Game"),
                    TextColor(SLATE_50.into())
                )]
            ),
            (
                Button,
                BackgroundColor(SKY_700.into()),
                Node {
                    padding: UiRect::all(Val::Px(5.)),
                    width: Val::Px(200.),
                    ..default()
                },
                children![(
                    Text::new("Options"),
                    TextColor(SLATE_50.into())
                )]
            ),
            (
                Button,
                BackgroundColor(SKY_700.into()),
                Node {
                    padding: UiRect::all(Val::Px(5.)),
                    width: Val::Px(200.),
                    ..default()
                },
                children![(
                    Text::new("Quit"),
                    TextColor(SLATE_50.into())
                )]
            )
        ],
    ));

    commands.spawn(Camera2d::default());
}
