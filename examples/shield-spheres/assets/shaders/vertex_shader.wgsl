#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d
#import bevy_pbr::{
    mesh_view_bindings::{
        globals,
        view
    },
    mesh_bindings::mesh,
    mesh_functions as mesh_functions,
    forward_io::Vertex,
    view_transformations::position_world_to_clip,
}
#import bevy_shader_utils::fresnel::fresnel
#import bevy_pbr::prepass_utils::prepass_depth

// @group(0) @binding(0) var<uniform> view: View;

// mostly a clone of bevy_pbr::forward_io::VertexOutput
// so that we can add the extra fields
struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
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
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(5) @interpolate(flat) instance_index: u32,
#endif
    @location(6) position_diff: f32,
};

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    let thickness = 5.0;
    // higher is shorter
    let how_long_to_stay_in_opposite_state = 30.0;
    let frequency = 2.0;
    // let position_diff = pow(sin(2.0 * material.time), 1.0);
    let position_diff = 1.0 - pow(thickness * sin(frequency * globals.time + vertex_no_morph.position.y + vertex_no_morph.position.z), how_long_to_stay_in_opposite_state);
    // let smooth_diff = smoothstep(0.0, 1.0, position_diff);
    let position = (vertex_no_morph.normal * (smoothstep(0.0, 1.0, position_diff)) * 0.04) + vertex_no_morph.position;

    var out: VertexOutput;
    out.position_diff = position_diff;

    var vertex = vertex_no_morph;

    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);

    #ifdef VERTEX_NORMALS
        out.world_normal = mesh_functions::mesh_normal_local_to_world(
            vertex.normal,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif

    #ifdef VERTEX_POSITIONS
        out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(position, 1.0));
        out.position = position_world_to_clip(out.world_position.xyz);
    #endif

    out.uv = vertex.uv;

    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
            world_from_local,
            vertex.tangent,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif

    #ifdef VERTEX_OUTPUT_INSTANCE_INDEX
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        out.instance_index = vertex_no_morph.instance_index;
    #endif

    return out;
}