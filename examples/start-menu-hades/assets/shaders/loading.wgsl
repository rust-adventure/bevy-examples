#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) color: vec4<f32>,
    @location(1) progress: f32
}

@group(1) @binding(0)
var<uniform> input: CustomUiMaterial;

@group(1) @binding(2) var loading_texture: texture_2d<f32>;
@group(1) @binding(3) var loading_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {

    let width = in.size.x;
    let height = in.size.y;

    let pos_x = in.uv.x * width;
    let pos_y = in.uv.y * height;

    let ratio = width / height;

    // let coord = vec2(pos_x % 100., pos_y % 100.);
    let coord = in.uv * vec2(10., 1.);
    // return vec4(coord.xy, 0.,1.);

    let sampled_color = textureSample(loading_texture, loading_sampler, coord);
    let progress_color = clamp(step(in.uv.x, input.progress) * sampled_color.rgb, vec3(0.5), vec3(1.)) * input.color.rgb;

    return vec4(progress_color.rgb, input.color.a);
}