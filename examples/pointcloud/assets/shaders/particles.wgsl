
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct Time {
    time_since_startup: f32,
};
@group(1) @binding(0)
var<uniform> time: Time;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let thickness = 5.0;
    // higher is shorter
    let how_long_to_stay_in_opposite_state = 30.0;
    let frequency = 2.0;
    let sine = sin(frequency * time.time_since_startup + vertex.position.y + vertex.position.z);
    let position_diff = 1.0 - pow(thickness * sine, how_long_to_stay_in_opposite_state);
    let position = (vertex.normal * (smoothstep(0.0, 1.0, position_diff)) * 0.02) + vertex.position;

    var out: VertexOutput;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
    // out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    out.clip_position = mesh_position_world_to_clip(out.world_position);
    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};

@fragment
fn fragment(
    in: FragmentInput,
) -> @location(0) vec4<f32> {
    return vec4(in.color.rgb, 0.4);
}