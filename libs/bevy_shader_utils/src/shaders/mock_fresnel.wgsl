#define_import_path bevy_shader_utils::fresnel

fn mock_fresnel(
    camera_view_world_position: vec3<f32>,
    world_position: vec3<f32>,
    world_normal: vec3<f32>,
    power: f32,
    strength: f32,
) -> f32 {
    // The view vector. V is a unit vector pointing from the fragment
    // on the sphere toward the camera.
    //
    // this comment is how you would write it in your own code
    // var V = normalize(view.world_position.xyz - world_position.xyz);
    var V = normalize(camera_view_world_position - world_position);

    // The dot product returns the angle between N and V where 
    // fragments on the sphere that are pointing at the camera
    // (have the same angle as the V) are 1.0, faces perpendicular 
    // to V are 0.0, faces pointing away are -1.0. This is why we 
    // clamp the value here, to make sure we don't end up with 
    // negative numbers for NdotV.
    let NdotV = max(dot(world_normal, V), 0.0001);
    
    // The fresnel value here is the inverse of NdotV. 
    // So fragments pointing away will now be 1.0 and ones 
    // pointing at the camera will be 0.0
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);

    // Here's were increasing the contrast with pow 
    // and making it brighter by multiplying by 2
    return pow(fresnel, power) * strength;
};

fn fresnel(
    camera_view_world_position: vec3<f32>,
    world_position: vec3<f32>,
    world_normal: vec3<f32>,
    power: f32,
    strength: f32,
) -> f32 {
    // The view vector. V is a unit vector pointing from the fragment
    // on the sphere toward the camera.
    //
    // this comment is how you would write it in your own code
    // var V = normalize(view.world_position.xyz - world_position.xyz);
    var V = normalize(camera_view_world_position - world_position);

    // The dot product returns the angle between N and V where 
    // fragments on the sphere that are pointing at the camera
    // (have the same angle as the V) are 1.0, faces perpendicular 
    // to V are 0.0, faces pointing away are -1.0.
    var fresnel = 1.0 - dot(world_normal, V);

    // The fresnel value here is the inverse of NdotV. 
    // So fragments pointing away will now be 1.0 and ones 
    // pointing at the camera will be 0.0
    // var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);

    // Here's were increasing the contrast with pow 
    // and making it brighter by multiplying by 2
    return pow(fresnel, power) * strength;
};