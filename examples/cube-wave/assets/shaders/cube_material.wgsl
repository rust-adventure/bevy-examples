#import bevy_pbr::{
    mesh_bindings::mesh,
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings::globals

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var vertex = vertex_no_morph;

    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);

    #ifdef VERTEX_NORMALS
        out.world_normal = mesh_functions::mesh_normal_local_to_world(
            vertex.normal,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif

    #ifdef VERTEX_POSITIONS
        let y = abs(sin(globals.time + material.offset / 3.0));

        var position: vec3<f32>;
        if vertex.position.y > 0.0 {
            position = vec3(vertex.position.x, vertex.position.y + y + 0.4, vertex.position.z);
        } else {
            position = vec3(vertex.position.x, vertex.position.y - y - 0.4, vertex.position.z);
        }
        out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(position, 1.0));
        out.position = position_world_to_clip(out.world_position.xyz);
    #endif

    out.uv = vertex.uv;

    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
            world_from_local,
            vertex.tangent,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif

    #ifdef VERTEX_OUTPUT_INSTANCE_INDEX
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        out.instance_index = vertex_no_morph.instance_index;
    #endif

    return out;
}

// #import bevy_pbr::{
//     mesh_view_bindings::globals,
//     mesh_bindings::mesh,
//     mesh_functions as mesh_functions,
//     view_transformations::position_world_to_clip,
//     forward_io::{Vertex,VertexOutput},
// }
// #import bevy_render::instance_index::get_instance_index

struct CustomMaterial {
    offset: f32,
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: CustomMaterial;

// struct Vertex {
//     @builtin(instance_index) instance_index: u32,
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>
// };

// @vertex
// fn vertex(vertex: Vertex) -> VertexOutput {

//     var model = mesh_functions::get_model_matrix(vertex.instance_index);

//     var out: VertexOutput;

//     out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, get_instance_index(vertex.instance_index));

//     let y = abs(sin(globals.time + material.offset / 3.0));

//     var position: vec3<f32>;
//     if vertex.position.y > 0.0 {
//         position = vec3(vertex.position.x, vertex.position.y + y + 0.4, vertex.position.z);
//     } else {
//         position = vec3(vertex.position.x, vertex.position.y - y - 0.4, vertex.position.z);
//     }

//     out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
//     // out.position is clip_position
//     out.position = position_world_to_clip(out.world_position.xyz);

//     return out;
// }

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    if mesh.world_normal.x == 1.0 {
        return vec4(0.57, 0.76, 0.74, 1.0);
    } else if mesh.world_normal.y == 1.0 {
        return vec4(0.92, 0.90, 0.73, 1.0);
    } else {
        return vec4(0.29, 0.37, 0.57, 1.0);
    }
}
