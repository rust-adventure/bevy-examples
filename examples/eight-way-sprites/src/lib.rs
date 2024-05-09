use bevy::prelude::*;
pub mod animation;
pub mod player;

#[derive(Component, Debug)]
pub struct Facing(pub Direction2d);
