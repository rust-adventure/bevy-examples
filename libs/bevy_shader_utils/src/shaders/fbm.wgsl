//  MIT License. © Inigo Quilez, Munrocket
//  noise_2d() is any noise here: Value, Perlin, Simplex, Worley
//
var m_two: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0.8, 0.6), vec2<f32>(-0.6, 0.8));
fn fbm(p: vec2<f32>, noise_2d: ) -> f32 {
  var f: f32 = 0.;
  f = f + 0.5000 * noise_2d(p); p = m_two * p * 2.02;
  f = f + 0.2500 * noise_2d(p); p = m_two * p * 2.03;
  f = f + 0.1250 * noise_2d(p); p = m_two * p * 2.01;
  f = f + 0.0625 * noise_2d(p);
  return f / 0.9375;
}