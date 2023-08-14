#import bevy_shader_utils::simplex_noise_3d simplex_noise_3d
#import bevy_pbr::mesh_vertex_output  MeshVertexOutput

struct CustomMaterial {
    offset: f32,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {
    if mesh.world_normal.x == 1.0 {
        return vec4(0.57, 0.76, 0.74, 1.0);
    } else if mesh.world_normal.y == 1.0 {
        return vec4(0.92, 0.90, 0.73, 1.0);
    } else {
        return vec4(0.29, 0.37, 0.57, 1.0);
    }
}
