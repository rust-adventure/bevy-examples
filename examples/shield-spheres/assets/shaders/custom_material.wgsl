#import bevy_shader_utils::simplex_noise_3d
#import bevy_pbr::utils
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::lighting

struct CustomMaterial {
    // color: vec4<f32>,
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
    @location(5) position_diff: f32,
    @location(6) view_position: vec4<f32>,
    @location(7) view_inverse_position: vec4<f32>,
) -> @location(0) vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    // var input: vec3<f32> = vec3<f32>(uv.x * 40.0, uv.y * 40.0, 1.);
    var noise = simplexNoise3(vec3<f32>(world_normal.xy * 400.2, material.time));
    var alpha = (noise + 1.0) / 2.0;
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv) * vec4<f32>(1.0, 1.0, 1.0, alpha);
    // return material.color * vec4<f32>(1.0, 1.0, 1.0, alpha);
    // let result: vec3<f32> = fresnel(world_normal.xyz, 1.0);

    // let whatever = fresnel(in.world_normal);
    let highlight = smoothstep(0.0, 1.0, position_diff + 1.0);

/// fresnel 2.0I
    var N = normalize(world_normal);
    var V = normalize(view.world_position.xyz - world_position.xyz);
    let NdotV = max(dot(N, V), 0.0001);
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);

    fresnel = pow(fresnel, 3.0) * 2.0;
    
/// fresnel 2.0I


    // let rate_x = dpdx(view_position.xyz);
    // let rate_y = dpdy(view_position.xyz);
    // let cross_rate: vec3<f32> = normalize(cross(rate_x, rate_y));

    // let bias = 0.31;
    // let scale = 0.2;
    // let power = 2.30;
    // let I = normalize(-view_inverse_position.xyz);
    // let N = normalize(world_normal.xyz);
    // let result = max(0.0, min(1.0, bias + scale * pow((1.0 + dot(I, N)), power)));

    // return vec4(cross_rate * position_diff, noise);
    // return vec4(0.941, 0.043, 0.843, noise * (1.0 - result));
    
    // return vec4(0.941, 0.043, 0.843, (1.0 - result));

    // return vec4(0.941, 0.043, 0.843, 1.0 - result);

    return vec4(vec3(0.941, 0.043, 0.843) * fresnel, 1.0 - fresnel);
    // smoothstep(0.0, 2.0, 1.0 - fresnel));

    // return vec4<f32>(position_diff, position_diff, position_diff, smoothstep(0.0, 3.0, world_normal.z));
    // return vec4<f32>(smoothstep(0.0, 1.0, result.x), smoothstep(0.0, 1.0, result.y), smoothstep(0.0, 1.0, result.z), 1.0);
}
