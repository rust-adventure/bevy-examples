use bevy::{
    core_pipeline::core_3d::{
        CORE_3D_DEPTH_FORMAT,
        graph::{Core3d, Node3d},
    },
    ecs::{
        query::QueryItem,
        system::{SystemParamItem, lifetimeless::SRes},
    },
    math::FloatOrd,
    pbr::{
        DrawMesh, MeshInputUniform, MeshPipeline,
        MeshPipelineKey, MeshPipelineViewLayoutKey,
        MeshUniform, RenderMeshInstances, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    platform::collections::{HashMap, HashSet},
    prelude::*,
    render::{
        Extract, Render, RenderApp, RenderDebugFlags,
        RenderSet,
        batching::{
            GetBatchData, GetFullBatchData,
            gpu_preprocessing::{
                IndirectParametersCpuMetadata,
                UntypedPhaseIndirectParametersBuffers,
                batch_and_prepare_sorted_render_phase,
            },
        },
        camera::ExtractedCamera,
        diagnostic::RecordDiagnostics,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex,
            ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        mesh::{
            MeshVertexAttribute, MeshVertexBufferLayoutRef,
            RenderMesh, allocator::MeshAllocator,
        },
        render_asset::RenderAssets,
        render_graph::{
            NodeRunError, RenderGraphApp,
            RenderGraphContext, RenderLabel, ViewNode,
            ViewNodeRunner,
        },
        render_phase::{
            AddRenderCommand,
            CachedRenderPipelinePhaseItem, DrawFunctionId,
            DrawFunctions, PhaseItem, PhaseItemExtraIndex,
            RenderCommand, RenderCommandResult,
            SetItemPipeline, SortedPhaseItem,
            SortedRenderPhasePlugin, TrackedRenderPass,
            ViewSortedRenderPhases, sort_phase_system,
        },
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout,
            BindGroupLayoutEntries, BindingType,
            BufferBindingType, CachedRenderPipelineId,
            ColorTargetState, ColorWrites,
            CommandEncoderDescriptor, CompareFunction,
            DepthStencilState, Extent3d, Face,
            FragmentState, FrontFace, MultisampleState,
            PipelineCache, PolygonMode, PrimitiveState,
            RenderPassDescriptor, RenderPipelineDescriptor,
            ShaderStages, ShaderType,
            SpecializedMeshPipeline,
            SpecializedMeshPipelineError,
            SpecializedMeshPipelines, StoreOp,
            TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages, VertexFormat,
            VertexState,
        },
        renderer::{RenderContext, RenderDevice},
        sync_world::{MainEntity, RenderEntity},
        texture::{ColorAttachment, TextureCache},
        view::{
            ExtractedView, RenderVisibleEntities,
            RetainedViewEntity, ViewDepthTexture,
            ViewTarget,
        },
    },
};
use nonmax::NonMaxU32;
use std::ops::Range;

pub mod post_process;

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
pub const ATTRIBUTE_SECTION_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new(
        "_SECTION_COLOR",
        923949917,
        VertexFormat::Float32x4,
    );

const SHADER_ASSET_PATH: &str = "custom_phase.wgsl";

#[derive(
    Component, ExtractComponent, Clone, Copy, Default,
)]
pub struct DrawSection;

#[derive(
    Component, ExtractComponent, Clone, Copy, ShaderType,
)]
pub struct SectionGroupId {
    pub id: u32,
}

impl FromWorld for SectionGroupId {
    fn from_world(world: &mut World) -> Self {
        let new_id = world
            .resource_mut::<SectionGroupIdGenerator>()
            .generate_id();
        Self { id: new_id }
    }
}

#[derive(Resource, Reflect, Default)]
struct SectionGroupIdGenerator(u32);

