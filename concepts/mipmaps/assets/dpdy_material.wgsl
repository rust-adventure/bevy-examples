#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let dy = dpdy(mesh.uv.y);
    // * 500. is arbitrary and only serves to bump the number up
    // into a range we can visualize
    // TODO: show values in xcode
    return vec4(abs(dy) * 500., 0., 0., 1.);
}
