#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

struct CustomMaterial {
    color: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: CustomMaterial;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}