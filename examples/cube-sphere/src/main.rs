//! A CubeSphere implementation, in the same style
//! as upstream Bevy's mesh primitives.
//!
//! A CubeSphere is an "inflated" cube to form a
//! sphere. It avoids the downsides of UVSpheres
//! and IcoSpheres in a way that makes planet
//! surfaces more amenable. (ex: UVSphere
//! triangles pinch at the poles)
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin::default())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_sphere =
        CubeSphere.mesh().subdivisions(10).build();

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2,
        )),
    ));
    // cube
    commands.spawn((
        Wireframe,
        Mesh3d(meshes.add(cube_sphere)),
        MeshMaterial3d(
            materials.add(Color::srgb_u8(124, 144, 255)),
        ),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// CubeSphere and CubeSphereBuilder mirrors the
// patterns for meshes from Bevy
struct CubeSphere;

#[derive(Default)]
struct CubeSphereBuilder {
    pub subdivisions: u32,
}

impl CubeSphereBuilder {
    /// Sets the subdivisions of the plane mesh.
    ///
    /// 0 - is the original plane geometry, the 4
    /// points in the XZ plane.
    ///
    /// 1 - is split by 1 line in the middle of
    /// the plane on both the X axis and the Z
    /// axis,     resulting in a plane with 4
    /// quads / 8 triangles.
    ///
    /// 2 - is a plane split by 2 lines on both
    /// the X and Z axes, subdividing the plane
    /// into 3     equal sections along each
    /// axis, resulting in a plane with 9 quads /
    /// 18 triangles.
    #[inline]
    pub fn subdivisions(
        mut self,
        subdivisions: u32,
    ) -> Self {
        self.subdivisions = subdivisions;
        self
    }
}

impl Meshable for CubeSphere {
    type Output = CubeSphereBuilder;

    fn mesh(&self) -> Self::Output {
        CubeSphereBuilder { subdivisions: 10 }
    }
}

impl From<CubeSphere> for Mesh {
    fn from(cube: CubeSphere) -> Self {
        cube.mesh().build()
    }
}

// The actual builder logic. Could be implemented
// more efficiently and with fewer allocations.
// UVs are also "todo"
//
// This implementation builds up each face of the
// cube, and inflates.
impl MeshBuilder for CubeSphereBuilder {
    fn build(&self) -> Mesh {
        let directions = [
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::NEG_X,
            Vec3::X,
            Vec3::Z,
            Vec3::NEG_Z,
        ];

        let (vert_lists, triangle_lists): (
            Vec<Vec<Vec3>>,
            Vec<Vec<u32>>,
        ) = directions
            .iter()
            .map(|direction| {
                face(self.subdivisions, *direction)
            })
            .unzip();

        let vertices = vert_lists
            .iter()
            .flat_map(|v| v.iter().map(|v| [v.x, v.y, v.z]))
            .collect::<Vec<[f32; 3]>>();

        let triangle_list = triangle_lists
            .iter()
            .enumerate()
            .flat_map(|(face_id, list)| {
                // local_face_index indexes go up to
                // resolution^2 - 1.
                // so the last vertex in a face with a
                // resolution of 10 is index
                // 99 (100 indices, starting at 0).
                //
                // that makes the *index* of the second
                // face's vertices
                // start at 100 and end at 199.
                list.iter().map(move |local_idx| {
                    let num_indices = self.subdivisions
                        * self.subdivisions;
                    local_idx + face_id as u32 * num_indices
                })
            })
            .collect::<Vec<u32>>();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(triangle_list))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices.clone(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vertices,
        )
        // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}

/// build one face of the "cubesphere"
/// resolution is the per-face resolution,
/// the number of lines, which in turns means
/// resolution-1 squares per axis on each face
fn face(
    resolution: u32,
    local_up: Vec3,
) -> (Vec<Vec3>, Vec<u32>) {
    let axis_a = local_up.yzx();
    let axis_b = local_up.cross(axis_a);

    let mut vertices = Vec::with_capacity(
        resolution as usize * resolution as usize,
    );

    // a resolution of 10 means 10 lines
    // which is 9 squares per side,
    // with 2 triangles per square
    // 3 vertices per triangle
    let mut triangles = Vec::with_capacity(
        (resolution as usize - 1)
            * (resolution as usize - 1)
            * 6,
    );

    for y in 0..resolution {
        for x in 0..resolution {
            let i = x + y * resolution;
            let percent_x =
                x as f32 / (resolution - 1) as f32;
            let percent_y =
                y as f32 / (resolution - 1) as f32;

            let point_on_unit_cube = local_up
                + (percent_x - 0.5) * 2.0 * axis_a
                + (percent_y - 0.5) * 2.0 * axis_b;

            // vertices.push(point_on_unit_sphere);
            vertices.push(point_on_unit_cube.normalize());

            if x != resolution - 1 && y != resolution - 1 {
                // triangle list vertices 1
                triangles.push(i);
                triangles.push(i + resolution + 1);
                triangles.push(i + resolution);

                // triangle list vertices 2
                triangles.push(i);
                triangles.push(i + 1);
                triangles.push(i + resolution + 1);
            }
        }
    }
    (vertices, triangles)
}
