# def process_graph [prefix: string; target: string] {
#   ktx create --format R32G32B32A32_SFLOAT $'assets/raw_assets/($prefix)_base_color.exr' $'($target)/($prefix)_base_color.ktx2'
#   ktx create --format R32G32B32A32_SFLOAT $'assets/raw_assets/($prefix)_normal_map.exr' $'($target)/($prefix)_normal_map.ktx2'
#   ktx create --format R32G32B32A32_SFLOAT $'assets/raw_assets/($prefix)_metallic_roughness.exr' $'($target)/($prefix)_metallic_roughness.ktx2'
#   oiiotool $'assets/raw_assets/($prefix)_depth_map.exr' -chnames R -o $'($target)/($prefix)_intermediate_depth_map.exr'
#   ktx create --format R32_SFLOAT $'($target)/($prefix)_intermediate_depth_map.exr' $'($target)/($prefix)_depth_map.ktx2'
# }

# # convert the "processed_single" images into the "processed" directory
# # process_graph "floor_graph" "assets/processed-single"

# ## layers 3

# def process_graph_to_texture_array [target: string] {
#   ktx create --format R32G32B32A32_SFLOAT --layers 3 assets/raw_assets/floor_graph_base_color.exr assets/raw_assets/grass_graph_base_color.exr assets/raw_assets/stone_graph_base_color.exr $'($target)/array_base_color.ktx2'
#   ktx create --format R32G32B32A32_SFLOAT --layers 3 assets/raw_assets/floor_graph_normal_map.exr assets/raw_assets/grass_graph_normal_map.exr assets/raw_assets/stone_graph_normal_map.exr $'($target)/array_normal_map.ktx2'
#   ktx create --format R32G32B32A32_SFLOAT --layers 3 assets/raw_assets/floor_graph_metallic_roughness.exr assets/raw_assets/grass_graph_metallic_roughness.exr assets/raw_assets/stone_graph_metallic_roughness.exr $'($target)/array_metallic_roughness.ktx2'
#   oiiotool $'assets/raw_assets/floor_graph_depth_map.exr' -chnames R -o $'($target)/floor_graph_intermediate_depth_map.exr'
#   oiiotool $'assets/raw_assets/grass_graph_depth_map.exr' -chnames R -o $'($target)/grass_graph_intermediate_depth_map.exr'
#   oiiotool $'assets/raw_assets/stone_graph_depth_map.exr' -chnames R -o $'($target)/stone_graph_intermediate_depth_map.exr'
#   ktx create --format R32G32B32A32_SFLOAT --layers 3 $'($target)/floor_graph_intermediate_depth_map.exr' $'($target)/grass_graph_intermediate_depth_map.exr' $'($target)/stone_graph_intermediate_depth_map.exr' $'($target)/array_depth_map.ktx2'
# }

# process_graph_to_texture_array "assets/processed_array"


## Size tests

# def process_graph_to_texture_array [target: string] {
# #   ktx create --format R32G32B32A32_SFLOAT --layers 3 --encode basis-lz assets/raw_assets/floor_graph_base_color.exr assets/raw_assets/grass_graph_base_color.exr assets/raw_assets/stone_graph_base_color.exr $'($target)/array_base_color.ktx2'
# #   ktx create --format R32G32B32A32_SFLOAT --layers 3 --normal-mode --normalize assets/raw_assets/floor_graph_normal_map.exr assets/raw_assets/grass_graph_normal_map.exr assets/raw_assets/stone_graph_normal_map.exr $'($target)/array_normal_map.ktx2'
# #   ktx create --format R32G32B32A32_SFLOAT --layers 3 assets/raw_assets/floor_graph_metallic_roughness.exr assets/raw_assets/grass_graph_metallic_roughness.exr assets/raw_assets/stone_graph_metallic_roughness.exr $'($target)/array_metallic_roughness.ktx2'
# #   oiiotool $'assets/raw_assets/floor_graph_depth_map.exr' -chnames R -o $'($target)/floor_graph_intermediate_depth_map.exr'
# #   oiiotool $'assets/raw_assets/grass_graph_depth_map.exr' -chnames R -o $'($target)/grass_graph_intermediate_depth_map.exr'
# #   oiiotool $'assets/raw_assets/stone_graph_depth_map.exr' -chnames R -o $'($target)/stone_graph_intermediate_depth_map.exr'
#   ktx create --format R8_UNORM --encode basis-lz --layers 3 $'($target)/floor_graph_intermediate_depth_map.exr' $'($target)/grass_graph_intermediate_depth_map.exr' $'($target)/stone_graph_intermediate_depth_map.exr' $'($target)/array_depth_map.ktx2'
# }

