#import bevy_shader_utils::simplex_noise_3d
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct CustomMaterial {
    time: f32;
};

[[group(1), binding(0)]]
var<uniform> my_material: CustomMaterial;

// [[stage(fragment)]]
// fn fragment([[location(2)]] uv: vec2<f32>) -> [[location(0)]] vec4<f32> {
//     // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
//     var input: vec3<f32> = vec3<f32>(uv.x * 40.0, uv.y * 40.0, 1.);
//     var noise = perlinNoise3(input);
//     var alpha = (noise + 1.0) / 2.0;
//     return vec4<f32>(1.0,1.0,1.0,alpha);
// }

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(in: FragmentInput)-> [[location(0)]] vec4<f32>   {

    var noise_step = .02;
    var base_color = vec3<f32>(0.533, 0.533, 0.80);

    var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step,in.frag_coord.y * noise_step,in.frag_coord.z * noise_step));
    var threshold = sin(my_material.time);
    var alpha = step(noise, threshold);

var edge_color = vec3<f32>(0.0, 1.0, 0.8);
var border_step = smoothStep(threshold - 0.2, threshold + 0.2, noise);
var dissolve_border = vec3<f32>(edge_color.x * border_step,edge_color.y * border_step,edge_color.z * border_step);

var output_color = vec4<f32>(base_color.x + dissolve_border.x,base_color.y + dissolve_border.y,base_color.z + dissolve_border.z, alpha);


  // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
        // the material members
        var pbr_input: PbrInput;

        pbr_input.material.base_color = output_color;
        pbr_input.material.reflectance = material.reflectance;
        pbr_input.material.flags = material.flags;
        pbr_input.material.alpha_cutoff = material.alpha_cutoff;

        // TODO use .a for exposure compensation in HDR
        var emissive: vec4<f32> = material.emissive;
        if ((material.flags & STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
            emissive = vec4<f32>(emissive.rgb * textureSample(emissive_texture, emissive_sampler, in.uv).rgb, 1.0);
        }
        pbr_input.material.emissive = emissive;

        var metallic: f32 = material.metallic;
        var perceptual_roughness: f32 = material.perceptual_roughness;
        if ((material.flags & STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT) != 0u) {
            let metallic_roughness = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in.uv);
            // Sampling from GLTF standard channels for now
            metallic = metallic * metallic_roughness.b;
            perceptual_roughness = perceptual_roughness * metallic_roughness.g;
        }
        pbr_input.material.metallic = metallic;
        pbr_input.material.perceptual_roughness = perceptual_roughness;

        var occlusion: f32 = 1.0;
        if ((material.flags & STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
            occlusion = textureSample(occlusion_texture, occlusion_sampler, in.uv).r;
        }
        pbr_input.occlusion = occlusion;

        pbr_input.frag_coord = in.frag_coord;
        pbr_input.world_position = in.world_position;
        pbr_input.world_normal = in.world_normal;

        pbr_input.is_orthographic = view.projection[3].w == 1.0;

        pbr_input.N = prepare_normal(
            material.flags,
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

        output_color = tone_mapping(pbr(pbr_input));

return output_color;
   
}
//    @builtin(position) coord: vec4<f32>,
