#import bevy_shader_utils::simplex_noise_2d::simplex_noise_2d

// This shader is used for the compute_mesh example
// The actual work it does is not important for the example and
// has been hardcoded to return a cube mesh

// `vertex_start` is the starting offset of the mesh data in the *vertex_data* storage buffer
// `index_start` is the starting offset of the index data in the *index_data* storage buffer
struct DataRanges {
    num_vertices: u32,
    vertex_start: u32,
    vertex_end: u32,
    index_start: u32,
    index_end: u32,
}

@group(0) @binding(0) var<uniform> data_range: DataRanges;
@group(0) @binding(1) var<storage, read_write> vertex_data: array<f32>;
@group(0) @binding(2) var<storage, read_write> index_data: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let terrain_scale = 50.;
    for (var i = 0u; i < data_range.num_vertices; i++) {
        let offset = i * 12 + data_range.vertex_start;
        let x = vertex_data[offset];
        let z = vertex_data[offset + 2];
        // vertex_data[offset + 1] = (x + z) / 20.;//(f32(offset) / f32(data_range.num_vertices));
        let height = simplex_noise_2d(vec2(x / terrain_scale, z / terrain_scale));
        let root = vec3(x / terrain_scale, height, z / terrain_scale);
        let x_vec = vec3(x / terrain_scale + 0.01, height, z / terrain_scale);
        let z_vec = vec3(x / terrain_scale, height, z / terrain_scale + 0.01);
        let normal = cross(x_vec, z_vec);

        vertex_data[offset + 3] = normal.x;
        vertex_data[offset + 4] = normal.y;
        vertex_data[offset + 5] = normal.z;

        let tangent = cross(normal, vec3(0., 1., 0.));

        vertex_data[offset + 8] = tangent.x;
        vertex_data[offset + 9] = tangent.y;
        vertex_data[offset + 10] = tangent.z;
        vertex_data[offset + 11] = 1.;

        vertex_data[offset + 1] = simplex_noise_2d(vec2(x / terrain_scale, z / terrain_scale)) * 10.;
    }
}
