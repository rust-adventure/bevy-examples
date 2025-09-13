#import bevy_pbr::forward_io::VertexOutput

#import bevy_shader_utils::perlin_noise_3d::perlin_noise_3d

struct Material {
    scale: f32
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: Material;


@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let f: f32 = perlin_noise_3d(material.scale * mesh.world_position.xyz);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

