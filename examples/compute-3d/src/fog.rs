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

pub struct VolumetricImage(pub Handle<Image>);
pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        let render_device =
            app.world.resource::<RenderDevice>();

        // let buffer = render_device.create_buffer(
        //     &BufferDescriptor {
        //         label: Some("fog texture buffer"),
        //         size: std::mem::size_of::<f32>() as u64,
        //         usage: BufferUsages::UNIFORM
        //             | BufferUsages::COPY_DST,
        //         mapped_at_creation: false,
        //     },
        // );

        app.add_plugin(
            ExtractResourcePlugin::<ExtractedFog>::default(
            ),
        );
        let image = app
            .world
            .resource::<VolumetricImage>()
            .0
            .clone();
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .insert_resource(FogMeta {
                image,
                bind_group: None,
            })
            .add_system_to_stage(
                RenderStage::Prepare,
                prepare_fog,
            );
    }
}

// #[derive(Resource, Default)]
#[derive(Default)]
struct ExtractedFog {
    image: Handle<Image>,
}

impl ExtractResource for ExtractedFog {
    type Source = VolumetricImage;

    fn extract_resource(volume: &Self::Source) -> Self {
        ExtractedFog {
            image: volume.0.clone(),
        }
    }
}

// #[derive(Resource)]
pub struct FogMeta {
    pub image: Handle<Image>,
    pub bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding
// uniform buffer
fn prepare_fog(
    extracted_fog: Res<ExtractedFog>,
    mut fog_meta: ResMut<FogMeta>,
    // render_queue: Res<RenderQueue>,
) {
    // write `time.seconds_since_startup` as a `&[u8]`
    // into the time buffer at offset 0.
    // render_queue.write_buffer(
    //     &fog_meta.buffer,
    //     0,
    //     bevy::core::cast_slice(&[extracted_fog]),
    // );
    fog_meta.image = extracted_fog.image.clone();
}
