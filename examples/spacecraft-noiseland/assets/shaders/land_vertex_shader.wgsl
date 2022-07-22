#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d

struct StandardMaterial {
    time: f32,
    // ship_position: vec3<f32>,
    // base_color: vec4<f32>;
    // emissive: vec4<f32>;
    // perceptual_roughness: f32;
    // metallic: f32;
    // reflectance: f32;
    // // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    // flags: u32;
    // alpha_cutoff: f32;
};

@group(1) @binding(0)
var<uniform> material: StandardMaterial;
@group(1) @binding(1)
var<uniform> ship_position: vec3<f32>;

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


    var noise = simplexNoise3(vertex.position);

    // out.color = vec4<f32>(vertex.position.x, noise, vertex.position.z, 1.0);
    out.color = vec4<f32>(0.5, noise, 0.5, 1.0);

    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position.x, noise, vertex.position.z, 1.0));
    // out.world_position.x = -100.0;
    // var offset = material.ship_position.z - out.world_position.z;
    var t = ship_position;
    out.world_position.y = out.world_position.y - 4.0 * sin(ship_position.z - out.world_position.z) / 100.;

    out.clip_position = mesh_position_world_to_clip(out.world_position);

    // var thing = directional_shadow_textures;

    return out;
}