# process_graph_to_texture_array "assets/processed_array_compressed"


## basisu tests

### Must specify -linear for normal_map, depth_map, and other non-srgb data
#### -tex_array

#### Choose between etc1s and uastc
#### zstd compression is default
#### -output_path/file

#### -separate_rg_to_color_alpha (normal maps)

#### -mipmap

def process_graph_to_texture_array [target: string] {
    basisu -tex_array -basis -mipmap -output_path $target assets/raw_assets/floor_graph_base_color.exr assets/raw_assets/grass_graph_base_color.exr assets/raw_assets/stone_graph_base_color.exr
    # basisu -tex_array -basis -normal_map -output_path $target assets/raw_assets/floor_graph_normal_map.png assets/raw_assets/grass_graph_normal_map.png assets/raw_assets/stone_graph_normal_map.png
    # # other valid normal map flags:
    # # -mip_renorm
    # # -separate_rg_to_color_alpha
    # basisu -tex_array -basis -linear -output_path $target assets/raw_assets/floor_graph_metallic_roughness.png assets/raw_assets/grass_graph_metallic_roughness.png assets/raw_assets/stone_graph_metallic_roughness.png
    # # force -uastc for depth_maps
    # basisu -tex_array -basis -linear -output_path $target assets/raw_assets/floor_graph_depth_map.png assets/raw_assets/grass_graph_depth_map.png assets/raw_assets/stone_graph_depth_map.png
}

process_graph_to_texture_array "assets/processed_array_basisu/exr/"

## kram tests

# alias kram = ~/resources/kram-macos/kram

# def process_graph_to_texture_array [target: string] {
#     ktx create --format R8G8B8A8_SRGB --assign-tf srgb --layers 3 assets/raw_assets/floor_graph_base_color.png assets/raw_assets/grass_graph_base_color.png assets/raw_assets/stone_graph_base_color.png $'($target)/uncompressed_base_color.ktx2'
#     ~/resources/kram-macos/kram encode -f bc7 -type 2darray -srgb -zstd 0 -o $'($target)/base_color.ktx2' -i $'($target)/uncompressed_base_color.ktx2'

#     ktx create --format R8G8B8_UNORM --assign-tf linear --layers 3 assets/raw_assets/floor_graph_normal_map.png assets/raw_assets/grass_graph_normal_map.png assets/raw_assets/stone_graph_normal_map.png $'($target)/uncompressed_normal_map.ktx2'
#     ~/resources/kram-macos/kram encode -f bc5 -type 2darray -normal -o $'($target)/normal_map.ktx2' -i $'($target)/uncompressed_normal_map.ktx2'

#     ktx create --format R8_UNORM --assign-tf linear --layers 3 assets/raw_assets/floor_graph_depth_map.png assets/raw_assets/grass_graph_depth_map.png assets/raw_assets/stone_graph_depth_map.png $'($target)/uncompressed_depth_map.ktx2'
#     ~/resources/kram-macos/kram encode -f bc4 -type 2darray -srclin  -o $'($target)/depth_map.ktx2' -i $'($target)/uncompressed_depth_map.ktx2'
# }

# process_graph_to_texture_array "assets/processed_array_kram"


# def process_graph [prefix: string; target: string] {
# #   ~/resources/kram-macos/kram encode -f bc7 -type 2d -srgb   -zstd 0 -o $'($target)/base_color.ktx2' -i $'assets/raw_assets/($prefix)_base_color.png' 
# #   ~/resources/kram-macos/kram encode -f bc5 -type 2d -normal -zstd 0 -o $'($target)/normal_map.ktx2' -i $'assets/raw_assets/($prefix)_normal_map.png'
# #   ~/resources/kram-macos/kram encode -f bc4 -type 2d -srclin -zstd 0 -o $'($target)/depth_map.ktx2'  -i $'assets/raw_assets/($prefix)_depth_map.png'
# }

# # convert the "processed_single" images into the "processed" directory
# process_graph "floor_graph" "assets/processed-single"