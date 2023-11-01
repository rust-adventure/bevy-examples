#import bevy_pbr::forward_io::VertexOutput

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

    if world_normal.y == 1.0 {
        return vec4(0.92, 0.90, 0.73, 1.0);
    }

    let ratio = world_normal.z - world_normal.x;

    let x_color = vec4(0.57, 0.76, 0.74, 1.0);
    let z_color = vec4(0.29, 0.37, 0.57, 1.0);

    return mix(x_color, z_color, ratio);
}
