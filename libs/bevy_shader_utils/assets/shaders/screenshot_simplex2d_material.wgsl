#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_shader_utils::simplex_noise_2d::simplex_noise_2d

struct Material {
    scale: f32
};

@group(2) @binding(0)
var<uniform> material: Material;


@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let f: f32 = simplex_noise_2d(material.scale * mesh.uv);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

