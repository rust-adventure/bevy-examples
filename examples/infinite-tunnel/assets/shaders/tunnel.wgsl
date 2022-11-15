#import bevy_pbr::mesh_view_bindings
#import bevy_shader_utils::simplex_noise_3d

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> offset_r: vec2<f32>;
@group(1) @binding(3)
var<uniform> offset_g: vec2<f32>;
@group(1) @binding(4)
var<uniform> offset_b: vec2<f32>;

// worley
// Permutation polynomial: (34x^2 + x) mod 289
// fn permute(x: vec3<f32>) -> vec3<f32> {
//   return (34.0 * x + 1.0) * x % 289.0;
// }

// fn dist(x: vec3<f32>, y: vec3<f32>, z: vec3<f32>, manhattanDistance: bool) -> vec3<f32> {
//   if manhattanDistance ==true {
//   return abs(x) + abs(y) + abs(z);
//     } else {
//      return (x * x + y * y + z * z);
//     };
// }

// fn worley(P: vec3<f32>, jitter: f32, manhattanDistance: bool) -> vec2<f32> {
// let K: f32 = 0.142857142857; // 1/7
// let Ko: f32 = 0.428571428571; // 1/2-K/2
// let K2: f32 = 0.020408163265306; // 1/(7*7)
// let Kz: f32 = 0.166666666667; // 1/6
// let Kzo: f32 = 0.416666666667; // 1/2-1/6*2

// 	let Pi = floor(P) % 289.0;
//  	let Pf = fract(P) - 0.5;

// 	let Pfx = Pf.x + vec3(1.0, 0.0, -1.0);
// 	let Pfy = Pf.y + vec3(1.0, 0.0, -1.0);
// 	let Pfz = Pf.z + vec3(1.0, 0.0, -1.0);

// 	let p = permute(Pi.x + vec3(-1.0, 0.0, 1.0));
// 	let p1 = permute(p + Pi.y - 1.0);
// 	let p2 = permute(p + Pi.y);
// 	let p3 = permute(p + Pi.y + 1.0);

// 	let p11 = permute(p1 + Pi.z - 1.0);
// 	let p12 = permute(p1 + Pi.z);
// 	let p13 = permute(p1 + Pi.z + 1.0);

// 	let p21 = permute(p2 + Pi.z - 1.0);
// 	let p22 = permute(p2 + Pi.z);
// 	let p23 = permute(p2 + Pi.z + 1.0);

// 	let p31 = permute(p3 + Pi.z - 1.0);
// 	let p32 = permute(p3 + Pi.z);
// 	let p33 = permute(p3 + Pi.z + 1.0);

// 	let ox11 = fract(p11*K) - Ko;
// 	let oy11 = floor(p11*K) % 7.0 * K - Ko;
// 	let oz11 = floor(p11*K2)*Kz - Kzo; // p11 < 289 guaranteed

// 	let ox12 = fract(p12*K) - Ko;
// 	let oy12 = floor(p12*K) % 7.0 * K - Ko;
// 	let oz12 = floor(p12*K2)*Kz - Kzo;

// 	let ox13 = fract(p13*K) - Ko;
// 	let oy13 = floor(p13*K) % 7.0 * K - Ko;
// 	let oz13 = floor(p13*K2)*Kz - Kzo;

// 	let ox21 = fract(p21*K) - Ko;
// 	let oy21 = floor(p21*K) % 7.0 * K - Ko;
// 	let oz21 = floor(p21*K2)*Kz - Kzo;

// 	let ox22 = fract(p22*K) - Ko;
// 	let oy22 = floor(p22*K) % 7.0 * K - Ko;
// 	let oz22 = floor(p22*K2)*Kz - Kzo;

// 	let ox23 = fract(p23*K) - Ko;
// 	let oy23 = floor(p23*K) % 7.0 * K - Ko;
// 	let oz23 = floor(p23*K2)*Kz - Kzo;

// 	let ox31 = fract(p31*K) - Ko;
// 	let oy31 = floor(p31*K) % 7.0 * K - Ko;
// 	let oz31 = floor(p31*K2)*Kz - Kzo;

// 	let ox32 = fract(p32*K) - Ko;
// 	let oy32 = floor(p32*K) % 7.0 * K - Ko;
// 	let oz32 = floor(p32*K2)*Kz - Kzo;

// 	let ox33 = fract(p33*K) - Ko;
// 	let oy33 = floor(p33*K) % 7.0 * K - Ko;
// 	let oz33 = floor(p33*K2)*Kz - Kzo;

// 	let dx11 = Pfx + jitter*ox11;
// 	let dy11 = Pfy.x + jitter*oy11;
// 	let dz11 = Pfz.x + jitter*oz11;

