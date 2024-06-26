use bevy::asset::Handle;
use bevy::math::Vec4;
use bevy::pbr::{
    AlphaMode, Material, MaterialPipeline,
    MaterialPipelineKey,
};
use bevy::reflect::TypeUuid;
use bevy::render::{
    color::Color, mesh::MeshVertexBufferLayout,
    render_asset::RenderAssets, render_resource::*,
    texture::Image,
};

#[derive(
    encase::ShaderType, Default, Debug, Clone, Copy,
)]
pub struct Stripe {
    pub frequency: f32,
    pub minimum_value: f32,
    pub power_value: f32,
    pub should_use: f32,
}

impl From<Stripe> for [f32; 4] {
    fn from(stripe: Stripe) -> Self {
        [
            stripe.frequency,
            stripe.minimum_value,
            stripe.power_value,
            stripe.should_use,
        ]
    }
}
/// A material with "standard" properties used in PBR lighting
/// Standard property values with pictures here
/// <https://google.github.io/filament/Material%20Properties.pdf>.
///
/// May be created directly from a [`Color`] or an [`Image`].
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "7424888b-c082-457b-abcf-517228cc0c22"]
#[bind_group_data(StandardMaterialKey)]
#[uniform(0, StandardMaterialUniform)]
pub struct StandardMaterial {
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything
    /// in between. If used together with a base_color_texture, this is factored into the final
    /// base color as `base_color * base_color_texture_value`
    pub base_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Color,
    #[texture(3)]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,
    /// Linear perceptual roughness, clamped to [0.089, 1.0] in the shader
    /// Defaults to minimum of 0.089
    /// If used together with a roughness/metallic texture, this is factored into the final base
    /// color as `roughness * roughness_texture_value`
    pub perceptual_roughness: f32,
    /// From [0.0, 1.0], dielectric to pure metallic
    /// If used together with a roughness/metallic texture, this is factored into the final base
    /// color as `metallic * metallic_texture_value`
    pub metallic: f32,
    #[texture(5)]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,
    /// Specular intensity for non-metals on a linear scale of [0.0, 1.0]
    /// defaults to 0.5 which is mapped to 4% reflectance in the shader
    pub reflectance: f32,
    #[texture(9)]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,
    /// Normal map textures authored for DirectX have their y-component flipped. Set this to flip
    /// it to right-handed conventions.
    pub flip_normal_map_y: bool,
    #[texture(7)]
    #[sampler(8)]
    pub occlusion_texture: Option<Handle<Image>>,
    /// Support two-sided lighting by automatically flipping the normals for "back" faces
    /// within the PBR lighting shader.
    /// Defaults to false.
    /// This does not automatically configure backface culling, which can be done via
    /// `cull_mode`.
    pub double_sided: bool,
    /// Whether to cull the "front", "back" or neither side of a mesh
    /// defaults to `Face::Back`
    pub cull_mode: Option<Face>,
    pub unlit: bool,
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,
    #[uniform(12)]
    pub time: f32,
    #[uniform(13)]
    pub stripe_one: Stripe,
    #[uniform(14)]
    pub stripe_two: Stripe,
    #[uniform(15)]
    pub stripe_three: Stripe,
    #[uniform(16)]
    pub stripe_four: Stripe,
    #[uniform(17)]
    pub stripe_five: Stripe,
    pub stripe_color_one: Color,
    pub stripe_color_two: Color,
    pub stripe_color_three: Color,
    pub stripe_color_four: Color,
    pub stripe_color_five: Color,
}

impl Default for StandardMaterial {
    fn default() -> Self {
        StandardMaterial {
            stripe_one: Stripe {
                frequency: 0.0,
                minimum_value: 0.0,
                power_value: 0.0,
                should_use: 0.0,
            },
            stripe_two: Stripe {
                frequency: 0.0,
                minimum_value: 0.0,
                power_value: 0.0,
                should_use: 0.0,
            },
            stripe_three: Stripe {
                frequency: 0.0,
                minimum_value: 0.0,
                power_value: 0.0,
                should_use: 0.0,
            },
            stripe_four: Stripe {
                frequency: 0.0,
                minimum_value: 0.0,
                power_value: 0.0,
                should_use: 0.0,
            },
            stripe_five: Stripe {
                frequency: 0.0,
                minimum_value: 0.0,
                power_value: 0.0,
                should_use: 0.0,
            },
            base_color: Color::rgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            // This is the minimum the roughness is clamped to in shader code
            // See <https://google.github.io/filament/Filament.html#materialsystem/parameterization/>
            // It's the minimum floating point value that won't be rounded down to 0 in the
            // calculations used. Although technically for 32-bit floats, 0.045 could be
            // used.
            perceptual_roughness: 0.089,
            // Few materials are purely dielectric or metallic
            // This is just a default for mostly-dielectric
            metallic: 0.01,
            metallic_roughness_texture: None,
            // Minimum real-world reflectance is 2%, most materials between 2-5%
            // Expressed in a linear scale and equivalent to 4% reflectance see
            // <https://google.github.io/filament/Material%20Properties.pdf>
            reflectance: 0.5,
            occlusion_texture: None,
            normal_map_texture: None,
            flip_normal_map_y: false,
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
            time: 0.,
            stripe_color_one: Color::RED,
            stripe_color_two: Color::RED,
            stripe_color_three: Color::RED,
            stripe_color_four: Color::RED,
            stripe_color_five: Color::RED,
        }
    }
}

