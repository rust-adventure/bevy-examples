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

#[derive(Resource)]
pub struct VolumetricImage(pub Handle<Image>);
pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
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

#[derive(Resource, Default)]
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

#[derive(Resource)]
pub struct FogMeta {
    pub image: Handle<Image>,
    pub bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding
// uniform buffer
fn prepare_fog(
    extracted_fog: Res<ExtractedFog>,
    mut fog_meta: ResMut<FogMeta>,
) {
    fog_meta.image = extracted_fog.image.clone();
}