impl SectionGroupIdGenerator {
    fn generate_id(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

fn insert_section_ids(
    trigger: Trigger<OnInsert, Mesh3d>,
    mut commands: Commands,
    mut generator: ResMut<SectionGroupIdGenerator>,
    query: Query<&SectionGroupId>,
) {
    if query.get(trigger.target()).is_err() {
        commands.entity(trigger.target()).insert(
            SectionGroupId {
                id: generator.generate_id(),
            },
        );
    }
}

#[derive(Resource, Default)]
pub struct SectionDataBindGroups(Option<BindGroup>);

pub struct SectionTexturePhasePlugin;

impl Plugin for SectionTexturePhasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SectionGroupIdGenerator>()
        .add_plugins((
            ExtractComponentPlugin::<
            DrawSection,
        >::default(),
        ExtractComponentPlugin::<
            SectionGroupId,
        >::default(),
        UniformComponentPlugin::<SectionGroupId>::default(),
        SortedRenderPhasePlugin::<
        SectionTexturePhase,
        MeshPipeline,
    >::new(RenderDebugFlags::default()),
    ));

        // TODO: one day with Constructs we can register a required
        // component that implements FromWorld, not Default
        // app.register_required_components::<Mesh3d, SectionGroupId>();
        // until then, observers.
        app.add_observer(insert_section_ids);

        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };
        render_app
            .init_resource::<SectionDataBindGroups>()
            .init_resource::<SpecializedMeshPipelines<SectionTexturePipeline>>()
            .init_resource::<DrawFunctions<SectionTexturePhase>>()
            .add_render_command::<SectionTexturePhase, DrawMesh3dSectionTexture>()
            .init_resource::<ViewSortedRenderPhases<SectionTexturePhase>>()
            .add_systems(ExtractSchedule, extract_camera_phases)
            .add_systems(
                Render,
                (
                    sort_phase_system::<SectionTexturePhase>.in_set(RenderSet::PhaseSort),
                    batch_and_prepare_sorted_render_phase::<SectionTexturePhase, SectionTexturePipeline>
                        .in_set(RenderSet::PrepareResources),
                    prepare_section_data_bind_group.in_set(RenderSet::PrepareBindGroups),
                    queue_custom_meshes.in_set(RenderSet::QueueMeshes),
                    prepare_section_textures.in_set(RenderSet::PrepareResources),
                ),
            );

        render_app
            .add_render_graph_node::<ViewNodeRunner<CustomDrawNode>>(Core3d, CustomDrawPassLabel)
            // Tell the node to run after the main pass
            .add_render_graph_edges(Core3d, (Node3d::MainOpaquePass, CustomDrawPassLabel));
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };
        // The pipeline needs the RenderDevice to be created and it's only available once plugins
        // are intialized
        render_app
            .init_resource::<SectionTexturePipeline>();
    }
}

