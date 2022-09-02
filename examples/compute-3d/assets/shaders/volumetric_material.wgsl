#import bevy_shader_utils::simplex_noise_2d

#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
// };

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) uv: vec2<f32>,
//     @location(1) clipclip: vec4<f32>,
// };

// @vertex
// fn vertex(vertex: Vertex) -> VertexOutput {
//     var out: VertexOutput;
//     out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
//     out.uv = vertex.uv;
//     out.clipclip = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));

//     return out;
// }

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
    @location(5) vertex_position: vec3<f32>,
    @location(6) view_position: vec4<f32>,
    @location(7) cheating_lower_bound: vec4<f32>,
    @location(8) cheating_upper_bound: vec4<f32>,
};
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.vertex_position = vertex.position;
    out.view_position = view.inverse_view * out.world_position;

    out.cheating_lower_bound = view.inverse_view * vec4(-0.5, -0.5, -0.5, 1.0);
    out.cheating_upper_bound = view.inverse_view * vec4(0.5, 0.5, 0.5, 1.0);

    return out;
}

struct Time {
    time_since_startup: f32,
};
@group(2) @binding(0)
var<uniform> time: Time;

@group(3) @binding(0)
var fog: texture_3d<f32>;
@group(3) @binding(1)
var fog_sampler: sampler;


fn ray_box_dst(
    boundsMin: vec3<f32>, 
    boundsMax: vec3<f32>, 
    rayOrigin: vec3<f32>, 
    invRaydir: vec3<f32>
    ) -> vec2<f32>{
    // Adapted from https://github.com/SebLague/Clouds/blob/fcc997c40d36c7bedf95a294cd2136b8c5127009/Assets/Scripts/Clouds/Shaders/Clouds.shader#L121
    // Adapted from: http://jcgt.org/published/0007/03/04/
    let t0: vec3<f32> = (boundsMin - rayOrigin) * invRaydir;
    let t1: vec3<f32> = (boundsMax - rayOrigin) * invRaydir;
    let tmin: vec3<f32> = min(t0, t1);
    let tmax: vec3<f32> = max(t0, t1);
    
    let dstA: f32 = max(max(tmin.x, tmin.y), tmin.z);
    let dstB: f32 = min(tmax.x, min(tmax.y, tmax.z));

    // CASE 1: ray intersects box from outside (0 <= dstA <= dstB)
    // dstA is dst to nearest intersection, dstB dst to far intersection

    // CASE 2: ray intersects box from inside (dstA < 0 < dstB)
    // dstA is the dst to intersection behind the ray, dstB is dst to forward intersection

    // CASE 3: ray misses box (dstA > dstB)

    let dstToBox: f32 = max(0.0, dstA);
    let dstInsideBox: f32 = max(0.0, dstB - dstToBox);
    return vec2<f32>(dstToBox, dstInsideBox);
}

let scale  = 1.0;
let offset = 0.0;
let density_threshold = 0.2;
let density_multiplier = 1.0;
fn sample_density(position: vec3<f32>) -> f32 {
    let uvw: vec3<f32> = position * scale * 0.001 + offset * 0.01;
    let shape: vec4<f32> = textureSample(
        fog,
        fog_sampler,
        vec3(uvw.xy + 0.5, uvw.z + 0.5),
        vec3<i32>(0,0,0)
    );
    let density = max(0.0, shape.r - density_threshold) * density_multiplier;
    return density;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // 0,0.1,0.2, etc
    // let tenths = fract(round(time.time_since_startup * 100.0) / 100.0);
    let ray_dir = normalize(in.view_position);
    let ray_box_info = ray_box_dst(
        in.cheating_lower_bound.xyz,
        in.cheating_upper_bound.xyz,
        vec3(0.0,0.0,0.0),
        ray_dir.xyz,
    );

    let dst_to_box = ray_box_info.x;
    let dst_inside_box = ray_box_info.y;

    let ray_hit_box: bool = dst_inside_box > 0.0;
    
    let color = textureSample(
        fog,
        fog_sampler,
        vec3(in.vertex_position.xy + 0.5, (in.vertex_position.z + 0.5)),
        vec3<i32>(0,0,0)
    );

    // if ray_hit_box {
        return vec4(color.rgb, 1.0);
    // } else {
    //     return vec4(1.0,1.0,1.0,1.0);
    // }

    // var dst_travelled = 0.0;
    // let num_steps = 10.0;
    // let step_size = dst_inside_box / num_steps;
//     // let dst_limit = min(dst_to_box, dst_inside_box);
// return vec4(step_size,step_size,step_size,1.0);
// return in.view_position;
    // var total_density: f32 = 0.0;
    // while (dst_travelled < dst_limit) {
    //     let ray_pos = vec3(0.0,0.0,0.0) + ray_dir.xyz * (dst_to_box + dst_travelled);
    //     total_density += sample_density(ray_pos) * step_size;
    //     dst_travelled += step_size;
    // }

    // let transmittance = exp(-total_density);

    // return vec4(total_density,total_density,total_density,1.0) * transmittance;
    // return vec4(transmittance,transmittance,transmittance,1.0) * transmittance;

    // return vec4(1.0,1.0,1.0,1.0) * transmittance;
   
}
