#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d

struct StandardMaterial {
    time: f32;
    // base_color: vec4<f32>;
    // emissive: vec4<f32>;
    // perceptual_roughness: f32;
    // metallic: f32;
    // reflectance: f32;
    // // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    // flags: u32;
    // alpha_cutoff: f32;
};

[[group(1), binding(0)]]
var<uniform> material: StandardMaterial;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
#ifdef VERTEX_UVS
    [[location(2)]] uv: vec2<f32>;
#endif
#ifdef VERTEX_TANGENTS
    [[location(3)]] tangent: vec4<f32>;
#endif
#ifdef VERTEX_COLORS
    [[location(4)]] color: vec4<f32>;
#endif
#ifdef SKINNED
    [[location(5)]] joint_indices: vec4<u32>;
    [[location(6)]] joint_weights: vec4<f32>;
#endif
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
#ifdef VERTEX_UVS
    [[location(2)]] uv: vec2<f32>;
#endif
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
#ifdef VERTEX_COLORS
    [[location(4)]] color: vec4<f32>;
#endif
};

[[stage(vertex)]]
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
    // out.color = vec4<f32>(1.0, 1.0, 0.4, 1.0);

//     var base_color = vertex.color;
// //  var base_color = vec4<f32>(1.0,0.4,1.0,1.0);
//     var noise_step = .02;
//     // var base_color = vec3<f32>(0.533, 0.533, 0.80);

//     // var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step, in.frag_coord.y * noise_step, in.frag_coord.z * noise_step));
//     var noise = simplexNoise3(vec3<f32>(vertex.position.x * noise_step, vertex.position.y * noise_step, vertex.position.z * noise_step));
//     var threshold = sin(material.time);
//     var alpha = step(noise, threshold);

//     var edge_color = vec3<f32>(0.0, 1.0, 0.8);
//     var border_step = smoothStep(threshold - 0.2, threshold + 0.2, noise);
//     var dissolve_border = vec3<f32>(edge_color.x * border_step, edge_color.y * border_step, edge_color.z * border_step);

//     var output_color = vec4<f32>(base_color.x + dissolve_border.x, base_color.y + dissolve_border.y, base_color.z + dissolve_border.z, alpha);

// var test = (simplexNoise3(vec3<f32>(material.time, material.time, material.time)) + 1.0) / 2.0;
// return vec4<f32>(test, test,test,test);
    // if (output_color.a == 0.0) { discard; } else {
    out.color = vec4<f32>(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    // }

    return out;
}