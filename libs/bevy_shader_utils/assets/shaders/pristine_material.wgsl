#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

#import bevy_shader_utils::pristine_grid::pristine_grid

struct PristineMaterial {
    color: vec4f
}

@group(1) @binding(0)
var<uniform> material: PristineMaterial;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let color = pristine_grid(80. * mesh.uv, vec2(0.1)) * material.color;
    return color;
}


