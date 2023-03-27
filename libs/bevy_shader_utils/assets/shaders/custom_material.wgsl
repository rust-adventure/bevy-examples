#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
// #import bevy_pbr::mesh_functions
#import bevy_shader_utils::voronoise

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;


struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(
    in: FragmentInput,
) -> @location(0) vec4<f32> {

    var p: vec2<f32> = 0.5 - 0.5*cos( globals.time + vec2(1.0,0.5) );
    p.y = 0.0;
    // var p = vec2(0.0,0.0);
    // var p = vec2(1.0,0.0);
    // var p = vec2(0.0,1.0);
    // var p = vec2(1.0,1.0);

    let f: f32 = voronoise(50.0 * (in.uv + globals.time / 10.0), p.x, p.y);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

