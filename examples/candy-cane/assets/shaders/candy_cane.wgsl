#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
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

struct Stripe {
    frequency: f32,
    minimum_value: f32,
    power_value: f32,
    should_use: f32,
    color: vec4f,
}

#import bevy_shader_utils::simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d

@group(2) @binding(100)
var<storage, read> stripes: array<Stripe>;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // start our color code
    var stripe_output: vec4<f32> = pbr_input.material.base_color;
    for (var i = 0; i < i32(arrayLength(&stripes)); i++) {
        let stripe_mix = make_stripe(
            in.uv,
            stripes[i].frequency,
            stripes[i].minimum_value,
            stripes[i].power_value,
            stripes[i].should_use
        );
        stripe_output = mix(stripe_output, stripes[i].color, stripe_mix);
    }

    // pbr_input.material.base_color = vec4<f32>(0.533, 0.533, 0.80, 1.0);

    pbr_input.material.base_color = stripe_output;
    // end our color code

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // we can optionally modify the lit color before post-processing is applied
    // out.color = vec4<f32>(vec4<u32>(out.color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    // out.color = out.color * 2.0;
#endif

    return out;
}

fn make_stripe(uv: vec2<f32>, frequency: f32, minimum_value: f32, power_value: f32, offset: f32) -> f32 {
    // custom constant
    let PI: f32 = 3.14159265358979323846264338327950288;

    /// use the uv u (or x) coordinate of the capsule
    /// to define a stripe, then multiply by a number 
    /// which will increase the frequency of the sin wave,
    /// increasing the number of stripes
    let mix_value = abs(sin(((frequency * uv.y) + uv.x) * PI + offset));


    /// control the fade between stripe color and base color
    ///
    /// use a minimum value and a saturate function to raise
    /// the mix_value by the minimum, then clamp the value to 1.0,
    /// which will result in a range 0.4..1.0. This controls the
    /// stripe color "thickness"
    ///
    /// larger minimum value is thicker stripe
    ///
    /// larger power value is thicker base color
    return pow(saturate(mix_value + minimum_value), power_value);
}

