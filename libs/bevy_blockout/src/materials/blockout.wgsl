#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

#import bevy::bevy_render::color_operations::hsv_to_rgb
#import bevy::bevy_render::color_operations::rgb_to_hsv

struct BlockoutMaterial {
    line_color: vec4f,
    color: vec4f,
    cell_multiplier: vec2f,
    line_size: vec2f
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> extension: BlockoutMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    pbr_input.material.base_color = mix(
        vec4(blockout(in)),
        extension.line_color,
        // mix the y world normal between the desaturated color and the "core" side color.
        // multiply it by the checkerboards, which overlap and darken their respective boxes
        mix(vec4(in.world_normal.y), pbr_input.material.base_color, extension.color) * vec4(checkerboard(in, 1.) + checkerboard(in, 5.), 1.)
    );

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    return out;
}

// a checkerboard pattern
// built using `mod`
fn checker(axis: vec2f, size: f32) -> f32 {
    let pos = floor(axis.xy / size);
    let tile = (pos.x + (pos.y % 2.0)) % 2.0;
    return abs(tile);
}

// a triplanar checkerboard built from 3 axis of checker()s
fn checkerboard(mesh: VertexOutput, size: f32) -> vec3f {

    // get the f32 value for each axis,
    // lock the lower end to a min of 0.3 so we don't get black grid positions
    let x = max(0.3, checker(mesh.world_position.zy, size));
    let y = max(0.3, checker(mesh.world_position.xz, size));
    let z = max(0.3, checker(mesh.world_position.xy, size));

    // blend the albedos per-axis based on the normal direction
    let normal = abs(mesh.world_normal);
    let weights = normal / (normal.x + normal.y + normal.z);
    let checkers = (x * weights.x + y * weights.y + z * weights.z);

    return vec3(checkers);
}

// Build a triplanar pristine grid albedo, which
// we'll later combine with the checkerboard, preferring
// to prioritize the lines over the checkboard colors
fn blockout(
    mesh: VertexOutput
) -> f32 {

    let x = pristine_grid(
        mesh.world_position.zy,
        extension.line_size
    );

    let y = pristine_grid(
        mesh.world_position.xz,
        extension.line_size
    );

    let z = pristine_grid(
        mesh.world_position.xy,
        extension.line_size
        // mix(step(
        //     vec2(0.2),
        //     modf(mesh.world_position.xy / 5.).fract,
        // ),
        //     vec2(0.1),
        //     vec2(0.1))
    );

    let normal = abs(mesh.world_normal);
    let weights = normal / (normal.x + normal.y + normal.z);

    return (x * weights.x + y * weights.y + z * weights.z);
}

// Pristine grid from The Best Darn Grid Shader (yet)
// https://bgolus.medium.com/the-best-darn-grid-shader-yet-727f9278b9d8

fn pristine_grid(uv: vec2f, lineWidth: vec2f) -> f32 {
    var ddx: vec2f = dpdx(uv);
    var ddy: vec2f = dpdy(uv);
    var uvDeriv: vec2f = vec2(length(vec2(ddx.x, ddy.x)), length(vec2(ddx.y, ddy.y)));
    let invertLine: vec2<bool> = vec2<bool>(lineWidth.x > 0.5, lineWidth.y > 0.5);
    var targetWidth: vec2<f32>;
    if invertLine.x {
        targetWidth.x = 1.0 - lineWidth.x;
    } else {
        targetWidth.x = lineWidth.x;
    };
    if invertLine.y {
        targetWidth.y = 1.0 - lineWidth.y;
    } else {
        targetWidth.y = lineWidth.y;
    };
    let drawWidth: vec2f = clamp(targetWidth, uvDeriv, vec2(0.5));
    let lineAA: vec2f = uvDeriv * 1.5;
    var gridUV: vec2f = abs(fract(uv) * 2.0 - 1.0);
    if invertLine.x { gridUV.x = gridUV.x; } else { gridUV.x = 1.0 - gridUV.x; };
    if invertLine.y { gridUV.y = gridUV.y; } else { gridUV.y = 1.0 - gridUV.y; };
    var grid2: vec2f = smoothstep(drawWidth + lineAA, drawWidth - lineAA, gridUV);

    grid2 *= clamp(targetWidth / drawWidth, vec2(0.0), vec2(1.0));
    grid2 = mix(grid2, targetWidth, clamp(uvDeriv * 2.0 - 1.0, vec2(0.0), vec2(1.0)));
    if invertLine.x {
        grid2.x = 1.0 - grid2.x;
    };// else { grid2.x = grid2.x };
    if invertLine.y {
        grid2.y = 1.0 - grid2.y;
    }; // else { grid2.y = grid2.y };
    return mix(grid2.x, 1.0, grid2.y);
}

// TODO: maybe use lab/lch for nicer color defaults?

//By Björn Ottosson
//https://bottosson.github.io/posts/oklab
//Shader functions adapted by "mattz"
//https://www.shadertoy.com/view/WtccD7
fn oklab_from_linear(linear: vec3f) -> vec3f {
    let im1: mat3x3<f32> = mat3x3<f32>(0.4121656120, 0.2118591070, 0.0883097947,
        0.5362752080, 0.6807189584, 0.2818474174,
        0.0514575653, 0.1074065790, 0.6302613616);

    let im2: mat3x3<f32> = mat3x3<f32>(0.2104542553, 1.9779984951, 0.0259040371,
        0.7936177850, -2.4285922050, 0.7827717662,
        -0.0040720468, 0.4505937099, -0.8086757660);

    let lms: vec3f = im1 * linear;

    return im2 * (sign(lms) * pow(abs(lms), vec3(1.0 / 3.0)));
}

fn linear_from_oklab(oklab: vec3f) -> vec3f {
    let m1: mat3x3<f32> = mat3x3<f32>(1.000000000, 1.000000000, 1.000000000,
        0.396337777, -0.105561346, -0.089484178,
        0.215803757, -0.063854173, -1.291485548);

    let m2: mat3x3<f32> = mat3x3<f32>(4.076724529, -1.268143773, -0.004111989,
        -3.307216883, 2.609332323, -0.703476310,
        0.230759054, -0.341134429, 1.706862569);
    let lms: vec3f = m1 * oklab;

    return m2 * (lms * lms * lms);
}
//By Inigo Quilez, under MIT license
//https://www.shadertoy.com/view/ttcyRS
fn oklab_mix(lin1: vec3f, lin2: vec3f, a: f32) -> vec3f {
    // https://bottosson.github.io/posts/oklab
    let kCONEtoLMS: mat3x3<f32> = mat3x3<f32>(
        0.4121656120, 0.2118591070, 0.0883097947,
        0.5362752080, 0.6807189584, 0.2818474174,
        0.0514575653, 0.1074065790, 0.6302613616
    );
    let kLMStoCONE: mat3x3<f32> = mat3x3<f32>(
        4.0767245293, -1.2681437731, -0.0041119885,
        -3.3072168827, 2.6093323231, -0.7034763098,
        0.2307590544, -0.3411344290, 1.7068625689
    );
                    
    // rgb to cone (arg of pow can't be negative)
    let lms1: vec3f = pow(kCONEtoLMS * lin1, vec3(1.0 / 3.0));
    let lms2: vec3f = pow(kCONEtoLMS * lin2, vec3(1.0 / 3.0));
    // lerp
    var lms: vec3f = mix(lms1, lms2, a);
    // gain in the middle (no oklab anymore, but looks better?)
    lms *= 1.0 + 0.2 * a * (1.0 - a);
    // cone to rgb
    return kLMStoCONE * (lms * lms * lms);
}