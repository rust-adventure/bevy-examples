use bevy::{
    prelude::*,
    render::{
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        render_resource::{
            BindGroup, Buffer, BufferDescriptor,
            BufferUsages,
        },
        renderer::{RenderDevice, RenderQueue},
        RenderApp, RenderStage,
    },
};

pub struct GpuTimePlugin;

impl Plugin for GpuTimePlugin {
    fn build(&self, app: &mut App) {
        let render_device =
            app.world.resource::<RenderDevice>();

        let buffer = render_device.create_buffer(
            &BufferDescriptor {
                label: Some("time uniform buffer"),
                size: std::mem::size_of::<f32>() as u64,
                usage: BufferUsages::UNIFORM
                    | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            },
        );

        app.add_plugin(ExtractResourcePlugin::<
            ExtractedTime,
        >::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .insert_resource(TimeMeta {
                buffer,
                bind_group: None,
            })
            .add_system_to_stage(
                RenderStage::Prepare,
                prepare_time,
            );
    }
}

#[derive(Resource, Default)]
struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        // dbg!(time.seconds_since_startup());
        ExtractedTime {
            seconds_since_startup: time.elapsed_seconds(),
        }
    }
}

#[derive(Resource)]
pub struct TimeMeta {
    pub buffer: Buffer,
    // I left this as an Option<BindGroup> because I only
    // need it for the VolumetricMaterial pipeline. The
    // compute pipeline does this differently, so it
    // doesn't get re-used.
    pub bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding
// uniform buffer
fn prepare_time(
    time: Res<ExtractedTime>,
    time_meta: ResMut<TimeMeta>,
    render_queue: Res<RenderQueue>,
) {
    // write `time.seconds_since_startup` as a `&[u8]`
    // into the time buffer at offset 0.
    render_queue.write_buffer(
        &time_meta.buffer,
        0,
        bevy::core::cast_slice(&[
            time.seconds_since_startup
        ]),
    );
}