#[derive(Resource)]
pub struct SectionTexturePipeline {
    /// The base mesh pipeline defined by bevy
    ///
    /// Since we want to draw using an existing bevy mesh we want to reuse the default
    /// pipeline as much as possible
    mesh_pipeline: MeshPipeline,
    /// Stores the shader used for this pipeline directly on the pipeline.
    /// This isn't required, it's only done like this for simplicity.
    shader_handle: Handle<Shader>,
    /// Stores the bind group layout for additional component data
    /// like that weird mesh_id substitute
    section_data_layout: BindGroupLayout,
}
impl FromWorld for SectionTexturePipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh_pipeline: MeshPipeline::from_world(world),
            shader_handle: world
                .resource::<AssetServer>()
                .load(SHADER_ASSET_PATH),
            section_data_layout: world
                .resource::<RenderDevice>()
                .create_bind_group_layout(
                    "section_data_bind_group_layout",
                    &BindGroupLayoutEntries::single(
                        ShaderStages::VERTEX_FRAGMENT,
                        BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: None,
                        },
                    ),
                ),
        }
    }
}
// For more information on how SpecializedMeshPipeline work, please look at the
// specialized_mesh_pipeline example
impl SpecializedMeshPipeline for SectionTexturePipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<
        RenderPipelineDescriptor,
        SpecializedMeshPipelineError,
    > {
        let mut vertex_attributes = Vec::new();
        let mut shader_defs = Vec::new();

        // add the position
        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
            // @location(0)
            vertex_attributes.push(
                Mesh::ATTRIBUTE_POSITION
                    .at_shader_location(0),
            );
        }
        // add the section color
        if layout.0.contains(ATTRIBUTE_SECTION_COLOR) {
            // @location(1)
            shader_defs.push("SECTION_COLORS".into());
            vertex_attributes.push(
                ATTRIBUTE_SECTION_COLOR
                    .at_shader_location(1),
            );
        }
        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
        let vertex_buffer_layout =
            layout.0.get_layout(&vertex_attributes)?;

        Ok(RenderPipelineDescriptor {
            label: Some("Specialized Mesh Pipeline".into()),
            // We want to reuse the data from bevy so we use the same bind groups as the default
            // mesh pipeline
            layout: vec![
                // Bind group 0 is the view uniform
                self.mesh_pipeline
                    .get_view_layout(
                        MeshPipelineViewLayoutKey::from(
                            key,
                        ),
                    )
                    .clone(),
                // Bind group 1 is the mesh uniform
                self.mesh_pipeline
                    .mesh_layouts
                    .model_only
                    .clone(),
                // extra data
                self.section_data_layout.clone(),
            ],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: self.shader_handle.clone(),
                shader_defs: shader_defs.clone(),
                entry_point: "vertex".into(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: self.shader_handle.clone(),
                shader_defs,
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: if key
                        .contains(MeshPipelineKey::HDR)
                    {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    // format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: key.primitive_topology(),
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                ..default()
            },
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare:
                    CompareFunction::GreaterEqual,
                stencil: default(),
                bias: default(),
            }),
            // It's generally recommended to specialize your pipeline for MSAA,
            // but it's not always possible
            multisample: MultisampleState::default(),
            zero_initialize_workgroup_memory: false,
        })
    }
}

// We will reuse render commands already defined by bevy to draw a 3d mesh
type DrawMesh3dSectionTexture = (
    SetItemPipeline,
    // This will set the view bindings in group 0
    SetMeshViewBindGroup<0>,
    // This will set the mesh bindings in group 1
    SetMeshBindGroup<1>,
    // Set
    SetSectionDataBindGroup<2>,
    // This will draw the mesh
    DrawMesh,
);

// This is the data required when you define a custom phase in bevy. More specifically this is the
// data required when using a ViewSortedRenderPhase. This would look differently if we wanted a
// batched render phase. Sorted phase are a bit easier to implement, but a batched phased would
// look similar.
//
// If you want to see how a batched phase implementation looks, you should look at the Opaque2d
// phase.
struct SectionTexturePhase {
    pub sort_key: FloatOrd,
    pub entity: (Entity, MainEntity),
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>,
    pub extra_index: PhaseItemExtraIndex,
    /// Whether the mesh in question is indexed (uses an index buffer in
    /// addition to its vertex buffer).
    pub indexed: bool,
}

// For more information about writing a phase item, please look at the custom_phase_item example
impl PhaseItem for SectionTexturePhase {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity.0
    }

    #[inline]
    fn main_entity(&self) -> MainEntity {
        self.entity.1
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    #[inline]
    fn batch_range_and_extra_index_mut(
        &mut self,
    ) -> (
        &mut Range<u32>,
        &mut PhaseItemExtraIndex,
    ) {
        (
            &mut self.batch_range,
            &mut self.extra_index,
        )
    }
}

impl SortedPhaseItem for SectionTexturePhase {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        // bevy normally uses radsort instead of the std slice::sort_by_key
        // radsort is a stable radix sort that performed better than `slice::sort_by_key` or `slice::sort_unstable_by_key`.
        // Since it is not re-exported by bevy, we just use the std sort for the purpose of the example
        items.sort_by_key(SortedPhaseItem::sort_key);
    }

    #[inline]
    fn indexed(&self) -> bool {
        self.indexed
    }
}

