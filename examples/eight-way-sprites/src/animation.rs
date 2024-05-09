use bevy::prelude::*;
use std::f32::consts::PI;

use crate::Facing;

pub enum Animation {
    Running,
    Idling,
    Dashing,
}
/// a temporary function to get the base index of an animation
/// and a direction
pub fn base_animation_index(
    animation: &Animation,
    facing: &Facing,
) -> Result<usize, String> {
    let num_spritesheet_directions = 8;

    let running_frames = 48;
    let running_base_index = 0;

    let idle_frames = 60;
    let idle_base_index =
        running_frames * num_spritesheet_directions;

    let dash_frames = 24;
    let dash_base_index = idle_base_index
        + idle_frames * num_spritesheet_directions;

    let facing_index = sprite_direction_index(facing)?;

    Ok(match animation {
        Animation::Running => {
            running_base_index
                + running_frames * facing_index
        }
        Animation::Idling => {
            idle_base_index + idle_frames * facing_index
        }
        Animation::Dashing => {
            dash_base_index + dash_frames * facing_index
        }
    })
}
pub fn sprite_direction_index(
    facing: &Facing,
) -> Result<usize, String> {
    // direction here is a number from -1..1
    let real_direction = facing.0.to_angle() / PI;

    // number of directions rendered out from blender
    let num_directions_in_spritesheet = 8;

    // directional index based on 2_PI/num_directions_in_spritesheet segments
    let real_direction_index = ((real_direction + 1.)
        * (num_directions_in_spritesheet as f32 / 2.))
        .round() as u32
        - 1;

    // This sprite direction should line up with the order of rendering.
    // start at "SouthWest" and go counter-clockwise around the character
    // to render in the right order
    match real_direction_index {
        5 => Ok(1), // North
        4 => Ok(2), // NorthEast
        3 => Ok(3), // East
        2 => Ok(4), // SouthEast
        1 => Ok(5), // South
        0 => Ok(0), // SouthWest
        7 => Ok(6), // West
        6 => Ok(7), // NorthWest
        _ => {
            let msg = "real_direction_index is the index of the spritesheet grouping for a specific navigational direction. Facing provided an index that was out of range.";
            error!(msg);
            return Err(msg.to_string());
        }
    }
}
