use bevy::{
    core_pipeline::core_3d::{
        AlphaMask3d, Opaque3d, Transparent3d,
    },
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey,
        MeshUniform, SetMaterialBindGroup,
        SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent,
            ExtractComponentPlugin, UniformComponentPlugin,
        },
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        mesh::MeshVertexBufferLayout,
        render_asset::{PrepareAssetLabel, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions,
            EntityRenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline,
            TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        texture::FallbackImage,
        view::{ExtractedView, VisibleEntities},
        Extract, RenderApp, RenderStage,
    },
    utils::{HashMap, HashSet},
};

use crate::time::TimeMeta;

#[derive(
    AsBindGroup, TypeUuid, Debug, Clone, Component,
)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
// #[derive(Component, Clone)]
pub struct VolumetricMaterial {
    #[texture(0, dimension = "3d")]
    #[sampler(1)]
    pub fog: Handle<Image>,
}

pub struct VolumetricMaterialPlugin;

// the VolumetricMaterialPlugin depends on the TimePlugin
impl Plugin for VolumetricMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<VolumetricMaterial>().add_plugin(
            ExtractComponentPlugin::<
                Handle<VolumetricMaterial>,
            >::extract_visible(),
        );
        app.sub_app_mut(RenderApp)
        //         .add_plugin(UniformComponentPlugin::<
        //     VolumetricMaterial,
        // >::default())
            .add_render_command::<Transparent3d, DrawVolume>()
            .init_resource::<VolumetricPipeline>()
            .init_resource::<ExtractedMaterials>()
            .init_resource::<RenderMaterials>()
            .init_resource::<SpecializedMeshPipelines<VolumetricPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_materials)
            .add_system_to_stage(
                RenderStage::Prepare,
                prepare_materials.after(PrepareAssetLabel::PreAssetPrepare),
            )
            .add_system_to_stage(RenderStage::Queue, queue_volume)
            .add_system_to_stage(RenderStage::Queue, queue_time_bind_group);
        // .add_system_to_stage(RenderStage::Queue, queue_material_meshes)
        //    ;
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

pub struct VolumetricPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    time_bind_group_layout: BindGroupLayout,
    material_bind_group_layout: BindGroupLayout,
}

impl FromWorld for VolumetricPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        // let shader = asset_server
        //     .load("shaders/volumetric_material.wgsl");
        let shader = asset_server
            .load("shaders/animate_shader.wgsl");

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

        let mesh_pipeline =
            world.resource::<MeshPipeline>();

        VolumetricPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            time_bind_group_layout,
            material_bind_group_layout:
                VolumetricMaterial::bind_group_layout(
                    render_device,
                ),
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
            self.material_bind_group_layout.clone(),
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
    SetVolumetricBindGroup<3>,
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

// #[derive(Resource)]
// pub struct VolumetricBindGroup {
//     pub bind_group: BindGroup,
// }

// pub fn queue_mesh_bind_group(
//     mut commands: Commands,
//     mesh_pipeline: Res<MeshPipeline>,
//     render_device: Res<RenderDevice>,
//     mesh_uniforms: Res<ComponentUniforms<MeshUniform>>,
// ) {
//     if let Some(mesh_binding) =
//         mesh_uniforms.uniforms().binding()
//     {
//         let mut volumetric_bind_group =
//             VolumetricBindGroup {
//                 bind_group: render_device
//                     .create_bind_group(
//                         &BindGroupDescriptor {
//                             entries: &[BindGroupEntry {
//                                 binding: 0,
//                                 resource: mesh_binding
//                                     .clone(),
//                             }],
//                             label: Some(
//                                 "volumetric_bind_group",
//                             ),
//                             layout: &mesh_pipeline
//                                 .mesh_layout,
//                         },
//                     ),
//             };

//         commands.insert_resource(volumetric_bind_group);
//     }
// }

pub struct SetVolumetricBindGroup<const I: usize>;

impl<const I: usize> EntityRenderCommand
    for SetVolumetricBindGroup<I>
{
    type Param = (
        SRes<RenderMaterials>,
        SQuery<Read<Handle<VolumetricMaterial>>>,
    );

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (materials, query): SystemParamItem<
            'w,
            '_,
            Self::Param,
        >,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let material_handle = query.get(item).unwrap();
        let material = materials
            .into_inner()
            .get(material_handle)
            .unwrap();
        pass.set_bind_group(I, &material.bind_group, &[]);
        RenderCommandResult::Success
    }
}

