#import bevy_ui::ui_vertex_output::UiVertexOutput

// struct CustomUiMaterial {
//     @location(0) color: vec4<f32>,
//     @location(1) progress: f32
// }

// @group(1) @binding(0)
// var<uniform> input: CustomUiMaterial;

@group(1) @binding(2) var backdrop_texture: texture_2d<f32>;
@group(1) @binding(3) var backdrop_sampler: sampler;

@group(1) @binding(4) var loading_texture: texture_2d<f32>;
@group(1) @binding(5) var loading_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let coord = in.uv;

    let sampled_color_backdrop = textureSample(backdrop_texture, backdrop_sampler, coord);
    let sampled_color = textureSample(loading_texture, loading_sampler, coord) * 5 * vec4(0.,1,0,1);

    // return mix(vec4(1.,1.,1.,1.), sampled_color_backdrop,  (sampled_color.r + sampled_color.g + sampled_color.b )/ 3);
    // return vec4(vec3((sampled_color_backdrop.r + sampled_color_backdrop.g + sampled_color_backdrop.b )/ 3), 1.0);
    let color = 1. - (((1. - sampled_color) * (1. - sampled_color_backdrop))/1.);
    return vec4(color.rgb, 1.);
    // return sampled_color_backdrop * sampled_color;
    // return vec4(sampled_color.rgb, 1.);
    // return vec4(sampled_color.rgb, sampled_color.r);
}