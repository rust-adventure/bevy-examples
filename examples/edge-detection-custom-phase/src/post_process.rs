use bevy::{
    core_pipeline::{
        Core3d, Core3dSystems, FullscreenShader,
        tonemapping::tonemapping,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        RenderApp,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex,
            ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_resource::{
            binding_types::{
                sampler, texture_2d, uniform_buffer,
            },
            *,
        },
        renderer::{
            RenderContext, RenderDevice, ViewQuery,
        },
        view::ViewTarget,
    },
};

use crate::SectionTexture;

/// It is generally encouraged to set up post
/// processing effects as a plugin
pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // The settings will be a component that lives in the main world but will
            // be extracted to the render world every frame.
            // This makes it possible to control the effect from the main world.
            // This plugin will take care of extracting it automatically.
            // It's important to derive [`ExtractComponent`] on [`PostProcessingSettings`]
            // for this plugin to work correctly.
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            // The settings will also be the data used in the shader.
            // This plugin will prepare the component for the GPU by creating a uniform buffer
            // and writing the data to that buffer every frame.
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));

        // We need to get the render app from the main app
        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };

        // render_app.add_systems(RenderStartup, init_post_process_pipeline);
        render_app.add_systems(
            Core3d,
            post_process_system
                .in_set(Core3dSystems::PostProcess)
                // TODO: Probably not supposed to run after tonemapping??
                .after(tonemapping),
        );
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<PostProcessPipeline>();
    }
}

fn post_process_system(
    world: &World,
    view: ViewQuery<(
        &'static ViewTarget,
        // This makes sure the node only runs on cameras
        // with the PostProcessSettings component
        &'static PostProcessSettings,
        &'static SectionTexture,
        // As there could be multiple post processing
        // components sent to the GPU (one per camera),
        // we need to get the index of the one that is
        // associated with the current view.
        &'static DynamicUniformIndex<PostProcessSettings>,
    )>,
    mut render_context: RenderContext,
    settings_uniforms: Res<
        ComponentUniforms<PostProcessSettings>,
    >,
    pipeline_cache: Res<PipelineCache>,
) {
    let (
        view_target,
        _post_process_settings,
        section_texture,
        settings_index,
    ) = view.into_inner();
    // Get the pipeline resource that contains the
    // global data we need to create the
    // render pipeline
    let post_process_pipeline =
        world.resource::<PostProcessPipeline>();

    // Get the pipeline from the cache
    let Some(pipeline) = pipeline_cache
        .get_render_pipeline(
            post_process_pipeline.pipeline_id,
        )
    else {
        return;
    };

    // Get the settings uniform binding
    let Some(settings_binding) =
        settings_uniforms.uniforms().binding()
    else {
        return;
    };

    // get the GpuImage for a Handle<Image>
    // let vertex_texture = world
    //     .resource::<RenderAssets<GpuImage>>()
    //     .get(&section_texture.0)
    //     .unwrap();
    // dbg!(&section_texture.sections.iter().len());
    let section_texture_view = &section_texture
        .sections
        .as_ref()
        .unwrap()
        .texture
        .default_view;
    // dbg!(&vertex_texture);

    // This will start a new "post process write",
    // obtaining two texture views from the
    // view target - a `source` and a `destination`.
    // `source` is the "current" main texture and you
    // _must_ write into `destination` because
    // calling `post_process_write()` on the
    // [`ViewTarget`] will internally flip the
    // [`ViewTarget`]'s main texture to the
    // `destination` texture. Failing to do so will
    // cause the current main texture
    // information to be lost.
    let post_process = view_target.post_process_write();

    // The bind_group gets created each frame.
    //
    // Normally, you would create a bind_group in the
    // Queue set, but this doesn't work with
    // the post_process_write(). The reason it
    // doesn't work is because each post_process_write
    // will alternate the source/destination.
    // The only way to have the correct
    // source/destination for the bind_group
    // is to make sure you get it during the node
    // execution.
    let bind_group =
        render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &pipeline_cache.get_bind_group_layout(
                &post_process_pipeline.layout,
            ),
            // It's important for this to match the
            // BindGroupLayout defined in the
            // PostProcessPipeline
            &BindGroupEntries::sequential((
                // Make sure to use the source view
                post_process.source,
                // Use the sampler created for the
                // pipeline
                &post_process_pipeline.sampler,
                // Set the settings binding
                settings_binding.clone(),
                section_texture_view.into_binding(),
                // Use the sampler created for the
                // pipeline
                &post_process_pipeline.vertex_id_sampler,
            )),
        );

    // Begin the render pass
    let mut render_pass = render_context
        .begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(
                RenderPassColorAttachment {
                    // We need to specify the post
                    // process destination view here
                    // to make sure we write to the
                    // appropriate texture.
                    view: post_process.destination,
                    resolve_target: None,
                    ops: Operations::default(),
                    depth_slice: None,
                },
            )],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

    // This is mostly just wgpu boilerplate for
    // drawing a fullscreen triangle,
    // using the pipeline/bind_group created above
    render_pass.set_render_pipeline(pipeline);
    // By passing in the index of the post process
    // settings on this view, we ensure
    // that in the event that multiple settings were
    // sent to the GPU (as would be the
    // case with multiple cameras), we use the correct
    // one.
    render_pass.set_bind_group(
        0,
        &bind_group,
        &[settings_index.index()],
    );
    render_pass.draw(0..3, 0..1);
}