impl CachedRenderPipelinePhaseItem for SectionTexturePhase {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

impl GetBatchData for SectionTexturePipeline {
    type Param = (
        SRes<RenderMeshInstances>,
        SRes<RenderAssets<RenderMesh>>,
        SRes<MeshAllocator>,
    );
    type CompareData = AssetId<Mesh>;
    type BufferData = MeshUniform;

    fn get_batch_data(
        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
        (_entity, main_entity): (Entity, MainEntity),
    ) -> Option<(
        Self::BufferData,
        Option<Self::CompareData>,
    )> {
        let RenderMeshInstances::CpuBuilding(
            ref mesh_instances,
        ) = **mesh_instances
        else {
            error!(
                "`get_batch_data` should never be called in GPU mesh uniform \
                building mode"
            );
            return None;
        };
        let mesh_instance =
            mesh_instances.get(&main_entity)?;
        let first_vertex_index = match mesh_allocator
            .mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        {
            Some(mesh_vertex_slice) => {
                mesh_vertex_slice.range.start
            }
            None => 0,
        };
        let mesh_uniform = {
            let mesh_transforms = &mesh_instance.transforms;
            let (
                local_from_world_transpose_a,
                local_from_world_transpose_b,
            ) = mesh_transforms
                .world_from_local
                .inverse_transpose_3x3();
            MeshUniform {
                world_from_local: mesh_transforms
                    .world_from_local
                    .to_transpose(),
                previous_world_from_local: mesh_transforms
                    .previous_world_from_local
                    .to_transpose(),
                lightmap_uv_rect: UVec2::ZERO,
                local_from_world_transpose_a,
                local_from_world_transpose_b,
                flags: mesh_transforms.flags,
                first_vertex_index,
                current_skin_index: u32::MAX,
                material_and_lightmap_bind_group_slot: 0,
                tag: 0,
                pad: 0,
            }
        };
        Some((mesh_uniform, None))
    }
}
impl GetFullBatchData for SectionTexturePipeline {
    type BufferInputData = MeshInputUniform;

    fn get_index_and_compare_data(
        (mesh_instances, _, _): &SystemParamItem<
            Self::Param,
        >,
        main_entity: MainEntity,
    ) -> Option<(NonMaxU32, Option<Self::CompareData>)>
    {
        // This should only be called during GPU building.
        let RenderMeshInstances::GpuBuilding(
            ref mesh_instances,
        ) = **mesh_instances
        else {
            error!(
                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
                    mode"
            );
            return None;
        };
        let mesh_instance =
            mesh_instances.get(&main_entity)?;
        Some((
            mesh_instance.current_uniform_index,
            mesh_instance
                .should_batch()
                .then_some(mesh_instance.mesh_asset_id),
        ))
    }

    fn get_binned_batch_data(
        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
        main_entity: MainEntity,
    ) -> Option<Self::BufferData> {
        let RenderMeshInstances::CpuBuilding(
            ref mesh_instances,
        ) = **mesh_instances
        else {
            error!(
                "`get_binned_batch_data` should never be called in GPU mesh uniform building mode"
            );
            return None;
        };
        let mesh_instance =
            mesh_instances.get(&main_entity)?;
        let first_vertex_index = match mesh_allocator
            .mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        {
            Some(mesh_vertex_slice) => {
                mesh_vertex_slice.range.start
            }
            None => 0,
        };

        Some(MeshUniform::new(
            &mesh_instance.transforms,
            first_vertex_index,
            mesh_instance.material_bindings_index.slot,
            None,
            None,
            None,
        ))
    }