struct ExtractedMaterials {
    extracted: Vec<(
        Handle<VolumetricMaterial>,
        VolumetricMaterial,
    )>,
    removed: Vec<Handle<VolumetricMaterial>>,
}

impl Default for ExtractedMaterials {
    fn default() -> Self {
        Self {
            extracted: Default::default(),
            removed: Default::default(),
        }
    }
}

/// Stores all prepared representations of [`Material`] assets for as long as they exist.
#[derive(Deref, DerefMut)]
pub struct RenderMaterials(
    pub  HashMap<
        Handle<VolumetricMaterial>,
        PreparedVolumetricMaterial,
    >,
);

impl Default for RenderMaterials {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct PreparedVolumetricMaterial {
    pub bindings: Vec<OwnedBindingResource>,
    pub bind_group: BindGroup,
}

/// All [`Material`] values of a given type that should be prepared next frame.
pub struct PrepareNextFrameMaterials {
    assets: Vec<(
        Handle<VolumetricMaterial>,
        VolumetricMaterial,
    )>,
}

impl Default for PrepareNextFrameMaterials {
    fn default() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}
/// This system prepares all assets of the corresponding [`Material`] type
/// which where extracted this frame for the GPU.
fn prepare_materials(
    mut prepare_next_frame: Local<
        PrepareNextFrameMaterials,
    >,
    mut extracted_assets: ResMut<ExtractedMaterials>,
    mut render_materials: ResMut<RenderMaterials>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
    pipeline: Res<VolumetricPipeline>,
) {
    let mut queued_assets =
        std::mem::take(&mut prepare_next_frame.assets);
    for (handle, material) in queued_assets.drain(..) {
        match prepare_volumetric_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials
                    .insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame
                    .assets
                    .push((handle, material));
            }
        }
    }

    for removed in
        std::mem::take(&mut extracted_assets.removed)
    {
        render_materials.remove(&removed);
    }

    for (handle, material) in
        std::mem::take(&mut extracted_assets.extracted)
    {
        match prepare_volumetric_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials
                    .insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame
                    .assets
                    .push((handle, material));
            }
        }
    }
}

fn prepare_volumetric_material(
    material: &VolumetricMaterial,
    render_device: &RenderDevice,
    images: &RenderAssets<Image>,
    fallback_image: &FallbackImage,
    pipeline: &VolumetricPipeline,
) -> Result<PreparedVolumetricMaterial, AsBindGroupError> {
    let prepared = material.as_bind_group(
        &pipeline.material_bind_group_layout,
        render_device,
        images,
        fallback_image,
    )?;

    Ok(PreparedVolumetricMaterial {
        bindings: prepared.bindings,
        bind_group: prepared.bind_group,
    })
}

// create a bind group for the volume materials buffer
// fn queue_volume_bind_group(
//     render_device: Res<RenderDevice>,
//     mut prepared_volumes: Query<&PreparedVolumes>,
//     pipeline: Res<VolumetricPipeline>,
// ) {
//     let bind_group = render_device.create_bind_group(
//         &BindGroupDescriptor {
//             label: None,
//             layout: &pipeline.material_bind_group_layout,
//             entries: &[BindGroupEntry {
//                 binding: 0,
//                 resource: time_meta
//                     .buffer
//                     .as_entire_binding(),
//             }],
//         },
//     );
//     time_meta.bind_group = Some(bind_group);
// }

fn extract_materials(
    mut commands: Commands,
    mut events: Extract<
        EventReader<AssetEvent<VolumetricMaterial>>,
    >,
    assets: Extract<Res<Assets<VolumetricMaterial>>>,
) {
    let mut changed_assets = HashSet::default();
    let mut removed = Vec::new();
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle }
            | AssetEvent::Modified { handle } => {
                changed_assets.insert(handle.clone_weak());
            }
            AssetEvent::Removed { handle } => {
                changed_assets.remove(handle);
                removed.push(handle.clone_weak());
            }
        }
    }

    let mut extracted_assets = Vec::new();
    for handle in changed_assets.drain() {
        if let Some(asset) = assets.get(&handle) {
            extracted_assets.push((handle, asset.clone()));
        }
    }

    commands.insert_resource(ExtractedMaterials {
        extracted: extracted_assets,
        removed,
    });
}