impl From<Color> for StandardMaterial {
    fn from(color: Color) -> Self {
        StandardMaterial {
            base_color: color,
            alpha_mode: if color.a() < 1.0 {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for StandardMaterial {
    fn from(texture: Handle<Image>) -> Self {
        StandardMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }
    }
}

// NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct StandardMaterialFlags: u32 {
        const BASE_COLOR_TEXTURE         = (1 << 0);
        const EMISSIVE_TEXTURE           = (1 << 1);
        const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
        const OCCLUSION_TEXTURE          = (1 << 3);
        const DOUBLE_SIDED               = (1 << 4);
        const UNLIT                      = (1 << 5);
        const ALPHA_MODE_OPAQUE          = (1 << 6);
        const ALPHA_MODE_MASK            = (1 << 7);
        const ALPHA_MODE_BLEND           = (1 << 8);
        const TWO_COMPONENT_NORMAL_MAP   = (1 << 9);
        const FLIP_NORMAL_MAP_Y          = (1 << 10);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`StandardMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct StandardMaterialUniform {
    pub stripe_one: Stripe,
    pub stripe_two: Stripe,
    pub stripe_three: Stripe,
    pub stripe_four: Stripe,
    pub stripe_five: Stripe,
    pub stripe_color_one: Vec4,
    pub stripe_color_two: Vec4,
    pub stripe_color_three: Vec4,
    pub stripe_color_four: Vec4,
    pub stripe_color_five: Vec4,
    pub time: f32,
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything
    /// in between.
    pub base_color: Vec4,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Vec4,
    /// Linear perceptual roughness, clamped to [0.089, 1.0] in the shader
    /// Defaults to minimum of 0.089
    pub roughness: f32,
    /// From [0.0, 1.0], dielectric to pure metallic
    pub metallic: f32,
    /// Specular intensity for non-metals on a linear scale of [0.0, 1.0]
    /// defaults to 0.5 which is mapped to 4% reflectance in the shader
    pub reflectance: f32,
    pub flags: u32,
    /// When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque,
    /// and any below means fully transparent.
    pub alpha_cutoff: f32,
}

impl AsBindGroupShaderType<StandardMaterialUniform>
    for StandardMaterial
{
    fn as_bind_group_shader_type(
        &self,
        images: &RenderAssets<Image>,
    ) -> StandardMaterialUniform {
        let mut flags = StandardMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |=
                StandardMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |=
                StandardMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if self.occlusion_texture.is_some() {
            flags |=
                StandardMaterialFlags::OCCLUSION_TEXTURE;
        }
        if self.double_sided {
            flags |= StandardMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= StandardMaterialFlags::UNLIT;
        }
        let has_normal_map =
            self.normal_map_texture.is_some();
        if has_normal_map {
            if let Some(texture) = images.get(
                self.normal_map_texture.as_ref().unwrap(),
            ) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }
        // NOTE: 0.5 is from the glTF default - do we want this?
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => {
                flags |=
                    StandardMaterialFlags::ALPHA_MODE_OPAQUE
            }
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |=
                    StandardMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => {
                flags |=
                    StandardMaterialFlags::ALPHA_MODE_BLEND
            }
        };

        StandardMaterialUniform {
            time: self.time,
            stripe_one: self.stripe_one,
            stripe_two: self.stripe_two,
            stripe_three: self.stripe_three,
            stripe_four: self.stripe_four,
            stripe_five: self.stripe_five,
            stripe_color_one: self
                .stripe_color_one
                .as_linear_rgba_f32()
                .into(),
            stripe_color_two: self
                .stripe_color_two
                .as_linear_rgba_f32()
                .into(),
            stripe_color_three: self
                .stripe_color_three
                .as_linear_rgba_f32()
                .into(),
            stripe_color_four: self
                .stripe_color_four
                .as_linear_rgba_f32()
                .into(),
            stripe_color_five: self
                .stripe_color_five
                .as_linear_rgba_f32()
                .into(),
            base_color: self
                .base_color
                .as_linear_rgba_f32()
                .into(),
            emissive: self.emissive.into(),
            roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            flags: flags.bits(),
            alpha_cutoff,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StandardMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
}

impl From<&StandardMaterial> for StandardMaterialKey {
    fn from(material: &StandardMaterial) -> Self {
        StandardMaterialKey {
            normal_map: material
                .normal_map_texture
                .is_some(),
            cull_mode: material.cull_mode,
        }
    }
}

impl Material for StandardMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if key.bind_group_data.normal_map {
            descriptor
                .fragment
                .as_mut()
                .unwrap()
                .shader_defs
                .push(String::from(
                    "STANDARDMATERIAL_NORMAL_MAP",
                ));
        }
        descriptor.primitive.cull_mode =
            key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        Ok(())
    }
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/vertex_shader.wgsl".into()
    // }
    fn fragment_shader() -> ShaderRef {
        "shaders/standard_extension.wgsl".into()
    }
    // fn fragment_shader() -> ShaderRef {
    //     PBR_SHADER_HANDLE.typed().into()
    // }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}
