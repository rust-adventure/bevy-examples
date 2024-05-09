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
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (
        _entity,
        mut atlas,
        facing,
        mut dashing,
        mut transform,
    ) in &mut query
    {
        let base_dash_speed = 300.;
        let distance: Vec2 = *facing.0
            * base_dash_speed
            * time.delta_seconds();
        transform.translation +=
            Vec3::new(distance.x, distance.y, 0.);

        dashing.timer.tick(time.delta());
        if dashing.timer.finished() {
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
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    let base_run_speed = 150.;

    for (
        _entity,
        mut atlas,
        facing,
        mut running,
        mut transform,
    ) in &mut query
    {
        let distance: Vec2 = *facing.0
            * base_run_speed
            * time.delta_seconds();
        transform.translation +=
            Vec3::new(distance.x, distance.y, 0.);

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
