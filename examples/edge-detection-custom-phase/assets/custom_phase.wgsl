#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip
}
#import bevy_pbr::mesh_bindings::mesh;

@group(2) @binding(0) var<uniform> section_group: u32;

struct Vertex {
    // This is needed if you are using batching and/or gpu preprocessing
    // It's a built in so you don't need to define it in the vertex layout
    @builtin(instance_index) instance_index: u32,
    // Like we defined for the vertex layout
    // position is at location 0
    @location(0) position: vec3<f32>,
    #ifdef SECTION_COLORS
    @location(1) color: vec4<f32>
    #endif
};

// This is the output of the vertex shader and we also use it as the input for the fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    #ifdef SECTION_COLORS
    @location(1) color: vec4<f32>
    #endif
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // This is how bevy computes the world position
    // The vertex.instance_index is very important. Especially if you are using batching and gpu preprocessing
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);

    #ifdef SECTION_COLORS
    if vertex.color.r == 1. || vertex.color.r == 0. {
        out.color = vertex.color;
    } else {
        out.color = (vertex.color + vec4(
            f32(section_group),
            0.,
            0.,
            0.
        ) / 13.);
    }
    #endif

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    #ifdef SECTION_COLORS
        // return the typically hand authored section id
    return in.color;
    #else
    return vec4(0., 0., 0., 0.);
        // TODO: maybe discard one day?
        // discard;
    #endif
}