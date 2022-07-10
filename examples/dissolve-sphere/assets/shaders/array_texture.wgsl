#import bevy_shader_utils::simplex_noise_3d
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions


struct CustomMaterial {
    time: f32;
    // flags: u32;
};

[[group(1), binding(0)]]
var<uniform> my_material: CustomMaterial;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
#ifdef VERTEX_COLORS
    [[location(4)]] color: vec4<f32>;
#endif
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let layer = i32(in.world_position.x) & 0x3;

    // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
    // the material members
    var pbr_input: PbrInput = pbr_input_new();

    // pbr_input.material.base_color = textureSample(my_array_texture, my_array_texture_sampler, in.uv, layer);
    pbr_input.material.base_color =  vec4<f32>(0.533, 0.533, 0.80, 1.0);

#ifdef VERTEX_COLORS
    pbr_input.material.base_color = pbr_input.material.base_color * in.color;
#endif

    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = prepare_normal(
        pbr_input.material.flags,
        // my_material.flags,
        in.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
        in.uv,
        in.is_front,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    var base_color = pbr(pbr_input) * pbr_input.material.base_color;
     var noise_step = .02;
    // var base_color = vec3<f32>(0.533, 0.533, 0.80);

    var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step,in.frag_coord.y * noise_step,in.frag_coord.z * noise_step));
    var threshold = sin(my_material.time);
    var alpha = step(noise, threshold);

var edge_color = vec3<f32>(0.0, 1.0, 0.8);
var border_step = smoothStep(threshold - 0.2, threshold + 0.2, noise);
var dissolve_border = vec3<f32>(edge_color.x * border_step,edge_color.y * border_step,edge_color.z * border_step);

var output_color = vec4<f32>(base_color.x + dissolve_border.x,base_color.y + dissolve_border.y,base_color.z + dissolve_border.z, alpha);

// if (output_color.a == 0.0) { discard; } else {
return output_color;
// }
}