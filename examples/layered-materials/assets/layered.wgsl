#import bevy_pbr::{
    // pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    pbr_bindings,
}
#import layered_materials::pbr_fragment::pbr_input_from_standard_material

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

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var base_color_texture: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var base_color_sampler: sampler;
// @group(#{MATERIAL_BIND_GROUP}) @binding(102) var metallic_roughness_texture: texture_2d_array<f32>;
// @group(#{MATERIAL_BIND_GROUP}) @binding(103) var metallic_roughness_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(104) var normal_map_texture: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(105) var normal_map_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(106) var depth_map: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(107) var depth_map_sampler: sampler;

// struct MyExtendedMaterial {
//     quantize_steps: u32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     // Web examples WebGL2 support: structs must be 16 byte aligned.
//     _webgl2_padding_8b: u32,
//     _webgl2_padding_12b: u32,
//     _webgl2_padding_16b: u32,
// #endif
// }

// @group(#{MATERIAL_BIND_GROUP}) @binding(100)
// var<uniform> my_extended_material: MyExtendedMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // let layer_0 = textureSample(
    //     depth_map,
    //     depth_map_sampler,
    //     in.uv,
    //     0,
    // );
    // let layer_1 = textureSample(
    //     depth_map,
    //     depth_map_sampler,
    //     in.uv,
    //     1,
    // );
    // let layer_2 = textureSample(
    //     depth_map,
    //     depth_map_sampler,
    //     in.uv,
    //     2,
    // );

    // let index = i32(floor(in.uv.x * 2.999));

    // let uv_transform = pbr_bindings::material.uv_transform;
    // var uv = (uv_transform * vec3(in.uv, 1.0)).xy;


    var index = 0;
    if in.uv.x < 0.25 {
        index = 0;
    } else if in.uv.x > 0.75 {
        index = 1;
    } else {
        let layer_0 = textureSample(
            depth_map,
            depth_map_sampler,
            in.uv,
            0,
        );
        let layer_1 = textureSample(
            depth_map,
            depth_map_sampler,
            in.uv,
            1,
        );
        if layer_0.x > layer_1.x && in.uv.x < layer_0.x {
            index = 0;
        } else {
            index = 1;
        }
    }
    // generate a PbrInput struct from the StandardMaterial bindings
    // let index = i32(floor(in.uv.x * 2.999));

    // let index = i32(0);
    var pbr_input = pbr_input_from_standard_material(
        in,
        is_front,
        index,
        base_color_texture,
        base_color_sampler,
        normal_map_texture,
        normal_map_sampler,
        depth_map,
        depth_map_sampler,
    );

    // pbr_input.material.base_color = textureSample(base_color_texture, base_color_sampler, in.uv, index);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    // out.color = vec4(1., 0., 0., 1.);

    return out;
}
