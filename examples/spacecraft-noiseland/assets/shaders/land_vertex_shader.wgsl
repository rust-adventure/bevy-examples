#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_functions,
    view_transformations::position_world_to_clip,
    mesh_bindings
}

#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d

@group(2) @binding(0)
var<uniform> ship_position: vec3<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    // The world-space position of the vertex.
    @location(0) position: vec3<f32>,
    // The color of the vertex.
    @location(1) color: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
 var noise = simplex_noise_3d(vertex.position);

    var out: VertexOutput;
    out.color = vec4<f32>(0.5, noise, 0.5, 1.0);

    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(vertex.position.x, noise, vertex.position.z, 1.0)
    );

    out.world_position.y = out.world_position.y - 4.0 * sin(ship_position.z - out.world_position.z) / 100.;

    out.position = position_world_to_clip(out.world_position.xyz);

    return out;
}
