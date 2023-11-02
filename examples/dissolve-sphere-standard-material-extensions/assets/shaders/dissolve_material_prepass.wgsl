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
    pbr_prepass_functions::prepass_alpha_discard(in);

    var out: FragmentOutput;

#ifdef DEPTH_CLAMP_ORTHO
    out.frag_depth = in.clip_position_unclamped.z;
#endif // DEPTH_CLAMP_ORTHO

#ifdef NORMAL_PREPASS
    // NOTE: Unlit bit not set means == 0 is true, so the true case is if lit

    if (material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        let double_sided = (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;

        let world_normal = pbr_functions::prepare_world_normal(
            in.world_normal,
            double_sided,
            is_front,
        );

        let normal = pbr_functions::apply_normal_mapping(
            material.flags,
            world_normal,
            double_sided,
            is_front,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
            in.world_tangent,
#endif // STANDARDMATERIAL_NORMAL_MAP
#endif // VERTEX_TANGENTS
#ifdef VERTEX_UVS
            in.uv,
#endif // VERTEX_UVS
            view.mip_bias,
        );

        out.normal = vec4(normal * 0.5 + vec3(0.5), 1.0);
    } else {
        out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
    }


    
#endif // NORMAL_PREPASS

#ifdef MOTION_VECTOR_PREPASS
    out.motion_vector = pbr_prepass_functions::calculate_motion_vector(in.world_position, in.previous_world_position);
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



