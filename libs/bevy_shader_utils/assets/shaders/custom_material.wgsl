#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions
#import bevy_shader_utils::voronoise

struct CustomMaterial {
    color: vec4<f32>,
    time: f32
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

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
    var out: VertexOutput;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
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
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {

    var p: vec2<f32> = 0.5 - 0.5*cos( material.time + vec2(1.0,0.5) );
    p.y = 0.0;
    // var p = vec2(0.0,0.0);
    // var p = vec2(1.0,0.0);
    // var p = vec2(0.0,1.0);
    // var p = vec2(1.0,1.0);

    let f: f32 = voronoise(50.0 * (in.uv + material.time / 10.0), p.x, p.y);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

