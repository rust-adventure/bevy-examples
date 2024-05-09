use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::Facing;

use self::{states::*, triggers::*};
pub mod states;
pub mod systems;
pub mod triggers;

#[derive(Component)]
pub struct Player;

#[derive(
    Actionlike,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Debug,
    Reflect,
)]
pub enum Action {
    Move,
    Dash,
}

pub fn player_state_machine() -> StateMachine {
    StateMachine::default()
        // Whenever the player presses dash, dash
        .trans::<Idling, _>(dash, Dashing::default())
        .trans::<Idling, _>(
            has_directional_input,
            Running::default(),
        )
        .trans::<Running, _>(dash, Dashing::default())
        .trans::<Running, _>(
            has_directional_input.not(),
            Idling::default(),
        )
        // when the timer runs out and the user isn't pressing a direction, idle
        .trans::<Dashing, _>(
            dash_timer_complete
                .and(has_directional_input.not()),
            Idling::default(),
        )
        // when the timer runs out and the user is pressing a direction, Run
        .trans::<Dashing, _>(
            dash_timer_complete.and(has_directional_input),
            Running::default(),
        )
        .set_trans_logging(true)
}

pub fn controller(
    mut query: Query<
        (&ActionState<Action>, &mut Facing),
        (
            With<Player>,
            Or<(With<Running>, With<Idling>)>,
        ),
    >,
) {
    for (action_state, mut facing) in &mut query {
        if action_state.pressed(&Action::Move) {
            let axis_pair = action_state
                .clamped_axis_pair(&Action::Move)
                .unwrap();
            if axis_pair.xy().length() > 0.1 {
                *facing = Facing(
                    Direction2d::new(axis_pair.xy())
                        .expect(
                            "should be a valid direction",
                        ),
                );
            }
        }
    }
}