    fn write_batch_indirect_parameters_metadata(
        indexed: bool,
        base_output_index: u32,
        batch_set_index: Option<NonMaxU32>,
        indirect_parameters_buffers: &mut UntypedPhaseIndirectParametersBuffers,
        indirect_parameters_offset: u32,
    ) {
        // Note that `IndirectParameters` covers both of these structures, even
        // though they actually have distinct layouts. See the comment above that
        // type for more information.
        let indirect_parameters =
            IndirectParametersCpuMetadata {
                base_output_index,
                batch_set_index: match batch_set_index {
                    None => !0,
                    Some(batch_set_index) => {
                        u32::from(batch_set_index)
                    }
                },
            };

        if indexed {
            indirect_parameters_buffers.indexed.set(
                indirect_parameters_offset,
                indirect_parameters,
            );
        } else {
            indirect_parameters_buffers.non_indexed.set(
                indirect_parameters_offset,
                indirect_parameters,
            );
        }
    }

    fn get_binned_index(
        _param: &SystemParamItem<Self::Param>,
        _query_item: MainEntity,
    ) -> Option<NonMaxU32> {
        None
    }
}

// When defining a custom phase, we need to extract it from the main world and add it to a resource
// that will be used by the render world. We need to give that resource all views that will use
// that phase
fn extract_camera_phases(
    mut commands: Commands,
    mut sections_phases: ResMut<
        ViewSortedRenderPhases<SectionTexturePhase>,
    >,
    cameras: Extract<
        Query<
            (
                RenderEntity,
                Entity,
                &Camera,
                Has<SectionsPrepass>,
            ),
            With<Camera3d>,
        >,
    >,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();
    for (
        render_entity,
        main_entity,
        camera,
        has_sections_prepass,
    ) in &cameras
    {
        if !camera.is_active {
            continue;
        }
        // This is the main camera, so we use the first subview index (0)
        let retained_view_entity = RetainedViewEntity::new(
            main_entity.into(),
            None,
            0,
        );

        if has_sections_prepass {
            sections_phases
                .insert_or_clear(retained_view_entity);
        } else {
            sections_phases.remove(&retained_view_entity);
        }
        live_entities.insert(retained_view_entity);

        commands
            .get_entity(render_entity)
            .expect("Camera entity wasn't synced.")
            .insert_if(SectionsPrepass, || {
                has_sections_prepass
            });
    }
    // Clear out all dead views.
    sections_phases.retain(|camera_entity, _| {
        live_entities.contains(camera_entity)
    });
}

// This is a very important step when writing a custom phase.
//
// This system determines which mesh will be added to the phase.
#[allow(clippy::too_many_arguments)]
fn queue_custom_meshes(
    custom_draw_functions: Res<
        DrawFunctions<SectionTexturePhase>,
    >,
    mut pipelines: ResMut<
        SpecializedMeshPipelines<SectionTexturePipeline>,
    >,
    pipeline_cache: Res<PipelineCache>,
    custom_draw_pipeline: Res<SectionTexturePipeline>,
    render_meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    mut custom_render_phases: ResMut<
        ViewSortedRenderPhases<SectionTexturePhase>,
    >,
    mut views: Query<(
        &ExtractedView,
        &RenderVisibleEntities,
        &Msaa,
    )>,
    has_marker: Query<(), With<DrawSection>>,
) {
    for (view, visible_entities, msaa) in &mut views {
        let Some(custom_phase) = custom_render_phases
            .get_mut(&view.retained_view_entity)
        else {
            continue;
        };
        let draw_custom = custom_draw_functions
            .read()
            .id::<DrawMesh3dSectionTexture>();

        // Create the key based on the view.
        // In this case we only care about MSAA and HDR
        let view_key =
            MeshPipelineKey::from_msaa_samples(
                msaa.samples(),
            ) | MeshPipelineKey::from_hdr(view.hdr);

        let rangefinder = view.rangefinder3d();

        // Since our phase can work on any 3d mesh we can reuse the default mesh 2d filter
        for (render_entity, visible_entity) in
            visible_entities.iter::<Mesh3d>()
        {
            // We only want meshes with the marker component to be queued to our phase.
            if has_marker.get(*render_entity).is_err() {
                continue;
            }
            let Some(mesh_instance) = render_mesh_instances
                .render_mesh_queue_data(*visible_entity)
            else {
                continue;
            };
            let Some(mesh) = render_meshes
                .get(mesh_instance.mesh_asset_id)
            else {
                continue;
            };

            // Specialize the key for the current mesh entity
            // For this example we only specialize based on the mesh topology
            // but you could have more complex keys and that's where you'd need to create those keys
            let mut mesh_key = view_key;
            mesh_key |=
                MeshPipelineKey::from_primitive_topology(
                    mesh.primitive_topology(),
                );

            let pipeline_id = pipelines.specialize(
                &pipeline_cache,
                &custom_draw_pipeline,
                mesh_key,
                &mesh.layout,
            );
            let pipeline_id = match pipeline_id {
                Ok(id) => id,
                Err(err) => {
                    error!("{}", err);
                    continue;
                }
            };
            let distance = rangefinder
                .distance_translation(
                    &mesh_instance.translation,
                );
            // At this point we have all the data we need to create a phase item and add it to our
            // phase
            custom_phase.add(SectionTexturePhase {
                // Sort the data based on the distance to the view
                sort_key: FloatOrd(distance),
                entity: (*render_entity, *visible_entity),
                pipeline: pipeline_id,
                draw_function: draw_custom,
                // Sorted phase items aren't batched
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
                indexed: mesh.indexed(),
            });
        }
    }
}

// Render label used to order our render graph node that will render our phase
#[derive(
    RenderLabel, Debug, Clone, Hash, PartialEq, Eq,
)]
struct CustomDrawPassLabel;

