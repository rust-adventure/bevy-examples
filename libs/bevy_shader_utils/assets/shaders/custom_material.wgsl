#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

#import bevy_shader_utils::voronoise::voronoise

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {

    var p: vec2<f32> = 0.5 - 0.5 * cos(globals.time + vec2(1.0, 0.5));
    p.y = 0.0;
    // var p = vec2(0.0,0.0);
    // var p = vec2(1.0,0.0);
    // var p = vec2(0.0,1.0);
    // var p = vec2(1.0,1.0);

    let f: f32 = voronoise(50.0 * (mesh.uv + globals.time / 10.0), p.x, p.y);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}

