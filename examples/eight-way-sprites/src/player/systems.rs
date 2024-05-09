use bevy::prelude::*;

use crate::{
    animation::{base_animation_index, Animation},
    Facing,
};

use super::states::*;

pub fn dashing(
    mut query: Query<(
        Entity,
        &mut TextureAtlas,
        &Facing,
        &mut Dashing,
    )>,
    time: Res<Time>,
) {
    // let base_index = ;
    for (_entity, mut atlas, facing, mut dashing) in
        &mut query
    {
        dashing.timer.tick(time.delta());
        if dashing.timer.finished() {
            // increase frame count
            dashing.frame = (dashing.frame + 1)
                .min(dashing.total_frames - 1);
            let Ok(base_index) = base_animation_index(
                &Animation::Dashing,
                &facing,
            ) else {
                continue;
            };
            atlas.index =
                base_index + dashing.frame as usize;
        }
    }
}

pub fn idling(
    mut query: Query<(
        Entity,
        &mut TextureAtlas,
        &Facing,
        &mut Idling,
    )>,
    time: Res<Time>,
) {
    // let base_index = ;
    for (_entity, mut atlas, facing, mut idling) in
        &mut query
    {
        idling.timer.tick(time.delta());
        if idling.timer.finished() {
            idling.frame =
                (idling.frame + 1) % idling.total_frames;

            let Ok(base_index) = base_animation_index(
                &Animation::Idling,
                &facing,
            ) else {
                continue;
            };

            atlas.index =
                base_index + idling.frame as usize;
        }
    }
}

pub fn running(
    mut query: Query<(
        Entity,
        &mut TextureAtlas,
        &Facing,
        &mut Running,
    )>,
    time: Res<Time>,
) {
    // let base_index = ;
    for (_entity, mut atlas, facing, mut running) in
        &mut query
    {
        running.timer.tick(time.delta());
        if running.timer.finished() {
            running.frame =
                (running.frame + 1) % running.total_frames;

            let Ok(base_index) = base_animation_index(
                &Animation::Running,
                &facing,
            ) else {
                continue;
            };

            atlas.index =
                base_index + running.frame as usize;
        }
    }
}
