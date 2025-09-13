#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

// selection is 1.0 for "use this normal"
// and -1.0 for "use the negative of this normal"
// and 0.0 for "don't use this normal"
// the w component is whether to use absolute value or not
struct CustomMaterial {
    selection: vec4<f32>,
    show_components: f32
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
   
    // world_normal is a vector where each value is -1.0 to 1.0
    // where the vector represents the normal direction in world-space
    // world-space aligns with the global x,y,z axis
    // 
    // For example N.x will be 1.0 for faces pointing directly in
    // the positive x direction
    var Normal = normalize(in.world_normal);

    if material.show_components == 1. {
        // .xxx is called "swizzling", which allows us to select any of
        // the components of the vector in any order, any amount of times
        // so Normal.xxx means "a vec3 where every value is the x value"
        var values_to_show: vec3<f32>;

        let selection = material.selection;

        if selection.x != 0.0 {
            values_to_show = Normal.xxx;
        } else if selection.y != 0.0 {
            values_to_show = Normal.yyy;
        } else if selection.z != 0.0 {
            values_to_show = Normal.zzz;
        } else {
            values_to_show = Normal.xyz;
        };

        if selection.w == 1.0 {
            values_to_show = abs(values_to_show);
        }

        // return vec4(Normal, 1.0);
        return vec4(values_to_show, 1.0);
    } else {
        // The view vector. V is a unit vector pointing from the fragment
        // on the sphere toward the camera.
        var V = normalize(view.world_position.xyz - in.world_position.xyz);

        // The dot product returns the angle between N and V where 
        // fragments on the sphere that are pointing at the camera
        // (have the same angle as the V) are 1.0, faces perpendicular 
        // to V are 0.0, faces pointing away are -1.0. This is why we 
        // clamp the value here, to make sure we don't end up with 
        // negative numbers for NdotV.
        let NdotV = max(dot(Normal, V), 0.0001);
        
        // return vec4(vec3(NdotV), 1.0);

        // The fresnel value here is just the inverse of NdotV. 
        // So fragments pointing away will now be 1.0 and ones 
        // pointing at the camera will be 0.0
        var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);

        // Here's were just increasing the contrast with pow 
        // and making it brighter by multiplying by 2
        fresnel = pow(fresnel, 3.0) * 2.0;

        // return vec4(vec3(fresnel), 1.0);
        let red = vec3(1.0, 0.0, 0.0);
        let green = vec3(1.0, 0.502, 0.0);
        return vec4(mix(red, green, fresnel), 1.0);
    }
}