#import bevy_pbr::{
    prepass_bindings,
    mesh_functions,
    prepass_io::{Vertex, VertexOutput, FragmentOutput},
    skinning,
    morph,
    mesh_view_bindings::{view, previous_view_proj},
    rgb9e5::vec3_to_rgb9e5_,
    pbr_prepass_functions::prepass_alpha_discard,
    pbr_prepass_functions,
    pbr_bindings::material,
    pbr_deferred_functions,
    pbr_types::{
        STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
    },
    pbr_functions,
    prepass_io,
    pbr_types
}
#import bevy_render::instance_index::get_instance_index
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;
 
// basically all of this prepass shader is just copy/pasted from
// the bevy pbr prepass shader.
@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var out: FragmentOutput;

#ifdef NORMAL_PREPASS
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
#endif

#ifdef DEPTH_CLAMP_ORTHO
    out.frag_depth = in.clip_position_unclamped.z;
#endif // DEPTH_CLAMP_ORTHO

#ifdef MOTION_VECTOR_PREPASS
    let clip_position_t = view.unjittered_clip_from_world * in.world_position;
    let clip_position = clip_position_t.xy / clip_position_t.w;
    let previous_clip_position_t = prepass_bindings::previous_view_uniforms.clip_from_world * in.previous_world_position;
    let previous_clip_position = previous_clip_position_t.xy / previous_clip_position_t.w;
    // These motion vectors are used as offsets to UV positions and are stored
    // in the range -1,1 to allow offsetting from the one corner to the
    // diagonally-opposite corner in UV coordinates, in either direction.
    // A difference between diagonally-opposite corners of clip space is in the
    // range -2,2, so this needs to be scaled by 0.5. And the V direction goes
    // down where clip space y goes up, so y needs to be flipped.
    out.motion_vector = (clip_position - previous_clip_position) * vec2(0.5, -0.5);
#endif // MOTION_VECTOR_PREPASS

#ifdef DEFERRED_PREPASS
    // There isn't any material info available for this default prepass shader so we are just writing 
    // emissive magenta out to the deferred gbuffer to be rendered by the first deferred lighting pass layer.
    // This is here so if the default prepass fragment is used for deferred magenta will be rendered, and also
    // as an example to show that a user could write to the deferred gbuffer if they were to start from this shader.
    out.deferred = vec4(0u, bevy_pbr::rgb9e5::vec3_to_rgb9e5_(vec3(1.0, 0.0, 1.0)), 0u, 0u);
    out.deferred_lighting_pass_id = 1u;
#endif

    

    // same calcuation for gaps as the material, but without color
    // output. 
    var noise_step = 4.0;
    let c = in.color.xyz;
    var noise = simplex_noise_3d(c * noise_step);
    var threshold = sin(globals.time);
    var alpha = step(noise, threshold);
    if alpha == 0.00 { discard; };

    return out;
}



