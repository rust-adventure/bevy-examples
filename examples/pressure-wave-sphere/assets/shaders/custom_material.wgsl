#import bevy_shader_utils::simplex_noise_3d

@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
) -> @location(0) vec4<f32> {

    var noise = simplexNoise3(vec3<f32>(world_normal.xyz * 2.0));
    var alpha = (noise + 1.0) / 2.0;

    return vec4<f32>(world_normal.xyz, smoothstep(-1.0, 3.0, noise));
}
