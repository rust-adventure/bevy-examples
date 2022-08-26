#import bevy_shader_utils::simplex_noise_2d

#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.uv = vertex.uv;
    return out;
}

@group(3) @binding(0)
var fog: texture_3d<f32>;
@group(3) @binding(1)
var fog_sampler: sampler;


struct Time {
    time_since_startup: f32,
};
// @group(0) @binding(0)
// var texture: texture_storage_3d<rgba8unorm, read_write>;
@group(0) @binding(1)
var<uniform> time: Time;

// @fragment
// fn fragment(
//     @builtin(front_facing) is_front: bool,
//     @builtin(position) coord: vec4<f32>,
    // @location(0) world_position: vec4<f32>,
    // @location(1) world_normal: vec3<f32>,
    // #ifdef VERTEX_UVS
    //    @location(2) uv: vec2<f32>,
    // #endif
    // #ifdef VERTEX_TANGENTS
    //    @location(3) world_tangent: vec4<f32>,
    // #endif
    // #ifdef VERTEX_COLORS
    //    @location(4) color: vec4<f32>,
    // #endif
// ) -> @location(0) vec4<f32> {
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // 0,0.1,0.2, etc
    let tenths = fract(round(time.time_since_startup * 10.0) / 10.0);

    let coord = in.clip_position;
    // let pos = coord.xyz;
    let color = textureSample(fog, fog_sampler, vec3(coord.xy * 0.0002, tenths), vec3<i32>(0,0,0));
    return vec4(color.rgb, 1.0);
    // return vec4(coord.xyz, 1.0);
    // return vec4(in.uv, 0.0, 1.0);
    // let noise = simplexNoise2(vec2(0.0,time.time_since_startup * 200.2));
    // return vec4(smoothstep(0.0, 10.0, time.time_since_startup), 0.0, 0.0, 1.0);
    // return vec4(tenths, tenths, tenths, 1.0);
}

