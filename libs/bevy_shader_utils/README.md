# Bevy Shader Utils

A utility package that provides a series of noise functions and other utilities for use in wgpu shaders.

## Quick Start

Use the import at the top of your wgsl file and Bevy takes care of the rest.

```wgsl
#import bevy_pbr::forward_io::VertexOutput

#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d

struct Material {
    scale: f32
};

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let f: f32 = simplex_noise_3d(material.scale * mesh.world_position.xyz);

    let color_a = vec3(0.282, 0.51, 1.0);
    let color_b = vec3(0.725, 0.816, 0.698);
    let mixed = mix(color_a, color_b, f);
    return vec4(mixed, 1.0);
}
```

The above shader is used by a material defined as such.

```rust
#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct ScreenshotSimplex3dMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for ScreenshotSimplex3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screenshot_simplex3d_material.wgsl".into()
    }
}
```

## Functions

### Perlin noise

2-dimensional:

```wgsl
#import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d

var value = perlin_noise_2d(vec2<f32>(5.0, 6.0))
```

3-dimensional:

```wgsl
#import bevy_shader_utils::perlin_noise_3d::perlin_noise_3d

var value = perlin_noise_3d(vec3<f32>(5.0, 6.0, 7.0))
```

### Simplex noise

2-dimensional:

```wgsl
#import bevy_shader_utils::simplex_noise_2d::simplex_noise_2d

var value = simplex_noise_2d(vec2<f32>(5.0, 6.0))
```

3-dimensional:

```wgsl
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d

var value = simplex_noise_3d(vec3<f32>(5.0, 6.0, 7.0))
```

### Voronoise

Voronoi and Noise: https://iquilezles.org/articles/voronoise/

```wgsl
#import bevy_shader_utils::voronoise::voronoise

var value = voronoise(vec2<f32>(5.0, 6.0), 0.0, 1.0)
```
