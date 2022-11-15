#import bevy_pbr::mesh_view_bindings
#import bevy_shader_utils::simplex_noise_2d
#import bevy_shader_utils::simplex_noise_3d

struct StandardMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: StandardMaterial;

@group(1) @binding(1)
var texture: texture_2d<f32>;

@group(1) @binding(2)
var our_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.viewport.z, view.viewport.w);
    // var offset_strength = 0.02;

let offset_strength = simplexNoise2(vec2(material.time * 5.0, material.time / 0.02))  * 0.02;
    // if noise > 0.5 {
    //     offset_strength = 0.0;
    // };
    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, offset_strength)).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );
    
    return output_color;
}