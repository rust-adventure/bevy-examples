// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    stroke_color: vec4f,
    width: u32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var vertex_id_texture: texture_2d<f32>;
@group(0) @binding(4) var vertex_id_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dimensions = textureDimensions(vertex_id_texture);

    let diff = sobel(
            vertex_id_texture,
            dimensions,
            in.uv,
            vec2u(settings.width, settings.width)
        );

    // render just vertex storage texture
    // return textureLoad(vertex_id_texture, vec2u(in.uv * vec2f(dimensions)), 1);

    // use the differences to decide whether to show outline or not
    // step() is used to determine a cutoff for showing/not showing 
    // the outline.
    // without step() the diff would cause a intermediate mixing
    // resulting in subdued outlines.
    return mix(
        textureSample(screen_texture, texture_sampler, in.uv),
        settings.stroke_color,
        step(0.01, diff)
    );

    // render just sobel 
    // return vec4(diff,diff,diff, 1.);
}

// "sample" (a textureLoad) north/south/east/west pixels
// and compare to current pixel
// then sum the differences
fn sobel(
    vertex_id_texture: texture_2d<f32>,
    dimensions: vec2u,
    uv: vec2f,
    offset: vec2u
) -> f32 {
    let offseti: vec2i = vec2i(offset);
    let xy = vec2i(uv * vec2f(dimensions));

    let px_center     : f32 = textureLoad(vertex_id_texture, xy, 1).r;

    let px_left       : f32 = textureLoad(vertex_id_texture, xy + vec2i(-offseti.x, 0), 1).r;
    let px_left_up    : f32 = textureLoad(vertex_id_texture, xy + vec2i(-offseti.x, 1), 1).r;
    let px_left_down  : f32 = textureLoad(vertex_id_texture, xy + vec2i(-offseti.x, -offseti.y), 1).r;

    let px_up         : f32 = textureLoad(vertex_id_texture, xy + vec2i(0,offseti.y), 1).r;

    let px_right      : f32 = textureLoad(vertex_id_texture, xy + vec2i(offseti.x, 0), 1).r;
    let px_right_up   : f32 = textureLoad(vertex_id_texture, xy + vec2i(offseti.x, offseti.y), 1).r;
    let px_right_down : f32 = textureLoad(vertex_id_texture, xy + vec2i(offseti.x, -offseti.y), 1).r;

    let px_down       : f32 = textureLoad(vertex_id_texture, xy + vec2i(0, -offseti.y), 1).r;

    return max(
        abs(
              1 * px_left_down
            + 2 * px_left
            + 1 * px_left_up
            - 1 * px_right_down
            - 2 * px_right
            - 1 * px_right_up
        ),
        abs(
              1 * px_left_up
            + 2 * px_up
            + 1 * px_right_up
            - 1 * px_left_down
            - 2 * px_down
            - 1 * px_right_down
        )
    );
}

// sample using roberts cross
fn roberts(
    vertex_id_texture: texture_2d<f32>,
    dimensions: vec2u,
    uv: vec2f,
    offset: vec3u
) -> f32 {
    let xy = vec2u(uv * vec2f(dimensions));
    let px_center: f32 = textureLoad(vertex_id_texture, xy, 1).r;
    let px_left  : f32 = textureLoad(vertex_id_texture, xy - offset.xz, 1).r;
    let px_right : f32 = textureLoad(vertex_id_texture, xy + offset.xz, 1).r;
    let px_up    : f32 = textureLoad(vertex_id_texture, xy + offset.zy, 1).r;
    let px_down  : f32 = textureLoad(vertex_id_texture, xy - offset.zy, 1).r;

    return abs(px_left - px_center)  +
           abs(px_right - px_center) +
           abs(px_up - px_center)    +
           abs(px_down - px_center);
}
