#import bevy_pbr::utils
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::lighting

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
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif

) -> @location(0) vec4<f32> {

    if world_normal.y == 1.0 {
        return vec4(0.92, 0.90, 0.73, 1.0);
    }

    let ratio = world_normal.z - world_normal.x;

    let x_color = vec4(0.57, 0.76, 0.74, 1.0);
    let z_color = vec4(0.29, 0.37, 0.57, 1.0);

    return mix(x_color, z_color, ratio);
}
