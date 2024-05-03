use bevy::diagnostic::{
    DiagnosticsStore, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::{app::AppExit, window::WindowMode};
use bevy::{
    asset::RecursiveDependencyLoadState,
    ui::widget::UiImageSize,
};
use bevy_asset_loader::prelude::*;
use bevy_tweening::{
    lens::{UiBackgroundColorLens, UiPositionLens},
    *,
};
use iyes_progress::{
    Progress, ProgressCounter, ProgressPlugin,
    ProgressSystem,
};
use start_menu_hades::loading_material::{
    AppStartLoadingIndicator, LoadingUiMaterial,
};
use strum::FromRepr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.84, 0.74, 0.65,
        )))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Fullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            ProgressPlugin::new(MyStates::AssetLoading)
                .continue_to(MyStates::Next),
            FrameTimeDiagnosticsPlugin,
            TweeningPlugin,
            UiMaterialPlugin::<LoadingUiMaterial>::default(
            ),
        ))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .load_collection::<TextureAssets>()
                .load_collection::<AudioAssets>(),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnExit(MyStates::AssetLoading),
            on_exit_loading,
        )
        .add_systems(OnEnter(MyStates::Next), expect)
        .add_systems(
            Update,
            (
                track_fake_long_task.track_progress(),
                second_fake.track_progress(),
                third_fake.track_progress(),
                fourth_fake.track_progress(),
                print_progress,
            )
                .chain()
                .run_if(in_state(MyStates::AssetLoading))
                .after(LoadingStateSet(
                    MyStates::AssetLoading,
                )),
        )
        .add_systems(
            Update,
            ((button_system, icon_button_system)
                .run_if(in_state(MyStates::Next)),),
        )
        .add_systems(
            Update,
            (
                remove_intro_tweens, // .run_if(on_event::<TweenCompleted>()),
                asset_animator_system::<LoadingUiMaterial>,
            ),
        )
        .run();
}

// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
const DURATION_LONG_TASK_IN_SECS: f64 = 8.0;

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "intro-text.wav")]
    plop: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
struct TextureAssets {
    #[asset(path = "Jersey15-Regular.ttf")]
    jersey15: Handle<Font>,
    #[asset(path = "player-test.png")]
    player: Handle<Image>,
    #[asset(path = "backdrop-blurred.png")]
    backdrop: Handle<Image>,
    #[asset(path = "navigation_e.png")]
    arrow_right: Handle<Image>,
    #[asset(path = "pointer_scifi_a.png")]
    pointer_a: Handle<Image>,
    #[asset(path = "pointer_scifi_b.png")]
    pointer_b: Handle<Image>,
    #[asset(path = "button-bg.png")]
    button_bg: Handle<Image>,
    #[asset(path = "logo-chrisbiscardi.png")]
    logo_chrisbiscardi: Handle<Image>,
    #[asset(path = "logo-chrisbiscardi-faded.png")]
    logo_chrisbiscardi_faded: Handle<Image>,
    #[asset(path = "logo-discord.png")]
    logo_discord: Handle<Image>,
    #[asset(path = "logo-discord-faded.png")]
    logo_discord_faded: Handle<Image>,
    #[asset(path = "logo-youtube.png")]
    logo_youtube: Handle<Image>,
    #[asset(path = "logo-youtube-faded.png")]
    logo_youtube_faded: Handle<Image>,
    #[asset(path = "logo-rust-adventure.png")]
    logo_rust_adventure: Handle<Image>,
}

