// This shader draws a circle with a given input color
#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> input: CustomUiMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    let uv = in.uv * 2.0 - 1.0;
    
    let distance = sd_rounded_box(uv, vec2(0.89,0.89), vec4(0.1));
    let mix_value = step(0.1, distance);
    let mix_value_2 = step(0.11, distance);

    let color = mix(
        vec4(input.color.rgb, 1.0),
        vec4(1.,1.,1.,1.),
        mix_value
    );
    let color_with_border = mix(
        color,
        vec4(1.,1.,1.,0.),
        mix_value_2,
    );
    return color_with_border;

}

fn sd_rounded_box(point: vec2f, b: vec2f, border_radii: vec4f ) -> f32
{
    var radii = border_radii;
    if (point.x<=0.0) {
        radii.x = radii.z;
        radii.y = radii.w;
    };
    if(point.y<=0.0) { radii.x  = radii.y;};
    let q: vec2f = abs(point)-b+radii.x;
    return min(max(q.x,q.y),0.0) + length(max(q,vec2(0.0))) - radii.x;
}