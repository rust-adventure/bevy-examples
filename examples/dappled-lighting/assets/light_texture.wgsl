#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var layer_1_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var layer_1_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var layer_2_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var layer_2_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let sample_base = textureSample(base_texture, base_sampler, mesh.uv + globals.time / 50.);
    let sample_layer_1 = textureSample(layer_1_texture, layer_1_sampler, mesh.uv + globals.time / 20. + vec2(0.1, 0.2));
    let sample_layer_2 = textureSample(layer_2_texture, layer_2_sampler, mesh.uv - globals.time / 18. + vec2(0.2, 0.1));
    return  sample_base * sample_layer_1 * sample_layer_2;
    // if mesh.uv.x < 0.1 || mesh.uv.x > 0.9 || mesh.uv.y < 0.1 || mesh.uv.y > 0.9 {
    //     return vec4(0, 0, 0, 1.);
    // } else {
    //     return vec4(1, 0, 0, 1);
    // }
}