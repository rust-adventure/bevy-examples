use bevy::{ecs::system::Command, prelude::*};
use bevy::{
    pbr::NotShadowCaster,
    render::mesh::VertexAttributeValues,
};

use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    *,
};
use std::{f32::consts::FRAC_PI_2, time::Duration};

use crate::materials::CubeMaterial;

#[derive(Copy, Clone)]
pub enum TweenEvents {
    SpawnNewCube = 0,
    DespawnSelf = 1,
}

impl TryFrom<u64> for TweenEvents {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use TweenEvents::*;

        match value {
            0 => Ok(SpawnNewCube),
            1 => Ok(DespawnSelf),
            _ => Err("invalid index for TweenEvents enum"
                .to_string()),
        }
    }
}

pub struct SpawnCube;

impl Command for SpawnCube {
    fn write(self, world: &mut World) {
        let cube_size = 0.2;
        let mut mesh =
            Mesh::from(shape::Cube { size: cube_size });
        if let Some(VertexAttributeValues::Float32x3(
            positions,
        )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[r, g, b]| {
                    [
                        (1. - *r) / 2.,
                        (1. - *g) / 2.,
                        (1. - *b) / 2.,
                        1.,
                    ]
                })
                .collect();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            );
        };

        let mesh_handle = {
            let mut meshes = world
                .get_resource_mut::<Assets<Mesh>>()
                .unwrap();
            meshes.add(mesh)
        };

        // mut meshes: ResMut<Assets<Mesh>>,
        let mut custom_materials = world
            .get_resource_mut::<Assets<CubeMaterial>>()
            .unwrap();

        let tween = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::BounceOut,
            // Animation time (one way only; for ping-pong it takes 2 seconds
            // to come back to start).
            Duration::from_secs(2),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformRotationLens {
                start: Quat::default(),
                end: Quat::from_rotation_y(FRAC_PI_2),
            },
        )
        .with_repeat_count(RepeatCount::Finite(1));

        let tween2 = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::BounceOut,
            // Animation time (one way only; for ping-pong it takes 2 seconds
            // to come back to start).
            Duration::from_secs(2),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformPositionLens {
                start: Vec3::new(0.0, 2.0, 0.0),
                end: Vec3::ZERO,
            },
        )
        .with_repeat_count(RepeatCount::Finite(1))
        .with_completed_event(
            TweenEvents::SpawnNewCube as u64,
        );

        let tween3 = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time (one way only; for ping-pong it takes 2 seconds
            // to come back to start).
            Duration::from_secs(1),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformPositionLens {
                start: Vec3::new(0.0, 0.0, 0.0),
                end: Vec3::new(0.0, -1.0, 0.0),
            },
        )
        .with_repeat_count(RepeatCount::Finite(1))
        .with_completed_event(
            TweenEvents::DespawnSelf as u64,
        );

        let material = custom_materials.add(CubeMaterial {
            color: Color::rgb(0.92, 0.90, 0.73),
        });
        world.spawn((
            MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(
                    0.0, 2.0, 0.0,
                ),
                ..default()
            },
            NotShadowCaster,
            Animator::new(Tracks::new([
                Sequence::from_single(tween),
                tween2
                    .then(Delay::new(
                        Duration::from_millis(200),
                    ))
                    .then(tween3),
            ])),
        ));
    }
}
