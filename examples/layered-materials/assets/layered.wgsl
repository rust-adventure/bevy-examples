
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_bindings,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_fragment::pbr_input_from_vertex_output,
    pbr_types::{PbrInput, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var base_color_texture: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_sampler: sampler;
// @group(#{MATERIAL_BIND_GROUP}) @binding(102) var metallic_roughness_texture: texture_2d_array<f32>;
// @group(#{MATERIAL_BIND_GROUP}) @binding(103) var metallic_roughness_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var normal_map_texture: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var normal_map_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var depth_map: texture_2d_array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var depth_map_sampler: sampler;

fn layered_material_default(in: VertexOutput, is_front: bool) -> PbrInput {
    var pbr_input = pbr_input_from_vertex_output(in, is_front, false);

    pbr_input.material.parallax_depth_scale = 0.0;
    pbr_input.material.flags = STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;

    return pbr_input;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
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
    
    var pbr_input = layered_material_default(in, is_front);
    pbr_input.material.base_color = textureSample(base_color_texture, base_color_sampler, in.uv, index);
    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    //out.color = vec4(1., 0., 0., 1.);

    return out;
}
