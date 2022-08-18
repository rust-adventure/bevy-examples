#import bevy_shader_utils::simplex_noise_3d


@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;


struct Time {
    time_since_startup: f32,
};
@group(0) @binding(1)
var<uniform> time: Time;


@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let location_for_noise = vec3<f32>(f32(invocation_id.x) * 0.052, f32(invocation_id.y) * 0.052, time.time_since_startup * 0.002);
    let noise = simplexNoise3(location_for_noise);
    let color = vec4<f32>(f32(noise));

    textureStore(texture, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let location_for_noise = vec3<f32>(f32(invocation_id.x) * 0.02, f32(invocation_id.y) * 0.02, time.time_since_startup * 1.0);
    let noise = simplexNoise3(location_for_noise);

    let color = vec4<f32>(noise, noise * noise, noise * 0.2, 1.0);

    storageBarrier();

    textureStore(texture, location, color);
}