#import bevy_shader_utils::simplex_noise_3d

struct CustomMaterial {
    // color: vec4<f32>,
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
) -> @location(0) vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    // var input: vec3<f32> = vec3<f32>(uv.x * 40.0, uv.y * 40.0, 1.);
    var noise = simplexNoise3(vec3<f32>(world_normal.xyz * 2.0));
    var alpha = (noise + 1.0) / 2.0;
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv) * vec4<f32>(1.0, 1.0, 1.0, alpha);
    // return material.color * vec4<f32>(1.0, 1.0, 1.0, alpha);
    // return vec4<f32>(uv.x, uv.y, 0.0, 1.0);

    return vec4<f32>(world_normal.xyz, smoothstep(-1.0, 3.0, noise));
}
