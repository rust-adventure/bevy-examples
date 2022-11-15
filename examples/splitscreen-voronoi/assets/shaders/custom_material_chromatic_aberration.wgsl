#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var player_one_texture: texture_2d<f32>;
@group(1) @binding(1)
var player_one_sampler: sampler;
@group(1) @binding(2)
var player_two_texture: texture_2d<f32>;
@group(1) @binding(3)
var player_two_sampler: sampler;

@group(1) @binding(4)
var<uniform> player_one_pos: vec2<f32>;
@group(1) @binding(5)
var<uniform> player_two_pos: vec2<f32>;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);

//     // Sample each color channel with an arbitrary shift
//     var output_color = vec4<f32>(
//         textureSample(texture, our_sampler, uv + offset_r).r,
//         textureSample(texture, our_sampler, uv + offset_g).g,
//         textureSample(texture, our_sampler, uv + offset_b).b,
//         1.0
//     );

// let y = distance(vec2(0.5,0.5), uv);
    
// let x_grid = (40.0 % view.width);
//     return vec4(output_color.xyz * sin((y + offset_r.x) * 1200.0) * x_grid, 1.0);
let dis_one = distance(player_one_pos, uv - 0.5);
let dis_two = distance(player_two_pos, uv - 0.5);
let sample_one = textureSample(player_one_texture, player_one_sampler, uv);
let sample_two = textureSample(player_two_texture, player_two_sampler, uv);
if dis_one >= dis_two {
return    sample_one;
} else {
return     vec4(0.5,0.5,1.0,1.0) * sample_two;

}
}