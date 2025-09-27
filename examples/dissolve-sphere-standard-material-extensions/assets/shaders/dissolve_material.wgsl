#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions,
    pbr_types::{
        STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
    },
    mesh_view_bindings::{view, previous_view_proj},
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{alpha_discard,apply_pbr_lighting, main_pass_post_lighting_processing},
}


#import bevy_pbr::mesh_view_bindings::globals
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    // pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    // alpha discard
    // pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);


    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // we can optionally modify the lit color before post-processing is applied
    out.color = out.color;

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);


   // we can optionally modify the final result here
    // custom code
    var base_color = out.color;
    var output_color: vec4f;
    var noise_step = 4.0;
    var noise = simplex_noise_3d(in.color.xyz * noise_step);
    var threshold = sin(globals.time);
    var alpha = step(noise, threshold);
    var edge_color = vec3<f32>(0.0, 1.0, 0.8);
    var border_step = smoothstep(threshold - 0.2, threshold + 0.2, noise);
    var dissolve_border = edge_color.xyz * border_step;

    output_color = vec4<f32>(
        base_color.xyz + dissolve_border.xyz,
        alpha
    );

    if output_color.a == 0.0 { discard; } else {
        return output_color;
    }
}
