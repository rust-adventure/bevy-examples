#import bevy_shader_utils::perlin_noise_2d

struct CustomMaterial {
    time: f32;
};

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

// [[stage(fragment)]]
// fn fragment([[location(2)]] uv: vec2<f32>) -> [[location(0)]] vec4<f32> {
//     // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
//     var input: vec3<f32> = vec3<f32>(uv.x * 40.0, uv.y * 40.0, 1.);
//     var noise = perlinNoise3(input);
//     var alpha = (noise + 1.0) / 2.0;
//     return vec4<f32>(1.0,1.0,1.0,alpha);
// }

[[stage(fragment)]]
fn fragment([[builtin(position)]] coord: vec4<f32>, [[location(2)]] uv: vec2<f32>) -> [[location(0)]] vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    var input1: vec2<f32> = vec2<f32>(coord.x  / 50.0, material.time);
    var input2: vec2<f32> = vec2<f32>(coord.y  / 50.0, material.time);
    var input3: vec2<f32> = vec2<f32>(coord.z  / 50.0, material.time);

    var noise1 = perlinNoise2(input1);
    var noise2 = perlinNoise2(input2);
    var noise3 = perlinNoise2(input3);

    var value1 = (noise1 + 1.0) / 2.0;
    var value2 = (noise2 + 1.0) / 2.0;
    var value3 = (noise3 + 1.0) / 2.0;

    var input_u1: vec2<f32> = vec2<f32>(uv.x  / 50.0, material.time);
    var input_u2: vec2<f32> = vec2<f32>(uv.y  / 50.0, material.time);

    var noise_u1 = perlinNoise2(input_u1);
    var noise_u2 = perlinNoise2(input_u2);

    var value_u1 = (noise_u1 + 1.0) / 2.0;
    var value_u2 = (noise_u2 + 1.0) / 2.0;


    // return vec4<f32>(value1,value2,value3,1.0);
    return vec4<f32>(uv.x,uv.y,value3,1.0);
}
//    @builtin(position) coord: vec4<f32>,
