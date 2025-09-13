
const HOW_CLOSE_IS_CLOSE_ENOUGH = 0.001;

// This is basically how big our scene is. each ray will be shot forward
// until it reaches this distance. the smaller it is, the quicker the 
// ray will reach the edge, which should help speed up this function
const FURTHEST_OUR_RAY_CAN_REACH = 10.0;

// This is how may steps our ray can take.
const HOW_MANY_STEPS_CAN_OUR_RAY_TAKE: i32 = 100;

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SdfMaterial {
    color: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: SdfMaterial;


// an sdf sphere at a specific position with a specific radius
// it also has a hardcoded id for fun manual color choices later
fn sdf_sphere(
    ray_position: vec3<f32>,
    position: vec3<f32>,
    radius: f32
) -> vec2<f32> {

    let distance_to_sphere_center: f32 = length(ray_position - position);
    let distance_to_sphere_surface: f32 = distance_to_sphere_center - radius;
    
  // this number is completely arbitrary and doesn't even need 
  // to be unique. Consider it a "type of sphere" identifier
    let sphere_id: f32 = 1.;

    return vec2(distance_to_sphere_surface, sphere_id);
}

// Takes in the position of the ray, and feeds back
// 2 values of how close it is to things in the world
// what thing it is closest two in the world.
fn sdf_world(
    ray_position: vec3<f32>
) -> vec2<f32> {
    let world = sdf_sphere(
        ray_position.xyz,
        vec3(0., 0., -.4),
        0.9
    );

    return world;
}



fn check_ray_hit(
    eyePosition: vec3<f32>,
    rayDirection: vec3<f32>
) -> vec2<f32> {
  // First we set some default values
  // our distance to surface will get overwritten every step,
  // so all that is important is that it is greater than our
  // 'how close is close enough' value
    var distanceToSurface: f32 = HOW_CLOSE_IS_CLOSE_ENOUGH * 2.;
    
  // The total distance traveled by the ray should start at 0
    var totalDistanceTraveledByRay: f32 = 0.;
    
  // if we hit something, this value will be overwritten by the
  // totalDistance traveled, and if we don't hit something it will
  // be overwritten by the furthest our ray can reach,
  // so it can be whatever!
    var finalDistanceTraveledByRay: f32 = -1.;
    
  // if our id is less that 0. , it means we haven't hit anything
  // so lets start by saying we haven't hit anything!
    var finalID: f32 = -1.;

    for (var i: i32 = 0; i < HOW_MANY_STEPS_CAN_OUR_RAY_TAKE; i++) {
    // First off, stop the iteration, if we are close enough to the surface!
        if distanceToSurface < HOW_CLOSE_IS_CLOSE_ENOUGH {
        break;
        }
      
    // Second off, stop the iteration, if we have reached the end of our scene! 
        if totalDistanceTraveledByRay > FURTHEST_OUR_RAY_CAN_REACH {
        break;
        }
    
    // To check how close we are to things in the world,
    // we need to get a position in the scene. to do this, 
    // we start at the rays origin, AKA the eye
    // and move along the ray direction, the amount we have already traveled.
        let currentPositionOfRay: vec3<f32> = eyePosition + rayDirection * totalDistanceTraveledByRay;
    
    // Distance to and ID of things in the world
        let distanceAndIDOfThingsInTheWorld: vec2<f32> = sdf_world(currentPositionOfRay);
      
 	// we get out the results from our mapping of the world
    // I am reassigning them for clarity
        let distanceToThingsInTheWorld: f32 = distanceAndIDOfThingsInTheWorld.x;
        let idOfClosestThingInTheWorld: f32 = distanceAndIDOfThingsInTheWorld.y;
     
    // We save out the distance to the surface, so that
    // next iteration we can check to see if we are close enough 
    // to stop all this silly iteration
        distanceToSurface = distanceToThingsInTheWorld;
      
    // We are also finalID to the current closest id,
    // because if we hit something, we will have the proper
    // id, and we can skip reassigning it later!
        finalID = idOfClosestThingInTheWorld;

        totalDistanceTraveledByRay += distanceToThingsInTheWorld;
    }

  // if we hit something set the finalDirastnce traveled by
  // ray to that distance!
    if totalDistanceTraveledByRay < FURTHEST_OUR_RAY_CAN_REACH {
        finalDistanceTraveledByRay = totalDistanceTraveledByRay;
    }
    
    
  // If the total distance traveled by the ray is further than
  // the ray can reach, that means that we've hit the edge of the scene
  // Set the final distance to be the edge of the scene
  // and the id to -1 to make sure we know we haven't hit anything
    if totalDistanceTraveledByRay > FURTHEST_OUR_RAY_CAN_REACH {
        finalDistanceTraveledByRay = FURTHEST_OUR_RAY_CAN_REACH;
        finalID = -1.;
    }

    return vec2(finalDistanceTraveledByRay, finalID);
}

fn calculate_transformation_matrix(
    ray_origin: vec3<f32>,
    target_position: vec3<f32>,
    roll: f32
) -> mat3x3<f32> {
    let forward: vec3<f32> = normalize(target_position - ray_origin);
    let right: vec3<f32> = normalize(cross(forward, vec3(sin(roll), cos(roll), 0.0)));
    let up: vec3<f32> = normalize(cross(right, forward));
    return mat3x3(right, up, forward);
}


// Here we are calcuting the normal of the surface
// This code figures out in what direction the SDF is increasing.
// 
// (SDFs are 0 at the barrier, lower internally, and higher
// the further away you get)
//
// This value is the same thing as telling you what direction
// the surface faces, AKA the normal of the surface. 
fn get_normal_of_surface(position_of_hit: vec3<f32>) -> vec3<f32> {

    let tiny_change_x = vec3(0.001, 0.0, 0.0);
    let tiny_change_y = vec3(0.0, 0.001, 0.0);
    let tiny_change_z = vec3(0.0, 0.0, 0.001);

    let up_tiny_change_in_x: f32 = sdf_world(position_of_hit + tiny_change_x).x;
    let down_tiny_change_in_x: f32 = sdf_world(position_of_hit - tiny_change_x).x;

    let tiny_change_in_x: f32 = up_tiny_change_in_x - down_tiny_change_in_x;


    let up_tiny_change_in_y: f32 = sdf_world(position_of_hit + tiny_change_y).x;
    let down_tiny_change_in_y: f32 = sdf_world(position_of_hit - tiny_change_y).x;

    let tiny_change_in_y: f32 = up_tiny_change_in_y - down_tiny_change_in_y;


    let up_tiny_change_in_z: f32 = sdf_world(position_of_hit + tiny_change_z).x;
    let down_tiny_change_in_z: f32 = sdf_world(position_of_hit - tiny_change_z).x;

    let tiny_change_in_z: f32 = up_tiny_change_in_z - down_tiny_change_in_z;


    let normal = vec3(
        tiny_change_in_x,
        tiny_change_in_y,
        tiny_change_in_z
    );

    return normalize(normal);
}

fn color_sphere(
    position_of_hit: vec3<f32>,
    normal_of_surface: vec3<f32>
) -> vec3<f32> {
    let position_of_sun = vec3(1., 4., 3.);
    let sphere_color = material.color.xyz;

    // the direction of the light from the sun
    // to the position of the hit
    let direction_of_light = normalize(position_of_sun - position_of_hit);

    // how much the surface faces the light direction
    var surface_light_similarity: f32 = dot(
        direction_of_light,
        normal_of_surface
    );
	
    // if the face value is negative, just make it 0.
    // so it doesn't give back negative light values
    surface_light_similarity = max(0., surface_light_similarity);

   	// our final color is the sphere color multiplied
    // by how much the surface faces the light
    var color: vec3<f32> = sphere_color * surface_light_similarity;
    
    // add in a bit of ambient color
    // just so we don't get any pure black
    // this effectively colors everything a bit,
    // but is most noticable in the shadow.
    color += vec3(0.15, 0.05, 0.1);

    return color;
}

fn calculate_color(
    ray_hit_info: vec2<f32>,
    position_of_eye: vec3<f32>,
    ray: vec3<f32>
) -> vec3<f32> {
    var color: vec3<f32>;

    if ray_hit_info.y < 0.0 {
      // ray missed
        color = vec3(0.0);
    } else {
      // ray hit
        let position_of_hit = position_of_eye + ray_hit_info.x * ray;
        let normal_of_surface = get_normal_of_surface(position_of_hit);

      // 1.0 is the sphere ID
        if ray_hit_info.y == 1.0 {
            color = color_sphere(position_of_hit, normal_of_surface);
        }
    }
    return color;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let position_of_eye = vec3(0., 0., 2.); 
    // let position_of_eye = view.world_position;   
    
    // move camera
    // position_of_eye = vec3( sin(time.time), 0., 2.);
    let looking_at = vec3(0., 0., 0.);
  
    // mat3
    let eye_matrix = calculate_transformation_matrix(
        position_of_eye,
        looking_at,
        0.
    ); 
   
    // from 0..1 to -1..1
    var uvish = in.uv.xy * 2.0 - 1.0;
    // flip y axis
    uvish.y = -1.0 * uvish.y;
    // var uvish = in.uv.xy;
    let ray_from_eye = normalize(
        eye_matrix * vec3(uvish, 2.)
    );

    let ray_hit_info = check_ray_hit(
        position_of_eye,
        ray_from_eye.xyz
    );

    let color = calculate_color(
        ray_hit_info,
        position_of_eye,
        ray_from_eye.xyz
    );

    if color.r + color.g + color.b < 0.1 {
        // discard fragments if rays didn't hit anything
        // this removes the rectangular border around the sphere
        discard;
    }
    return vec4(color, 1.0);
}

