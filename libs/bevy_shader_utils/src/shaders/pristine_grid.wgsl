// Pristine grid from The Best Darn Grid Shader (yet)
// https://bgolus.medium.com/the-best-darn-grid-shader-yet-727f9278b9d8

#define_import_path bevy_shader_utils::pristine_grid

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

// // version with explicit gradients for use with raycast shaders like this one
// float pristineGrid( in vec2 uv, in vec2 ddx, in vec2 ddy, vec2 lineWidth)
// {
//     vec2 uvDeriv = vec2(length(vec2(ddx.x, ddy.x)), length(vec2(ddx.y, ddy.y)));
//     bvec2 invertLine = bvec2(lineWidth.x > 0.5, lineWidth.y > 0.5);
//     vec2 targetWidth = vec2(
//       invertLine.x ? 1.0 - lineWidth.x : lineWidth.x,
//       invertLine.y ? 1.0 - lineWidth.y : lineWidth.y
//       );
//     vec2 drawWidth = clamp(targetWidth, uvDeriv, vec2(0.5));
//     vec2 lineAA = uvDeriv * 1.5;
//     vec2 gridUV = abs(fract(uv) * 2.0 - 1.0);
//     gridUV.x = invertLine.x ? gridUV.x : 1.0 - gridUV.x;
//     gridUV.y = invertLine.y ? gridUV.y : 1.0 - gridUV.y;
//     vec2 grid2 = smoothstep(drawWidth + lineAA, drawWidth - lineAA, gridUV);

//     grid2 *= clamp(targetWidth / drawWidth, 0.0, 1.0);
//     grid2 = mix(grid2, targetWidth, clamp(uvDeriv * 2.0 - 1.0, 0.0, 1.0));
//     grid2.x = invertLine.x ? 1.0 - grid2.x : grid2.x;
//     grid2.y = invertLine.y ? 1.0 - grid2.y : grid2.y;
//     return mix(grid2.x, 1.0, grid2.y);
// }
