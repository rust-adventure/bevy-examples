#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d
#import bevy_shader_utils::fresnel::fresnel
#import bevy_pbr::utils
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::lighting
#import bevy_pbr::{
    mesh_view_bindings::{
        globals,
        view
    },
}

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    #ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(5) @interpolate(flat) instance_index: u32,
#endif
    @location(6) position_diff: f32,
) -> @location(0) vec4<f32> {

    var noise = simplex_noise_3d(vec3<f32>(world_normal.xy * 400.2, globals.time));
    var alpha = (noise + 1.0) / 2.0;

    let highlight = smoothstep(0.0, 1.0, position_diff + 1.0);

    let fresnel = fresnel(view.world_position.xyz, world_position.xyz, world_normal, 2.0, 2.0);


    // let rate_x = dpdx(view_position.xyz);
    // let rate_y = dpdy(view_position.xyz);
    // let cross_rate: vec3<f32> = normalize(cross(rate_x, rate_y));
    let pink = vec3(0.941, 0.043, 0.843);
    let second = vec3(0.841, 0.0, 0.743);
    return vec4(mix(world_normal, vec3(0.991, 0.093, 0.893), highlight), smoothstep(0.0, 1.5, fresnel));

    // return vec4<f32>(position_diff, position_diff, position_diff, smoothstep(0.0, 3.0, world_normal.z));
    // return vec4<f32>(smoothstep(0.0, 1.0, result.x), smoothstep(0.0, 1.0, result.y), smoothstep(0.0, 1.0, result.z), 1.0);
}