#[derive(Default)]
struct CustomDrawNode;
impl ViewNode for CustomDrawNode {
    type ViewQuery = (
        &'static ExtractedCamera,
        &'static ViewTarget,
        &'static ExtractedView,
        &'static SectionTexture,
        &'static ViewDepthTexture,
    );

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (camera, _target, view, section_texture, depth): QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // First, we need to get our phases resource
        let Some(section_phases) = world
            .get_resource::<ViewSortedRenderPhases<
            SectionTexturePhase,
        >>() else {
            return Ok(());
        };
        // Initialize diagnostic recording.
        // not required but makes profiling easier
        let diagnostics =
            render_context.diagnostic_recorder();

        // this target writes directly to output. keeping as
        // maybe potentially useful for debug?
        // let color_attachments =
        //     [Some(target.get_color_attachment())];
        // write to the section texture
        let color_attachments = [section_texture
            .sections
            .as_ref()
            .map(|s| s.get_attachment())];

        // Get the view entity from the graph
        let view_entity = graph.view_entity();

        // let group_id_uniforms = world
        //     .resource::<ComponentUniforms<SectionGroupId>>(
        //     );
        // let Some(group_id_binding) =
        //     group_id_uniforms.uniforms().binding()
        // else {
        //     return Ok(());
        // };
        // Get the phase for the current view running our node
        let Some(section_phase) =
            section_phases.get(&view.retained_view_entity)
        else {
            return Ok(());
        };

        let depth_stencil_attachment =
            Some(depth.get_attachment(StoreOp::Store));

        // This will generate a task to generate the command buffer in parallel
        render_context.add_command_buffer_generation_task(move |render_device| {
            #[cfg(feature = "trace")]
            let _ = info_span!("custom_section_pass").entered();

            // Command encoder setup
            let mut command_encoder =
                render_device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("custom section pass encoder"),
                });

            // Render pass setup
            let render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("section pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let mut render_pass = TrackedRenderPass::new(&render_device, render_pass);
            let pass_span = diagnostics.pass_span(&mut render_pass, "custom_pass");

            if let Some(viewport) = camera.viewport.as_ref() {
                render_pass.set_camera_viewport(viewport);
            }

            // Render the phase
            if !section_phase.items.is_empty() {
                if let Err(err) = section_phase.render(&mut render_pass, world, view_entity) {
                    error!("Error encountered while rendering the custom phase {err:?}");
                }
            }
            // render_pass.set_bind_group(2, group_id_binding, &[]);

            pass_span.end(&mut render_pass);
            drop(render_pass);
            command_encoder.finish()
        });

        Ok(())
    }
}

