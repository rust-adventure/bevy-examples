use bevy::{ecs::system::Command, prelude::*};
use bevy::{
    pbr::NotShadowCaster,
    render::mesh::VertexAttributeValues,
};

use std::{f32::consts::FRAC_PI_2, time::Duration};

use crate::materials::CubeMaterial;

pub struct SpawnCube;

impl Command for SpawnCube {
    fn apply(self, world: &mut World) {
        let cube_size = 0.2;
        let mesh =
            Mesh::from(Cuboid::from_length(cube_size));

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

        let material = custom_materials.add(CubeMaterial {
            color: Color::srgb(0.92, 0.90, 0.73),
        });
        world.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(0.0, 2.0, 0.0),
            NotShadowCaster,
        ));
    }
}
