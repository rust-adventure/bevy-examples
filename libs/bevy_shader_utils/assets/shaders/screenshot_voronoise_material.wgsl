#import bevy_pbr::mesh_vertex_output MeshVertexOutput

#import bevy_shader_utils::voronoise voronoise

struct Material {
    x: f32,
    y: f32,
    scale: f32
};

@group(1) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(
    mesh: MeshVertexOutput
) -> @location(0) vec4<f32> {
    let f: f32 = voronoise(material.scale * mesh.uv, material.x, material.y);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

