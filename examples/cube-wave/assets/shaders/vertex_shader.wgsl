#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_shader_utils::mock_fresnel

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

#import bevy_shader_utils::simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d
#import bevy_pbr::utils

struct CustomMaterial {
    // time: f32,
    offset: f32,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

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
    @location(5) position_diff: f32,
    @location(6) view_position: vec4<f32>,
    @location(7) view_inverse_position: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    // let thickness = 5.0;
    // // higher is shorter
    // let how_long_to_stay_in_opposite_state = 30.0;
    // let frequency = 2.0;
    // // let position_diff = pow(sin(2.0 * material.time), 1.0);
    // let position_diff = 1.0 - pow(thickness * sin(frequency * material.time + vertex.position.x + vertex.position.y), how_long_to_stay_in_opposite_state);
    // // let smooth_diff = smoothstep(0.0, 1.0, position_diff);
    // let position = (vertex.normal * (smoothstep(0.0, 1.0, position_diff)) * 0.02) + vertex.position;

    // let position = vertex.position * vec3(1.0, abs(sin(material.time / 100.0 + material.offset / 100.0)), 1.0);
    // let position = vertex.position;

    var out: VertexOutput;
    out.position_diff = 0.0;


#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    out.world_normal = skin_normals(model, vertex.normal);
#else
    var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif

    // let new_world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    let y = abs(sin(globals.time + material.offset / 3.0));

    var position = vec3(vertex.position.x, vertex.position.y + y + 0.4, vertex.position.z);

    if vertex.position
        .
        y > 0.0 {
            position = vec3(vertex.position.x, vertex.position.y + y + 0.4, vertex.position.z);
        } else {
            position = vec3(vertex.position.x, vertex.position.y - y - 0.4, vertex.position.z);
        }


        out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));

    // out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
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

        out.color = out.clip_position;


        out.view_inverse_position = view.inverse_view * out.world_position;
        out.view_position = view.view * out.world_position;

        // let position = vertex.position * vec3(1.0, abs(sin(globals.time + vertex.position.x + vertex.position.z)), 1.0);

    // out.view_inverse_position = view.inverse_view_proj * out.world_position;
        return out;
}