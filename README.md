# Bevy Examples

This repo is a selection of contained examples using the Bevy game engine.
It's split into three sub-folders:

- concepts
- examples
- libs

## Concepts

Concepts examples showing off ECS APIs and approachs like Component Hooks, Observers, Relationships, and mipmaps.

## Examples

A variety of demos, often shader-related.

### candy-cane

A shader example showing the usage of `ShaderStorageBuffer` to pass variable amounts of configuration to a material to render candy cane stripes.

![candy canes](examples/candy-cane/readme/demo.avif)

### cube-wave

![cube wave animation](examples/cube-wave/readme/demo.gif)

A custom material that uses a vertex shader to modulate the position of a series of cubes.

### dissolve-sphere-standard-material-extensions

![dissolve](examples/dissolve-sphere-standard-material-extensions/readme/demo.png)

A dissolve effect that takes advantage of a custom prepass shader.

### edge-detection-custom-phase

![edge detection](examples/edge-detection-custom-phase/readme/full-output.avif)

Outlines can be achieved in many ways, and the source for edge detection with something like a sobel filter can come from depth prepass, normal data, or anywhere else.
This demo uses a custom render phase and artist-authored vertex colors to control where outlines show up.
The custom phase renders the vertex data, using Bevy's mesh data pielines, to an image which is then used as the data source for the sobel filter in a postprocess effect.

### fresnel-effect

![fresnel](examples/fresnel-effect/readme/demo.avif)

Fresnel is an effect that is based on the viewing angle.
This demo shows off the fresnel effect combined and broken down into its components.

There is an associated video talking through the effect using [Bevy 0.8](https://www.youtube.com/watch?v=a66SysxGebo) which is fairly old, but the concepts are still relevant and the code in the repo here is up to date with Bevy's latest version.

###

## libs

Re-usable crates, some are public and some are used in the examples.

### bevy_shader_utils

![shader utils](./libs/bevy_shader_utils/readme/pristine_grid.avif)

A utility package that provides a series of noise functions and other utilities for use in wgpu shaders.
Noise functions provided include: perlin, simplex, voronoise, and a pristine grid.

### bevy_blockout

![blockout](./libs/bevy_blockout/readme/demo.avif)

A triplanar blockout material that uses worldspace coordinates to texture meshes using a grid where 1 box equals 1 world unit

### bevy_prepass_debug

A crate used in examples to display the depth, normal, and motion vector prepass information when examples use them.