fn track_fake_long_task(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64()
        > DURATION_LONG_TASK_IN_SECS
    {
        info_once!("Long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}
fn second_fake(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64() > 4. {
        info_once!("Second long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}
fn third_fake(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64() > 6. {
        info_once!("Third long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}
fn fourth_fake(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64() > 7. {
        info_once!("fourth long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}

fn spawn_camera(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut ui_materials: ResMut<Assets<LoadingUiMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialNodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Px(125.),
                align_self: AlignSelf::Center,
                ..default()
            },
            material: ui_materials.add(LoadingUiMaterial {
                color: Color::rgb(0.84, 0.74, 0.65)
                    .rgba_linear_to_vec4(),
                progress: 0.0,
                texture: asset_server
                    .load("pattern_40.png"),
            }),
            ..default()
        },
        AppStartLoadingIndicator,
    ));
}

#[derive(Component)]
enum ButtonBehavior {
    Quit,
    Settings,
    Play,
}
#[derive(Component)]
enum IconButtonBehavior {
    Www,
    Discord,
    Personal,
    YouTube,
}
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ButtonBehavior,
            &mut BackgroundColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut app: EventWriter<AppExit>,
    mut arrow_query: Query<&mut Visibility, With<UiImage>>,
) {
    for (
        interaction,
        behavior,
        mut background_color,
        children,
    ) in &mut interaction_query
    {
        let mut text =
            text_query.get_mut(children[1]).unwrap();
        let mut visibility =
            arrow_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = Color::GREEN;
                match behavior {
                    ButtonBehavior::Quit => {
                        app.send(AppExit);
                    }
                    ButtonBehavior::Settings => {
                        dbg!("settings");
                    }
                    ButtonBehavior::Play => {
                        dbg!("play");
                    }
                }
                *visibility = Visibility::Visible;
                *background_color = Color::NONE.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color =
                    Color::SEA_GREEN;
                *visibility = Visibility::Visible;
                *background_color =
                    Color::rgba(1., 1., 1., 1.).into();
            }
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE;
                *background_color = Color::NONE.into();
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn icon_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &IconButtonBehavior,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<(&mut Visibility, &mut Text)>,
    mut image_query: Query<(
        &mut UiImage,
        &mut BackgroundColor,
    )>,
    texture_assets: Res<TextureAssets>,
) {
    for (interaction, behavior, children) in
        &mut interaction_query
    {
        let (mut visibility, mut text) =
            text_query.get_mut(children[1]).unwrap();
        let (mut image, mut background_color) =
            image_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = Color::GREEN;
                match behavior {
                    IconButtonBehavior::Www => {
                        dbg!("click www");
                    }
                    IconButtonBehavior::Discord => {
                        dbg!("click discord");
                    }
                    IconButtonBehavior::Personal => {
                        dbg!("click personal");
                    }
                    IconButtonBehavior::YouTube => {
                        dbg!("click youtube");
                    }
                }
                *visibility = Visibility::Visible;
                // *background_color =
                //     Color::rgba(1., 1., 1., 1.).into();
            }
            Interaction::Hovered => {
                dbg!("hovered");
                match behavior {
                    IconButtonBehavior::Www => {}
                    IconButtonBehavior::Discord => {
                        *image = texture_assets
                            .logo_discord
                            .clone()
                            .into();
                    }
                    IconButtonBehavior::Personal => {
                        *image = texture_assets
                            .logo_chrisbiscardi
                            .clone()
                            .into();
                    }
                    IconButtonBehavior::YouTube => {
                        *image = texture_assets
                            .logo_youtube
                            .clone()
                            .into();
                    }
                }
                text.sections[0].style.color =
                    Color::SEA_GREEN;
                *visibility = Visibility::Visible;
                *background_color =
                    Color::rgba(1., 1., 1., 1.0).into();
            }
            Interaction::None => {
                match behavior {
                    IconButtonBehavior::Www => {}
                    IconButtonBehavior::Discord => {
                        *image = texture_assets
                            .logo_discord_faded
                            .clone()
                            .into();
                    }
                    IconButtonBehavior::Personal => {
                        *image = texture_assets
                            .logo_chrisbiscardi_faded
                            .clone()
                            .into();
                    }
                    IconButtonBehavior::YouTube => {
                        *image = texture_assets
                            .logo_youtube_faded
                            .clone()
                            .into();
                    }
                }

                text.sections[0].style.color = Color::WHITE;

                *visibility = Visibility::Hidden;
                *background_color =
                    Color::rgba(1., 1., 1., 0.5).into();
            }
        }
    }
}

#[derive(FromRepr)]
enum TweenCompletedAction {
    Remove = 42,
}
fn remove_intro_tweens(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
) {
    debug_once!("remove_intro_tweens");
    for ev in reader.read() {
        dbg!("TweenEvent");
        match TweenCompletedAction::from_repr(
            ev.user_data as usize,
        ) {
            Some(TweenCompletedAction::Remove) => {
                commands
                    .entity(ev.entity)
                    .despawn_recursive();
            }
            None => {
                warn!(
                    "unhandled TweenCompleted event {:?}",
                    ev.entity
                );
            }
        };
    }
}

struct LoadingColorLens {
    color: Vec4,
}

impl Lens<LoadingUiMaterial> for LoadingColorLens {
    fn lerp(
        &mut self,
        target: &mut LoadingUiMaterial,
        ratio: f32,
    ) {
        target.color = Vec4::new(
            self.color.x,
            self.color.y,
            self.color.z,
            1. - ratio,
        );
    }
}

fn on_exit_loading(
    mut commands: Commands,
    loading_indicator_query: Query<
        Entity,
        With<AppStartLoadingIndicator>,
    >,
) {
    for indicator in &loading_indicator_query {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_millis(100),
            LoadingColorLens {
                color: Color::rgb(0.84, 0.74, 0.65)
                    .rgba_linear_to_vec4(),
            },
        )
        .with_repeat_count(RepeatCount::Finite(1))
        .with_completed_event(42);

        commands
            .entity(indicator)
            .insert(AssetAnimator::new(tween));
    }
}
fn expect(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    texture_assets: Res<TextureAssets>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut quit: EventWriter<AppExit>,
) {
    commands.spawn(AudioBundle {
        source: audio_assets.plop.clone(),
        ..default()
    });

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_millis(750),
        UiBackgroundColorLens {
            start: Color::rgba(1., 1., 1., 0.),
            end: Color::rgba(1., 1., 1., 0.8),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));

    commands.spawn((
        ImageBundle {
            style: Style {
                height: Val::Vh(100.),
                width: Val::Vw(100.),
                ..default()
            },
            image: texture_assets.backdrop.clone().into(),
            background_color: Color::hsla(1., 1., 1., 0.)
                .into(),
            ..default()
        },
        Animator::new(tween),
    ));

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_millis(250),
        UiBackgroundColorLens {
            start: Color::rgba(1., 1., 1., 0.),
            end: Color::rgba(1., 1., 1., 1.),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));

    let tween_pos = Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_millis(250),
        UiPositionLens {
            start: UiRect::left(Val::Px(-100.)),
            end: UiRect::left(Val::Px(0.)),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(10.),
                    right: Val::Percent(10.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                },
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        height: Val::Percent(80.),
                        width: Val::Auto,
                        ..default()
                    },
                    image: texture_assets
                        .player
                        .clone()
                        .into(),
                    ..default()
                },
                Animator::new(tween),
                Animator::new(tween_pos),
            ));
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(10.),
                    right: Val::Percent(70.),
                    top: Val::Percent(20.),
                    bottom: Val::Percent(10.),
                },
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            padding: UiRect {
                                left: Val::Px(35.),
                                right: Val::Px(10.),
                                top: Val::Px(20.),
                                bottom: Val::Px(20.),
                            },
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        image: texture_assets
                            .button_bg
                            .clone()
                            .into(),
                        ..default()
                    },
                    ButtonBehavior::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            position_type:
                                PositionType::Absolute,
                            left: Val::Px(-75.),
                            ..default()
                        },
                        image: texture_assets
                            .arrow_right
                            .clone()
                            .into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    });

                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 120.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            padding: UiRect {
                                left: Val::Px(35.),
                                right: Val::Px(10.),
                                top: Val::Px(20.),
                                bottom: Val::Px(20.),
                            },
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        image: texture_assets
                            .button_bg
                            .clone()
                            .into(),
                        ..default()
                    },
                    ButtonBehavior::Settings,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            position_type:
                                PositionType::Absolute,
                            left: Val::Px(-75.),
                            ..default()
                        },
                        image: texture_assets
                            .arrow_right
                            .clone()
                            .into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Options",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 120.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            padding: UiRect {
                                left: Val::Px(35.),
                                right: Val::Px(10.),
                                top: Val::Px(20.),
                                bottom: Val::Px(20.),
                            },
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        image: texture_assets
                            .button_bg
                            .clone()
                            .into(),
                        ..default()
                    },
                    ButtonBehavior::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            position_type:
                                PositionType::Absolute,
                            left: Val::Px(-75.),
                            ..default()
                        },
                        image: texture_assets
                            .arrow_right
                            .clone()
                            .into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 120.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    // logos
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(10.),
                    right: Val::Percent(0.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(10.),
                },
                align_self: AlignSelf::End,
                align_items: AlignItems::End,
                column_gap: Val::Px(50.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            // width: Val::Percent(100.),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        ..default()
                    },
                    IconButtonBehavior::Www,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Px(125.),
                            ..default()
                        },
                        image: texture_assets
                            .logo_rust_adventure
                            .clone()
                            .into(),
                        background_color: Color::rgba(
                            1., 1., 1., 0.5,
                        )
                        .into(),
                        ..default()
                    });
                    parent
                        .spawn(TextBundle::from_section(
                            "Website",
                            TextStyle {
                                font: texture_assets
                                    .jersey15
                                    .clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ))
                        .insert(Visibility::Hidden);
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            // width: Val::Percent(100.),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        ..default()
                    },
                    IconButtonBehavior::Discord,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Px(125.),
                            ..default()
                        },
                        image: texture_assets
                            .logo_discord_faded
                            .clone()
                            .into(),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "discord",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            // width: Val::Percent(100.),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        ..default()
                    },
                    IconButtonBehavior::Personal,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Px(125.),
                            ..default()
                        },
                        image: texture_assets
                            .logo_chrisbiscardi_faded
                            .clone()
                            .into(),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "www",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            // width: Val::Percent(100.),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE
                            .into(),
                        ..default()
                    },
                    IconButtonBehavior::YouTube,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Px(125.),
                            ..default()
                        },
                        image: texture_assets
                            .logo_youtube_faded
                            .clone()
                            .into(),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "youtube",
                        TextStyle {
                            font: texture_assets
                                .jersey15
                                .clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(0.),
                    right: Val::Px(30.),
                    top: Val::Px(30.),
                    bottom: Val::Percent(0.),
                },
                align_self: AlignSelf::Start,
                justify_content: JustifyContent::FlexEnd,
                // align_items: AlignItems::End,
                // column_gap: Val::Px(50.),/
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("version {VERSION}"),
                TextStyle {
                    font: texture_assets.jersey15.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
    mut ui_materials: ResMut<Assets<LoadingUiMaterial>>,
) {
    if let Some(progress) =
        progress.map(|counter| counter.progress())
    {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
            for (_, mat) in ui_materials.iter_mut() {
                mat.progress = progress.done as f32
                    / progress.total as f32;
            }
        }
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
