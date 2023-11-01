#import bevy_pbr::forward_io::VertexOutput
#import bevy_shader_utils::perlin_noise_3d::perlin_noise_3d

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    var input: vec3<f32> = vec3<f32>(in.uv.x * 40.0, in.uv.y * 40.0, 1.);
    var noise = perlin_noise_3d(input);
    var alpha = (noise + 1.0) / 2.0;
    return material.color * textureSample(base_color_texture, base_color_sampler, in.uv) * vec4<f32>(1.0, 1.0, 1.0, alpha);
}
