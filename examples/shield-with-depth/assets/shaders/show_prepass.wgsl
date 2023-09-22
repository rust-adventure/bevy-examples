#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::prepass_utils prepass_depth
#import bevy_pbr::prepass_utils prepass_normal
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    padding_1: u32,
    padding_2: u32,
}
@group(1) @binding(0)
var<uniform> settings: ShowPrepassSettings;

@fragment
fn fragment(
        mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let sample_index = 0u;
    if settings.show_depth == 1u {
        let depth = prepass_depth(mesh.position, sample_index);
        return vec4(depth, depth, depth, 1.0);
    } else if settings.show_normals == 1u {
        let normal = prepass_normal(mesh.position, sample_index);
        return vec4(normal, 1.0);
    }

    return vec4(0.0);
}