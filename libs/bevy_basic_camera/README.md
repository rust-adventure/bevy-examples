# Bevy Shader Utils

A utility package that provides a series of noise functions and other utilities for use in wgpu shaders.

## Quick Start

Use the import at the top of your wgsl file and Bevy takes care of the rest.

```wgsl
#import bevy_shader_utils::perlin_noise_3d

struct CustomMaterial {
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;
[[group(1), binding(1)]]
var base_color_texture: texture_2d<f32>;
[[group(1), binding(2)]]
var base_color_sampler: sampler;

[[stage(fragment)]]
fn fragment([[location(2)]] uv: vec2<f32>) -> [[location(0)]] vec4<f32> {
    var input: vec3<f32> = vec3<f32>(uv.x * 40.0, uv.y * 40.0, 1.);
    var noise = perlinNoise3(input);
    var alpha = (noise + 1.0) / 2.0;
    return material.color
      * textureSample(base_color_texture, base_color_sampler, uv)
      * vec4<f32>(1.0,1.0,1.0,alpha);
}
```

## Functions

### Perlin noise

2-dimensional:

```wgsl
#import bevy_shader_utils::perlin_noise_2d

var value = perlinNoise2(vec2<f32>(5.0, 6.0))
```

3-dimensional:

```wgsl
#import bevy_shader_utils::perlin_noise_3d

var value = perlinNoise3(vec3<f32>(5.0, 6.0, 7.0))
```

### Simplex noise

2-dimensional:

```wgsl
#import bevy_shader_utils::simplex_noise_2d

var value = simplexNoise2(vec2<f32>(5.0, 6.0))
```

3-dimensional:

```wgsl
#import bevy_shader_utils::simplex_noise_3d

var value = simplexNoise3(vec3<f32>(5.0, 6.0, 7.0))
```

### Fractal Brownian Motion

```wgsl
#import bevy_shader_utils::fbm

var value = fbm(vec2<f32>(5.0, 6.0))
```

### Voronoise

Voronoi and Noise: https://iquilezles.org/articles/voronoise/

```wgsl
#import bevy_shader_utils::voro_noise_2d

var value = voroNoise2(vec2<f32>(5.0, 6.0), 0.0, 1.0)
```
