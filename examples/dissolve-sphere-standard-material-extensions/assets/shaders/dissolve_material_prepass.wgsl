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


 
#ifdef PREPASS_FRAGMENT
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

    var output_color: vec4f;

    // var base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    var noise_step = 2.0;
    // var base_color = vec3<f32>(0.533, 0.533, 0.80);

    // var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step, in.frag_coord.y * noise_step, in.frag_coord.z * noise_step));
    // var noise = simplex_noise_3d(in.color.xyz * noise_step);
    // var noise = simplex_noise_3d(vec3(in.world_position.xyz / 1000. * noise_step));
    let c = in.color.xyz;
    var noise = simplex_noise_3d(c * noise_step);

    var threshold = sin(globals.time);
    // var threshold = sin(2023);
    var alpha = step(noise, threshold);

    // var edge_color = vec3<f32>(0.0, 1.0, 0.8);
    var edge_color = output_color * 3.0;
    var border_step = smoothstep(threshold - 0.2, threshold + 0.2, noise);
    var dissolve_border = edge_color.xyz * border_step;

    output_color = vec4<f32>(
        dissolve_border,
        alpha
    );
// var test = (simplexNoise3(vec3<f32>(material.time, material.time, material.time)) + 1.0) / 2.0;
// return vec4<f32>(test, test,test,test);

    if output_color.a == 0.00 { discard; } else {
        // return out;
    }

    var pbr_input = pbr_types::pbr_input_new();
    pbr_input.frag_coord = in.position;
    pbr_input.material = material;
    // pbr_input.material.base_color = output_color;
    pbr_input.material.base_color = vec4(1.0, 0.0, 0.0, 1.0);
    pbr_input.material.flags |= STANDARD_MATERIAL_FLAGS_UNLIT_BIT;
    pbr_input.material.flags = material.flags;
    out.deferred = pbr_deferred_functions::deferred_gbuffer_from_pbr_input(pbr_input);
    out.deferred_lighting_pass_id = 1u;


    return out;
}
#else
@fragment
fn fragment(in: VertexOutput) {
    pbr_prepass_functions::prepass_alpha_discard(in);
    var output_color: vec4f;

    // var base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    var noise_step = 2.0;
    // var base_color = vec3<f32>(0.533, 0.533, 0.80);

    // var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step, in.frag_coord.y * noise_step, in.frag_coord.z * noise_step));
    // var noise = simplex_noise_3d(in.color.xyz * noise_step);
    // var noise = simplex_noise_3d(vec3(in.world_position.xyz / 1000. * noise_step));
    let c = in.x.xyz;
    var noise = simplex_noise_3d(c * noise_step);

    var threshold = sin(globals.time);
    // var threshold = sin(2023);
    var alpha = step(noise, threshold);

    // var edge_color = vec3<f32>(0.0, 1.0, 0.8);
    var edge_color = output_color * 3.0;
    var border_step = smoothstep(threshold - 0.2, threshold + 0.2, noise);
    var dissolve_border = edge_color.xyz * border_step;

    output_color = vec4<f32>(
        dissolve_border,
        alpha
    );
// var test = (simplexNoise3(vec3<f32>(material.time, material.time, material.time)) + 1.0) / 2.0;
// return vec4<f32>(test, test,test,test);

    if output_color.a == 0.00 { discard; } else {
        // return out;
    }
}
#endif // PREPASS_FRAGMENT



