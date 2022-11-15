use crate::time::TimeMeta;
use bevy::{
    prelude::*,
    render::{
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
};
use std::borrow::Cow;

// HAS TO MATCH SIZE IN MAIN
pub const SIZE: (u32, u32, u32) = (720, 720, 720);
const WORKGROUP_SIZE: u32 = 10;

pub struct CloudGeneratorComputePlugin;

impl Plugin for CloudGeneratorComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractResourcePlugin::<
            CloudGeneratorImage,
        >::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<CloudGeneratorPipeline>()
            .add_system_to_stage(
                RenderStage::Queue,
                queue_bind_group,
            );

        let mut render_graph =
            render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "cloud_generator",
            CloudGeneratorNode::default(),
        );
        render_graph
            .add_node_edge(
                "cloud_generator",
                bevy::render::main_graph::node::CAMERA_DRIVER,
            )
            .unwrap();
    }
}

// Resource is opt-in in main branch
#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct CloudGeneratorImage(pub Handle<Image>);

#[derive(Resource)]
struct CloudGeneratorImageBindGroup(BindGroup);

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<CloudGeneratorPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    cloud_generator_image: Res<CloudGeneratorImage>,
    render_device: Res<RenderDevice>,
    time_meta: ResMut<TimeMeta>,
) {
    let view = &gpu_images[&cloud_generator_image.0];

    let bind_group = render_device.create_bind_group(
        &BindGroupDescriptor {
            label: None,
            layout: &pipeline.texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &view.texture_view,
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: time_meta
                        .buffer
                        .as_entire_binding(),
                },
            ],
        },
    );
    commands.insert_resource(CloudGeneratorImageBindGroup(
        bind_group,
    ));
}

#[derive(Resource)]
pub struct CloudGeneratorPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for CloudGeneratorPipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D3,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                        },
                        count: None,
                    }],
                });

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/compute_cloud.wgsl");

        let mut pipeline_cache =
            world.resource_mut::<PipelineCache>();

        let init_pipeline = pipeline_cache
            .queue_compute_pipeline(
                ComputePipelineDescriptor {
                    label: None,
                    layout: Some(vec![
                        texture_bind_group_layout.clone(),
                    ]),
                    shader: shader.clone(),
                    shader_defs: vec![],
                    entry_point: Cow::from("init"),
                },
            );

        let update_pipeline = pipeline_cache
            .queue_compute_pipeline(
                ComputePipelineDescriptor {
                    label: None,
                    layout: Some(vec![
                        texture_bind_group_layout.clone(),
                    ]),
                    shader,
                    shader_defs: vec![],
                    entry_point: Cow::from("update"),
                },
            );

        CloudGeneratorPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum CloudGeneratorState {
    Loading,
    Init,
    Update,
}

struct CloudGeneratorNode {
    state: CloudGeneratorState,
}

impl Default for CloudGeneratorNode {
    fn default() -> Self {
        Self {
            state: CloudGeneratorState::Loading,
        }
    }
}

impl render_graph::Node for CloudGeneratorNode {
    fn update(&mut self, world: &mut World) {
        let pipeline =
            world.resource::<CloudGeneratorPipeline>();
        let pipeline_cache =
            world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded,
        // transition to the next stage
        match self.state {
            CloudGeneratorState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.init_pipeline,
                        )
                {
                    self.state = CloudGeneratorState::Init;
                }
            }
            CloudGeneratorState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.update_pipeline,
                        )
                {
                    self.state =
                        CloudGeneratorState::Update;
                }
            }
            CloudGeneratorState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world
            .resource::<CloudGeneratorImageBindGroup>()
            .0;
        let pipeline_cache =
            world.resource::<PipelineCache>();
        let pipeline =
            world.resource::<CloudGeneratorPipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(
                &ComputePassDescriptor::default(),
            );

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            CloudGeneratorState::Loading => {}
            CloudGeneratorState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(
                        pipeline.init_pipeline,
                    )
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIZE.0 / WORKGROUP_SIZE,
                    SIZE.1 / WORKGROUP_SIZE,
                    SIZE.2 / WORKGROUP_SIZE,
                );
            }
            CloudGeneratorState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(
                        pipeline.update_pipeline,
                    )
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIZE.0 / WORKGROUP_SIZE,
                    SIZE.1 / WORKGROUP_SIZE,
                    SIZE.2 / WORKGROUP_SIZE,
                );
            }
        }

        Ok(())
    }
}
