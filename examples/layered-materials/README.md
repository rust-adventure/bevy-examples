These commands are run from the root of the example, next to the Cargo.toml and assets/ directory.

## Converting one color .exr to ktx

converting srgb data to ktx2, like base_color, requires a format specification

```
ktx create --format R32G32B32A32_SFLOAT raw_assets/floor_graph_base_color.exr processed/floor_base_color.ktx2
```

## Converting depth_map from substance designer

substance designer depth/height output to .exr contains a channel called Y, so we rename it to channel R

```
oiiotool raw_assets/floor_graph_depth_map.exr -chnames R -o out_depth_map_test.exr
```

Then the depth map can be converted to ktx:

```
ktx create --format R32_SFLOAT raw_assets/out_depth_map_test.exr processed/floor_depth_map.ktx2
```

## Take all of a graph's outputs and convert to ktx

```
ktx create --format R32G32B32A32_SFLOAT raw_assets/floor_graph_base_color.exr processed/floor_base_color.ktx2
ktx create --format R32G32B32A32_SFLOAT raw_assets/floor_graph_normal_map.exr processed/floor_graph_normal_map.ktx2
ktx create --format R32G32B32A32_SFLOAT raw_assets/floor_graph_metallic_roughness.exr processed/floor_graph_metallic_roughness.ktx2
oiiotool raw_assets/floor_graph_depth_map.exr -chnames R -o out_depth_map_test.exr
ktx create --format R32_SFLOAT raw_assets/out_depth_map_test.exr processed/floor_depth_map.ktx2
```
