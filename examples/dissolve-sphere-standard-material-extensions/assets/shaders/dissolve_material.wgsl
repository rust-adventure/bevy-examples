#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    pbr_functions,
        pbr_types::{
        STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
    },
        mesh_view_bindings::{view, previous_view_proj},
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

#import bevy_pbr::mesh_view_bindings::globals
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d

// struct DissolveMaterial {
//     // quantize_steps: u32,
// }

// @group(1) @binding(100)
// var<uniform> my_extended_material: DissolveMaterial;


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
      // we can optionally modify the final result here
      // custom code
    var base_color = out.color;
    var output_color: vec4f;
    
//  var base_color = vec4<f32>(1.0,0.4,1.0,1.0);
    var noise_step = 2.0;
    // var base_color = vec3<f32>(0.533, 0.533, 0.80);

    // var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step, in.frag_coord.y * noise_step, in.frag_coord.z * noise_step));
    // var noise = simplex_noise_3d(in.world_position.xyz / 1000. * noise_step);
    // var noise = simplex_noise_3d(vec3(in.uv.xy * noise_step, 1.0));
    // var noise = simplex_noise_3d(in.color.xyz * noise_step);
    var noise = simplex_noise_3d(in.color.xyz * noise_step);


    // var noise = simplex_noise_3d(in.color.xyz * noise_step);
    var threshold = sin(globals.time);
    var alpha = step(noise, threshold);

    // var edge_color = vec3<f32>(0.0, 1.0, 0.8);
    var edge_color = output_color * 3.0;
    var border_step = smoothstep(threshold - 0.2, threshold + 0.2, noise);
    var dissolve_border = edge_color.xyz * border_step;

    output_color = vec4<f32>(
        base_color.xyz + dissolve_border.xyz,
        alpha
    );
// var test = (simplexNoise3(vec3<f32>(material.time, material.time, material.time)) + 1.0) / 2.0;
// return vec4<f32>(test, test,test,test);
    // if output_color.a == 0.0 { discard; } else {
    //     return vec4(1.0, 1.0, 0., 1.);
    //     // return output_color;
    // }
    return output_color;
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // we can optionally modify the lit color before post-processing is applied
    out.color = out.color;

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);



    // return out.color;
#endif
    return vec4(1.0, 0.0, 0.0, 1.0);
}
