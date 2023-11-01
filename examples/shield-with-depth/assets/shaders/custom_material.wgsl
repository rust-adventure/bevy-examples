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
#import bevy_render::{
    instance_index::get_instance_index
}

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
fn vertex(vertex: Vertex) -> VertexOutput {
    let thickness = 5.0;
    // higher is shorter
    let how_long_to_stay_in_opposite_state = 30.0;
    let frequency = 2.0;
    // let position_diff = pow(sin(2.0 * material.time), 1.0);
    let position_diff = 1.0 - pow(thickness * sin(frequency * globals.time + vertex.position.y + vertex.position.z), how_long_to_stay_in_opposite_state);
    // let smooth_diff = smoothstep(0.0, 1.0, position_diff);
    let position = (vertex.normal * (smoothstep(0.0, 1.0, position_diff)) * 0.04) + vertex.position;

    var out: VertexOutput;
    out.position_diff = position_diff;

    var model = mesh_functions::get_model_matrix(vertex.instance_index);


    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, get_instance_index(vertex.instance_index));


#ifdef VERTEX_POSITIONS
    // out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif


    return out;
}

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: VertexOutput,
    // @location(6) position_diff: f32,
) -> @location(0) vec4<f32> {
    // return color;
    var noise = simplex_noise_3d(vec3<f32>(mesh.world_normal.xy * 4.2, globals.time));
    var alpha = (noise + 1.0) / 2.0;

    let highlight = smoothstep(0.0, 1.0, mesh.position_diff + 1.0);

    let fresnel = fresnel(view.world_position.xyz, mesh.world_position.xyz, mesh.world_normal, 2.0, 1.0);

    let offset = 0.82;
    let intersection_intensity = 10.0;
    let sample_index = 0u;

    let depth = prepass_depth(mesh.position, sample_index);

    // thanks to https://github.com/IceSentry for this line in particular,
    // which I was having trouble landing on
    var intersection = 1.0 - ((mesh.position.z - depth) * 100.0) - offset;
    intersection = smoothstep(0.0, 1.0, intersection);
    if is_front {
        intersection *= intersection_intensity;
    } else {
        intersection *= intersection_intensity / 2.0;
    }

    let color = mix(vec3(1.00, 0.455, 0.827), vec3(1.00, 0.555, 0.927), highlight) * (alpha + 0.5) * 5.0;
    if is_front {
        return vec4(color * (10.0 * highlight + 1.0), fresnel * 0.4 + intersection + highlight * 0.003);
    } else {
        return vec4(color, intersection);
    }
}