#[derive(Component)]
pub struct SectionTexture {
    /// The section texture generated by the plugin.
    pub sections: Option<ColorAttachment>,
    /// The size of the textures.
    pub size: Extent3d,
}

#[derive(Component)]
pub struct SectionsPrepass;

/// Prepares the textures used by the section pass
fn prepare_section_textures(
    mut commands: Commands,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
    sections_phases: Res<
        ViewSortedRenderPhases<SectionTexturePhase>,
    >,
    views_3d: Query<(
        Entity,
        &ExtractedCamera,
        &ExtractedView,
        &Msaa,
        Has<SectionsPrepass>,
    )>,
) {
    let mut sections_textures = <HashMap<_, _>>::default();

    for (entity, camera, view, msaa, sections_prepass) in
        &views_3d
    {
        if !sections_phases
            .contains_key(&view.retained_view_entity)
        {
            continue;
        };

        let Some(physical_target_size) =
            camera.physical_target_size
        else {
            continue;
        };

        let size = Extent3d {
            depth_or_array_layers: 1,
            width: physical_target_size.x,
            height: physical_target_size.y,
        };

        let cached_sections_texture = sections_prepass.then(|| {
            sections_textures
                .entry(camera.target.clone())
                .or_insert_with(|| {
                    let descriptor = TextureDescriptor {
                        label: Some("prepass_sections_texture"),
                        size,
                        mip_level_count: 1,
                        sample_count: msaa.samples(),
                        dimension: TextureDimension::D2,
                        format: if view.hdr {
                            ViewTarget::TEXTURE_FORMAT_HDR
                        } else {
                            TextureFormat::bevy_default()
                        },
                        usage: TextureUsages::COPY_DST
                            | TextureUsages::RENDER_ATTACHMENT
                            | TextureUsages::TEXTURE_BINDING,
                        view_formats: &[],
                    };
                    texture_cache.get(&render_device, descriptor)
                })
                .clone()
        });

        commands.entity(entity).insert(SectionTexture {
            sections: cached_sections_texture.map(|t| {
                ColorAttachment::new(
                    t,
                    None,
                    Some(LinearRgba::BLACK),
                )
            }),
            size,
        });
    }
}

pub struct SetSectionDataBindGroup<const I: usize>;
impl<P: PhaseItem, const I: usize> RenderCommand<P>
    for SetSectionDataBindGroup<I>
{
    type Param = (SRes<SectionDataBindGroups>,);
    type ViewQuery = ();
    type ItemQuery =
        &'static DynamicUniformIndex<SectionGroupId>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        item_query: Option<
            &DynamicUniformIndex<SectionGroupId>,
        >,
        (section_groups,): SystemParamItem<
            'w,
            '_,
            Self::Param,
        >,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let section_groups = section_groups.into_inner();

        pass.set_bind_group(
            I,
            &section_groups.0.as_ref().unwrap(),
            &[item_query.unwrap().index()],
        );

        RenderCommandResult::Success
    }
}

fn prepare_section_data_bind_group(
    mut commands: Commands,
    section_texture_pipeline: Res<SectionTexturePipeline>,
    render_device: Res<RenderDevice>,
    uniforms: Res<ComponentUniforms<SectionGroupId>>,
) {
    // todo: maybe reset the resource instead?
    let Some(uniforms) = uniforms.binding() else {
        return;
    };

    let bind_group = render_device.create_bind_group(
        "section_data_bind_group",
        &section_texture_pipeline.section_data_layout,
        &BindGroupEntries::single(uniforms),
    );
    commands.insert_resource(SectionDataBindGroups(Some(
        bind_group,
    )));
}
