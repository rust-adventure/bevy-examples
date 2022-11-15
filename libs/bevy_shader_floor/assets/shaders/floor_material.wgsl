struct FloorMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: FloorMaterial;
@group(1) @binding(1)
var<uniform> repeat: vec4<f32>;
@group(1) @binding(2)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var base_color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    return textureSample(base_color_texture, base_color_sampler, vec2(uv.x * repeat.x, (1.0 - uv.y) * repeat.y));
}