use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use eight_way_sprites::{
    player::{
        controller, player_state_machine, states::*,
        systems::*, Action, Player,
    },
    Facing,
};
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((
            DefaultPlugins,
            InputManagerPlugin::<Action>::default(),
            StateMachinePlugin,
        ))
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Next)
                .load_collection::<MyAssets>(),
        )
        .add_systems(OnEnter(AppState::Next), startup)
        .add_systems(
            Update,
            (dashing, idling, running, controller)
                .run_if(in_state(AppState::Next)),
        )
        .run();
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    // #[asset(texture_atlas_layout(
    //     tile_size_x = 100.,
    //     tile_size_y = 100.,
    //     columns = 24,
    //     rows = 16
    // ))]
    // #[asset(texture_atlas_layout(
    //     tile_size_x = 100.,
    //     tile_size_y = 100.,
    //     columns = 32,
    //     rows = 27
    // ))]
    // #[asset(texture_atlas_layout(
    //     tile_size_x = 200.,
    //     tile_size_y = 200.,
    //     columns = 32,
    //     rows = 27
    // ))]
    #[asset(texture_atlas_layout(
        tile_size_x = 100.,
        tile_size_y = 100.,
        columns = 33,
        rows = 32
    ))]
    druid_layout: Handle<TextureAtlasLayout>,
    // you can configure the sampler for the sprite sheet image
    #[asset(image(sampler = nearest))]
    #[asset(path = "druid-three-sheet.png")]
    druid: Handle<Image>,
}

fn startup(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
) {
    commands.spawn(Camera2dBundle::default());
    let input_map =
        InputMap::new([(Action::Dash, KeyCode::Space)])
            .with(Action::Move, DualAxis::left_stick())
            .with(Action::Move, VirtualDPad::wasd());

    commands.spawn((
        SpriteBundle {
            texture: my_assets.druid.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        TextureAtlas::from(my_assets.druid_layout.clone()),
        InputManagerBundle::with_map(input_map),
        Facing(Direction2d::NEG_Y),
        Player,
        Idling::default(),
        player_state_machine(),
    ));
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum AppState {
    #[default]
    AssetLoading,
    Next,
}
