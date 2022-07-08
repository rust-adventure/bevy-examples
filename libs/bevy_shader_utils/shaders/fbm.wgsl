//  MIT License. © Inigo Quilez, Munrocket
//  noise2() is any noise here: Value, Perlin, Simplex, Worley
//
var m2: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0.8, 0.6), vec2<f32>(-0.6, 0.8));
fn fbm(p: vec2<f32>) -> f32 {
  var f: f32 = 0.;
  f = f + 0.5000 * noise2(p); p = m2 * p * 2.02;
  f = f + 0.2500 * noise2(p); p = m2 * p * 2.03;
  f = f + 0.1250 * noise2(p); p = m2 * p * 2.01;
  f = f + 0.0625 * noise2(p);
  return f / 0.9375;
}