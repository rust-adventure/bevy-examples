use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{states::*, Action};

pub fn dash(
    In(entity): In<Entity>,
    query_player: Query<&ActionState<Action>>,
) -> Option<()> {
    let Ok(action_state) = query_player.get(entity) else {
        warn!("player components should be findable by player entity");
        return None;
    };

    action_state.just_pressed(&Action::Dash).then_some(())
}

pub fn dash_timer_complete(
    In(entity): In<Entity>,
    query_player: Query<&Dashing>,
) -> Option<()> {
    let Ok(dashing) = query_player.get(entity) else {
        warn!("player components should be findable by player entity");
        return None;
    };
    (dashing.frame >= dashing.total_frames - 1)
        .then_some(())
}

pub fn no_movement(
    In(entity): In<Entity>,
    query_player: Query<&ActionState<Action>>,
) -> Option<()> {
    let Ok(action_state) = query_player.get(entity) else {
        warn!("player components should be findable by player entity");
        return None;
    };

    let Some(axis_pair) =
        action_state.clamped_axis_pair(&Action::Move)
    else {
        return Some(());
    };
    (axis_pair.xy().length() <= 0.1).then_some(())
}