// This contains global data used by the render
// pipeline. This will be created once on startup.
#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    vertex_id_sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device =
            world.resource::<RenderDevice>();

        // We need to define the bind group layout used
        // for our pipeline
        let layout = BindGroupLayoutDescriptor::new(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float {
                        filterable: true,
                    }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<PostProcessSettings>(
                        true,
                    ),
                    // The vertex color id texture
                    texture_2d(TextureSampleType::Float {
                        filterable: true,
                    }),
                    // The sampler that will be used to sample the vertex color id texture
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        // We can create the sampler here since it won't
        // change at runtime and doesn't depend on the
        // view
        let sampler = render_device
            .create_sampler(&SamplerDescriptor::default());
        let vertex_id_sampler = render_device
            .create_sampler(&SamplerDescriptor::default());

        // Get the shader handle
        let shader =
            world.load_asset("post_processing.wgsl");

        let fullscreen_shader = world
            .get_resource::<FullscreenShader>()
            .expect("FullscreenShader Resource is required for post-process")
            .to_vertex_state();

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue its creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader,
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: Some("fragment".into()),
                    targets: vec![Some(ColorTargetState {
                        format: ViewTarget::TEXTURE_FORMAT_HDR,
                        //     format: if key
                        //     .contains(MeshPipelineKey::HDR)
                        // {
                        //     ViewTarget::TEXTURE_FORMAT_HDR
                        // } else {
                        //     TextureFormat::bevy_default()
                        // },
                        // format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all fields can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                immediate_size: 0,
                zero_initialize_workgroup_memory: false,
            });

        Self {
            layout,
            sampler,
            vertex_id_sampler,
            // vertex_texture_handle,
            pipeline_id,
        }
    }
}

// This is the component that will get passed to
// the shader
#[derive(
    Component,
    Default,
    Clone,
    Copy,
    ExtractComponent,
    ShaderType,
)]
pub struct PostProcessSettings {
    pub stroke_color: LinearRgba,
    pub width: u32,
    // pub display:
    // WebGL2 structs must be 16 byte aligned.
    // #[cfg(feature = "webgl2")]
    // _webgl2_padding: Vec3,
}

/// What should the post-process display.
/// TODO: can we shader def this in?
enum PostProcessDisplay {
    Default,
    SobelOnly,
    SectionTextureOnly,
}