// 	let dx12 = Pfx + jitter*ox12;
// 	let dy12 = Pfy.x + jitter*oy12;
// 	let dz12 = Pfz.y + jitter*oz12;

// 	let dx13 = Pfx + jitter*ox13;
// 	let dy13 = Pfy.x + jitter*oy13;
// 	let dz13 = Pfz.z + jitter*oz13;

// 	let dx21 = Pfx + jitter*ox21;
// 	let dy21 = Pfy.y + jitter*oy21;
// 	let dz21 = Pfz.x + jitter*oz21;

// 	let dx22 = Pfx + jitter*ox22;
// 	let dy22 = Pfy.y + jitter*oy22;
// 	let dz22 = Pfz.y + jitter*oz22;

// 	let dx23 = Pfx + jitter*ox23;
// 	let dy23 = Pfy.y + jitter*oy23;
// 	let dz23 = Pfz.z + jitter*oz23;

// 	let dx31 = Pfx + jitter*ox31;
// 	let dy31 = Pfy.z + jitter*oy31;
// 	let dz31 = Pfz.x + jitter*oz31;

// 	let dx32 = Pfx + jitter*ox32;
// 	let dy32 = Pfy.z + jitter*oy32;
// 	let dz32 = Pfz.y + jitter*oz32;

// 	let dx33 = Pfx + jitter*ox33;
// 	let dy33 = Pfy.z + jitter*oy33;
// 	let dz33 = Pfz.z + jitter*oz33;

// 	var d11 = dist(dx11, dy11, dz11, manhattanDistance);
// 	var d12 =dist(dx12, dy12, dz12, manhattanDistance);
// 	var d13 = dist(dx13, dy13, dz13, manhattanDistance);
// 	var d21 = dist(dx21, dy21, dz21, manhattanDistance);
// 	var d22 = dist(dx22, dy22, dz22, manhattanDistance);
// 	var d23 = dist(dx23, dy23, dz23, manhattanDistance);
// 	var d31 = dist(dx31, dy31, dz31, manhattanDistance);
// 	var d32 = dist(dx32, dy32, dz32, manhattanDistance);
// 	var d33 = dist(dx33, dy33, dz33, manhattanDistance);

// 	let d1a = min(d11, d12);
// 	d12 = max(d11, d12);
// 	d11 = min(d1a, d13); // Smallest now not in d12 or d13
// 	d13 = max(d1a, d13);
// 	d12 = min(d12, d13); // 2nd smallest now not in d13
// 	let d2a = min(d21, d22);
// 	d22 = max(d21, d22);
// 	d21 = min(d2a, d23); // Smallest now not in d22 or d23
// 	d23 = max(d2a, d23);
// 	d22 = min(d22, d23); // 2nd smallest now not in d23
// 	let d3a = min(d31, d32);
// 	d32 = max(d31, d32);
// 	d31 = min(d3a, d33); // Smallest now not in d32 or d33
// 	d33 = max(d3a, d33);
// 	d32 = min(d32, d33); // 2nd smallest now not in d33
// 	let da = min(d11, d21);
// 	d21 = max(d11, d21);
// 	d11 = min(da, d31); // Smallest now in d11
// 	d31 = max(da, d31); // 2nd smallest now not in d31
// 	d11.xy = (d11.x < d11.y) ? d11.xy : d11.yx;
// 	d11.xz = (d11.x < d11.z) ? d11.xz : d11.zx; // d11.x now smallest
// 	d12 = min(d12, d21); // 2nd smallest now not in d21
// 	d12 = min(d12, d22); // nor in d22
// 	d12 = min(d12, d31); // nor in d31
// 	d12 = min(d12, d32); // nor in d32
// 	d11.yz = min(d11.yz,d12.xy); // nor in d12.yz
// 	d11.y = min(d11.y,d12.z); // Only two more to go
// 	d11.y = min(d11.y,d11.z); // Done! (Phew!)
// 	return sqrt(d11.xy); // F1, F2

// }
// // worley

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.viewport.z, view.viewport.w);
    let coord = vec3(uv * 2.0 - 1.0, 0.0);
    let l = length(coord) / 2.0;
    let multiply_add_coord = coord * l + (l + offset_r.x);
    let ll = l*l;
    let noise = simplexNoise3(multiply_add_coord * 10.0);
    let mixed = mix(0.0, noise, ll);
    // return vec4(mixed, mixed, mixed, 1.0);

    let gradient_angle = smoothstep(0.0,360.0, degrees(atan2(coord.y, coord.x)));
    // return vec4(gradient_angle, 0.0, 0.0, 1.0);
    let tex = textureSample(texture, our_sampler, vec2(gradient_angle * 2.0 - 1.0, multiply_add_coord.y));
    return vec4(tex.xyz,   1.0);
}