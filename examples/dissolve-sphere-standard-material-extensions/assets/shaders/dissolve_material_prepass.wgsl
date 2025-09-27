#import bevy_pbr::prepass_io::{Vertex, VertexOutput}
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@fragment
fn fragment(in: VertexOutput) {
    // NOTE: The PREPASS_FRAGMENT shader def is responsible for
    // controlling whether this prepass returns a FragmentOutput
    // or not.
    // We never check it here, so *if* you were using the normal
    // prepass texture in the main pass, it would no information.


    // same calcuation for gaps as the material, but without color
    // output. 
    var noise_step = 4.0;
    let c = in.color.xyz;
    var noise = simplex_noise_3d(c * noise_step);
    var threshold = sin(globals.time);
    var alpha = step(noise, threshold);
    if alpha <= 0.01 { discard; };
}
