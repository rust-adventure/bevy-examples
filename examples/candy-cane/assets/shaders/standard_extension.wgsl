#import bevy_pbr::mesh_view_bindings
// #import bevy_pbr::pbr_bindings
// #import bevy_pbr::pbr_types
struct StandardMaterial {
    time: f32,
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    alpha_cutoff: f32,
}

let STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT: u32         = 1u;
let STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT: u32           = 2u;
let STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT: u32 = 4u;
let STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT: u32          = 8u;
let STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT: u32               = 16u;
let STANDARD_MATERIAL_FLAGS_UNLIT_BIT: u32                      = 32u;
let STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE: u32              = 64u;
let STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MASK: u32                = 128u;
let STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND: u32               = 256u;
let STANDARD_MATERIAL_FLAGS_TWO_COMPONENT_NORMAL_MAP: u32       = 512u;
let STANDARD_MATERIAL_FLAGS_FLIP_NORMAL_MAP_Y: u32              = 1024u;

// Creates a StandardMaterial with default values
fn standard_material_new() -> StandardMaterial {
    var material: StandardMaterial;

    // NOTE: Keep in-sync with src/pbr_material.rs!
    material.base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    material.emissive = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    material.perceptual_roughness = 0.089;
    material.metallic = 0.01;
    material.reflectance = 0.5;
    material.flags = STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;
    material.alpha_cutoff = 0.5;
    material.time = 0.0;

    return material;
}

#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

#import bevy_shader_utils::simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d

@group(1) @binding(0)
var<uniform> material: StandardMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;
@group(1) @binding(5)
var metallic_roughness_texture: texture_2d<f32>;
@group(1) @binding(6)
var metallic_roughness_sampler: sampler;
@group(1) @binding(7)
var occlusion_texture: texture_2d<f32>;
@group(1) @binding(8)
var occlusion_sampler: sampler;
@group(1) @binding(9)
var normal_map_texture: texture_2d<f32>;
@group(1) @binding(10)
var normal_map_sampler: sampler;


struct FragmentInput {
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
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

/// Begin Candy Cane Colors

    let u_sin = abs(sin(in.uv.x));

/// End Candy Cane Colors

    // var output_color = vec4<f32>(0.533, 0.533, 0.80, 1.0);

    var output_color: vec4<f32> = material.base_color * u_sin;
// #ifdef VERTEX_COLORS
//     output_color = output_color * in.color;
// #endif
#ifdef VERTEX_UVS
    if ((material.flags & STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    }
#endif

    // NOTE: Unlit bit not set means == 0 is true, so the true case is if lit
    if ((material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
        // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
        // the material members
        var pbr_input: PbrInput;

        pbr_input.material.base_color = output_color;
        pbr_input.material.reflectance = material.reflectance;
        pbr_input.material.flags = material.flags;
        pbr_input.material.alpha_cutoff = material.alpha_cutoff;

        // TODO use .a for exposure compensation in HDR
        var emissive: vec4<f32> = material.emissive;
#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
            emissive = vec4<f32>(emissive.rgb * textureSample(emissive_texture, emissive_sampler, in.uv).rgb, 1.0);
        }
#endif
        pbr_input.material.emissive = emissive;

        var metallic: f32 = material.metallic;
        var perceptual_roughness: f32 = material.perceptual_roughness;
#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT) != 0u) {
            let metallic_roughness = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in.uv);
            // Sampling from GLTF standard channels for now
            metallic = metallic * metallic_roughness.b;
            perceptual_roughness = perceptual_roughness * metallic_roughness.g;
        }
#endif
        pbr_input.material.metallic = metallic;
        pbr_input.material.perceptual_roughness = perceptual_roughness;

        var occlusion: f32 = 1.0;
#ifdef VERTEX_UVS
        if ((material.flags & STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
            occlusion = textureSample(occlusion_texture, occlusion_sampler, in.uv).r;
        }
#endif
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
#ifdef VERTEX_UVS
            in.uv,
#endif
            in.is_front,
        );
        pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

        output_color = tone_mapping(pbr(pbr_input));
    }

    // return output_color;
// custom code
//     var base_color = output_color;
    
// //  var base_color = vec4<f32>(1.0,0.4,1.0,1.0);
//     var noise_step = 5.0;
//     // var base_color = vec3<f32>(0.533, 0.533, 0.80);

//     // var noise = simplexNoise3(vec3<f32>(in.frag_coord.x * noise_step, in.frag_coord.y * noise_step, in.frag_coord.z * noise_step));
//     var noise = simplexNoise3(vec3<f32>(in.color.x * noise_step, in.color.y * noise_step, in.color.z * noise_step));
//     var threshold = sin(material.time);
//     var alpha = step(noise, threshold);

//     // var edge_color = vec3<f32>(0.0, 1.0, 0.8);
//     var edge_color = output_color * 3.0;
//     var border_step = smoothstep(threshold - 0.2, threshold + 0.2, noise);
//     var dissolve_border = vec3<f32>(edge_color.x * border_step, edge_color.y * border_step, edge_color.z * border_step);

//     var output_color = vec4<f32>(
//         base_color.x + dissolve_border.x,
//         base_color.y + dissolve_border.y,
//         base_color.z + dissolve_border.z,
//         alpha
//     );
// var test = (simplexNoise3(vec3<f32>(material.time, material.time, material.time)) + 1.0) / 2.0;
// return vec4<f32>(test, test,test,test);
    if (output_color.a == 0.0) { discard; } else {
        return output_color;
    }
}