use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::system::{
        lifetimeless::{Read, SRes},
        SystemParamItem,
    },
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey,
        MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::{
            ExtractComponent, ExtractComponentPlugin,
        },
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions,
            EntityRenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline,
            TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        view::ExtractedView,
        RenderApp, RenderStage,
    },
};

use crate::{fog::FogMeta, time::TimeMeta};

#[derive(
    AsBindGroup, TypeUuid, Debug, Clone, Component,
)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct VolumetricMaterial {
    #[texture(0, dimension = "3d")]
    #[sampler(1)]
    pub fog: Handle<Image>,
}

pub struct VolumetricMaterialPlugin;

// the VolumetricMaterialPlugin depends on the TimePlugin
impl Plugin for VolumetricMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<
            VolumetricMaterial,
        >::default());

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawVolume>()
            .init_resource::<VolumetricPipeline>()
            .init_resource::<SpecializedMeshPipelines<VolumetricPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_volume)
            .add_system_to_stage(RenderStage::Queue, queue_fog_bind_group)
            .add_system_to_stage(RenderStage::Queue, queue_time_bind_group);
    }
}

impl ExtractComponent for VolumetricMaterial {
    type Query = Read<VolumetricMaterial>;

    type Filter = ();

    fn extract_component(
        query: bevy::ecs::query::QueryItem<Self::Query>,
    ) -> Self {
        VolumetricMaterial {
            fog: query.fog.clone(),
        }
    }
}

// add each entity with a mesh and a
// `VolumetricMaterial` to every view's
// `Transparent3d` render phase using the
// `VolumetricPipeline`
#[allow(clippy::too_many_arguments)]
fn queue_volume(
    transparent_3d_draw_functions: Res<
        DrawFunctions<Transparent3d>,
    >,
    custom_pipeline: Res<VolumetricPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<
        SpecializedMeshPipelines<VolumetricPipeline>,
    >,
    mut pipeline_cache: ResMut<PipelineCache>,
    render_meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<
        (Entity, &MeshUniform, &Handle<Mesh>),
        With<VolumetricMaterial>,
    >,
    mut views: Query<(
        &ExtractedView,
        &mut RenderPhase<Transparent3d>,
    )>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawVolume>()
        .unwrap();

    let key =
        MeshPipelineKey::from_msaa_samples(msaa.samples)
            | MeshPipelineKey::from_primitive_topology(
                PrimitiveTopology::TriangleList,
            );

    for (view, mut transparent_phase) in &mut views {
        let rangefinder = view.rangefinder3d();
        for (entity, mesh_uniform, mesh_handle) in
            &material_meshes
        {
            if let Some(mesh) =
                render_meshes.get(mesh_handle)
            {
                let pipeline = pipelines
                    .specialize(
                        &mut pipeline_cache,
                        &custom_pipeline,
                        key,
                        &mesh.layout,
                    )
                    .unwrap();
                transparent_phase.add(Transparent3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: rangefinder
                        .distance(&mesh_uniform.transform),
                });
            }
        }
    }
}

// create a bind group for the time uniform buffer
fn queue_time_bind_group(
    render_device: Res<RenderDevice>,
    mut time_meta: ResMut<TimeMeta>,
    pipeline: Res<VolumetricPipeline>,
) {
    let bind_group = render_device.create_bind_group(
        &BindGroupDescriptor {
            label: None,
            layout: &pipeline.time_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: time_meta
                    .buffer
                    .as_entire_binding(),
            }],
        },
    );
    time_meta.bind_group = Some(bind_group);
}

fn queue_fog_bind_group(
    render_device: Res<RenderDevice>,
    mut fog_meta: ResMut<FogMeta>,
    pipeline: Res<VolumetricPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
) {
    let gpu_image =
        gpu_images.get(&fog_meta.image).unwrap();
    let bind_group = render_device.create_bind_group(
        &BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &gpu_image.texture_view,
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(
                        &gpu_image.sampler,
                    ),
                },
            ],
            label: Some("fog_image_bind_group"),
            layout: &pipeline.fog_image_layout,
        },
    );

    fog_meta.bind_group = Some(bind_group);
}
pub struct VolumetricPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    time_bind_group_layout: BindGroupLayout,
    fog_image_layout: BindGroupLayout,
}

impl FromWorld for VolumetricPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server
            .load("shaders/volumetric_material.wgsl");
        // let shader = asset_server
        //     .load("shaders/animate_shader.wgsl");

        let render_device =
            world.resource::<RenderDevice>();
        let time_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("time bind group"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                    },
                    count: None,
                }],
            });

        let fog_image_layout = render_device
            .create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            sample_type:
                                TextureSampleType::Float {
                                    filterable: true,
                                },
                            view_dimension:
                                TextureViewDimension::D3,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(
                            SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
                label: Some("fog image layout"),
            },
        );

        let mesh_pipeline =
            world.resource::<MeshPipeline>();

        VolumetricPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            time_bind_group_layout,
            fog_image_layout,
        }
    }
}

impl SpecializedMeshPipeline for VolumetricPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<
        RenderPipelineDescriptor,
        SpecializedMeshPipelineError,
    > {
        let mut descriptor =
            self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader =
            self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.time_bind_group_layout.clone(),
            self.fog_image_layout.clone(),
        ]);
        Ok(descriptor)
    }
}

// a type alias that specifies the draw function
// including the bind group ids for each bind
// group
type DrawVolume = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetTimeBindGroup<2>,
    SetFogBindGroup<3>,
    DrawMesh,
);

struct SetTimeBindGroup<const I: usize>;

impl<const I: usize> EntityRenderCommand
    for SetTimeBindGroup<I>
{
    type Param = SRes<TimeMeta>;

    fn render<'w>(
        _view: Entity,
        _item: Entity,
        time_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let time_bind_group = time_meta
            .into_inner()
            .bind_group
            .as_ref()
            .unwrap();
        pass.set_bind_group(I, time_bind_group, &[]);

        RenderCommandResult::Success
    }
}

struct SetFogBindGroup<const I: usize>;

impl<const I: usize> EntityRenderCommand
    for SetFogBindGroup<I>
{
    type Param = SRes<FogMeta>;

    fn render<'w>(
        _view: Entity,
        _item: Entity,
        fog_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let fog_bind_group = fog_meta
            .into_inner()
            .bind_group
            .as_ref()
            .unwrap();
        pass.set_bind_group(I, fog_bind_group, &[]);

        RenderCommandResult::Success
    }
}
