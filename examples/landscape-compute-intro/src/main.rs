use std::ops::Not;

use bevy::{
    anti_alias::fxaa::Fxaa,
    asset::RenderAssetUsages,
    camera::Exposure,
    camera_controller::free_camera::{
        FreeCamera, FreeCameraPlugin,
    },
    color::palettes::tailwind::GREEN_800,
    core_pipeline::{
        schedule::camera_driver, tonemapping::Tonemapping,
    },
    light::{
        Atmosphere, AtmosphereEnvironmentMapLight,
        CascadeShadowConfigBuilder, VolumetricFog,
        VolumetricLight, atmosphere::ScatteringMedium,
        light_consts::lux,
    },
    pbr::{AtmosphereSettings, ScreenSpaceReflections},
    platform::collections::HashSet,
    post_process::bloom::Bloom,
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup,
        extract_component::{
            ExtractComponent, ExtractComponentPlugin,
        },
        mesh::allocator::MeshAllocator,
        render_resource::{
            binding_types::{
                storage_buffer, uniform_buffer,
            },
            *,
        },
        renderer::{
            RenderContext, RenderGraph, RenderQueue,
        },
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;

const SHADER_ASSET_PATH: &str = "landscape.wgsl";

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight::NONE)
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            ComputeShaderMeshGeneratorPlugin,
            ShaderUtilsPlugin,
            ExtractComponentPlugin::<GenerateMesh>::default(
            ),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

// We need a plugin to organize all the systems and render node required for this example
struct ComputeShaderMeshGeneratorPlugin;
impl Plugin for ComputeShaderMeshGeneratorPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };

        render_app
            .init_resource::<ChunksToProcess>()
            .add_systems(
                RenderStartup,
                init_compute_pipeline,
            )
            .add_systems(Render, prepare_chunks)
            .add_systems(
                RenderGraph,
                compute_landscape.before(camera_driver),
            );
    }
    fn finish(&self, app: &mut App) {
        let Some(render_app) =
            app.get_sub_app_mut(RenderApp)
        else {
            return;
        };
        render_app
            .world_mut()
            .resource_mut::<MeshAllocator>()
            // This allows using the mesh allocator slabs as
            // storage buffers directly in the compute shader.
            // Which means that we can write from our compute
            // shader directly to the allocated mesh slabs.
            .extra_buffer_usages = BufferUsages::STORAGE;
    }
}

/// Holds a handle to the empty mesh that should be filled
/// by the compute shader.
#[derive(Component, ExtractComponent, Clone)]
struct GenerateMesh {
    num_vertices: u32,
    mesh_handle: Handle<Mesh>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scattering_mediums: ResMut<
        Assets<ScatteringMedium>,
    >,
) {
    let empty_mesh = {
        let mut mesh =
            Plane3d::new(Vec3::Y, Vec2::splat(200.))
                .mesh()
                .subdivisions(100)
                .build();
        mesh.generate_tangents().unwrap();
        // let mut mesh = Mesh::new(
        //     PrimitiveTopology::TriangleList,
        //     RenderAssetUsages::RENDER_WORLD,
        // )
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_POSITION,
        //     vec![[0.; 3]; 50],
        // )
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_NORMAL,
        //     vec![[0.; 3]; 50],
        // )
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_UV_0,
        //     vec![[0.; 2]; 50],
        // )
        // .with_inserted_indices(Indices::U32(vec![0; 50]));

        mesh.asset_usage = RenderAssetUsages::RENDER_WORLD;
        mesh
    };
    // for attr in empty_mesh.attributes() {
    //     dbg!(attr);
    // }
    dbg!(empty_mesh.indices().unwrap().len());
    let num_vertices = empty_mesh.indices().unwrap().len();
    let handle = meshes.add(empty_mesh);

    commands.spawn((
        GenerateMesh {
            num_vertices: num_vertices as u32,
            mesh_handle: handle.clone(),
        },
        Mesh3d(handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN_800.into(),
            perceptual_roughness: 0.,
            reflectance: 0.,
            ..default()
        })),
        Transform::from_xyz(0., 0., 0.),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-25., 45., 90.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Atmosphere::earthlike(
            scattering_mediums
                .add(ScatteringMedium::default()),
        ),
        // Can be adjusted to change the scene scale and rendering quality
        AtmosphereSettings::default(),
        // The directional light illuminance used in this scene
        // (the one recommended for use with this feature) is
        // quite bright, so raising the exposure compensation helps
        // bring the scene to a nicer brightness range.
        Exposure { ev100: 13.0 },
        // Tonemapper chosen just because it looked good with the scene, any
        // tonemapper would be fine :)
        Tonemapping::AcesFitted,
        // Bloom gives the sun a much more natural look.
        Bloom::NATURAL,
        // Enables the atmosphere to drive reflections and ambient lighting (IBL) for this view
        AtmosphereEnvironmentMapLight::default(),
        FreeCamera::default(),
        VolumetricFog {
            ambient_intensity: 0.0,
            ..default()
        },
        Msaa::Off,
        Fxaa::default(),
        ScreenSpaceReflections::default(),
    ));
    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config =
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.3,
            maximum_distance: 15.0,
            ..default()
        }
        .build();

    // Sun
    commands.spawn((
        DirectionalLight {
            // shadow_maps_enabled: true,
            // lux::RAW_SUNLIGHT is recommended for use with this feature, since
            // other values approximate sunlight *post-scattering* in various
            // conditions. RAW_SUNLIGHT in comparison is the illuminance of the
            // sun unfiltered by the atmosphere, so it is the proper input for
            // sunlight to be filtered by the atmosphere.
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, 0.4, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        VolumetricLight,
        cascade_shadow_config,
    ));
}

/// This is called `ChunksToProcess` because this example originated
/// from a use case of generating chunks of landscape or voxels
/// It only exists in the render world.
#[derive(Resource, Default)]
struct ChunksToProcess(Vec<(u32, AssetId<Mesh>)>);

/// `processed` is a `HashSet` contains the `AssetId`s that have been
/// processed. We use that to remove `AssetId`s that have already
/// been processed, which means each unique `GenerateMesh` will result
/// in one compute shader mesh generation process instead of generating
/// the mesh every frame.
fn prepare_chunks(
    meshes_to_generate: Query<&GenerateMesh>,
    mut chunks: ResMut<ChunksToProcess>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<ComputePipeline>,
    mut processed: Local<HashSet<(u32, AssetId<Mesh>)>>,
) {
    // If the pipeline isn't ready, then meshes
    // won't be processed. So we want to wait until
    // the pipeline is ready before considering any mesh processed.
    if pipeline_cache
        .get_compute_pipeline(pipeline.pipeline)
        .is_some()
    {
        // get the AssetId for each Handle<Mesh>
        // which we'll use later to get the relevant buffers
        // from the mesh_allocator
        let chunk_data: Vec<(u32, AssetId<Mesh>)> =
            meshes_to_generate
                .iter()
                .filter_map(|gmesh| {
                    let id = (
                        gmesh.num_vertices,
                        gmesh.mesh_handle.id(),
                    );
                    // Some(id)
                    processed
                        .contains(&id)
                        .not()
                        .then_some(id)
                })
                .collect();

        // Cache any meshes we're going to process this frame
        for id in &chunk_data {
            processed.insert(*id);
        }

        chunks.0 = chunk_data;
    }
}

#[derive(Resource)]
struct ComputePipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}

// init only happens once
fn init_compute_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // offsets
                uniform_buffer::<DataRanges>(false),
                // vertices
                storage_buffer::<Vec<u32>>(false),
                // indices
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );
    let shader = asset_server.load(SHADER_ASSET_PATH);
    let pipeline = pipeline_cache.queue_compute_pipeline(
        ComputePipelineDescriptor {
            label: Some(
                "Mesh generation compute shader".into(),
            ),
            layout: vec![layout.clone()],
            shader: shader.clone(),
            ..default()
        },
    );
    commands.insert_resource(ComputePipeline {
        layout,
        pipeline,
    });
}

// A uniform that holds the vertex and index offsets
// for the vertex/index mesh_allocator buffer slabs
#[derive(ShaderType)]
struct DataRanges {
    num_vertices: u32,
    vertex_start: u32,
    vertex_end: u32,
    index_start: u32,
    index_end: u32,
}

fn compute_landscape(
    mut render_context: RenderContext,
    chunks: Res<ChunksToProcess>,
    mesh_allocator: Res<MeshAllocator>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<ComputePipeline>,
    render_queue: Res<RenderQueue>,
) {
    for mesh_id in &chunks.0 {
        if let Some(init_pipeline) = pipeline_cache
            .get_compute_pipeline(pipeline.pipeline)
        {
            // the mesh_allocator holds slabs of meshes, so the buffers we get here
            // can contain more data than just the mesh we're asking for.
            // That's why there is a range field.
            // You should *not* touch data in these buffers that is outside of the range.
            let vertex_buffer_slice = mesh_allocator
                .mesh_vertex_slice(&mesh_id.1)
                .unwrap();
            let index_buffer_slice = mesh_allocator
                .mesh_index_slice(&mesh_id.1)
                .unwrap();

            let first = DataRanges {
                num_vertices: mesh_id.0,
                // there are 12 vertex data values (pos, normal, uv, tangent) per vertex
                // and the vertex_buffer_slice.range.start is in "vertex elements"
                // which includes all of that data, so each index is worth 8 indices
                // to our shader code.
                vertex_start: vertex_buffer_slice
                    .range
                    .start
                    * 12,
                vertex_end: vertex_buffer_slice.range.end
                    * 12,
                // but each vertex index is a single value, so the index of the
                // vertex indices is exactly what the value is
                index_start: index_buffer_slice.range.start,
                index_end: index_buffer_slice.range.end,
            };

            let mut uniforms = UniformBuffer::from(first);
            uniforms.write_buffer(
                render_context.render_device(),
                &render_queue,
            );

            // pass in the full mesh_allocator slabs as well as the first index
            // offsets for the vertex and index buffers
            let bind_group = render_context
                .render_device()
                .create_bind_group(
                    None,
                    &pipeline_cache.get_bind_group_layout(
                        &pipeline.layout,
                    ),
                    &BindGroupEntries::sequential((
                        &uniforms,
                        vertex_buffer_slice
                            .buffer
                            .as_entire_buffer_binding(),
                        index_buffer_slice
                            .buffer
                            .as_entire_buffer_binding(),
                    )),
                );

            let mut pass = render_context
                .command_encoder()
                .begin_compute_pass(
                    &ComputePassDescriptor {
                        label: Some(
                            "Mesh generation compute pass",
                        ),
                        ..default()
                    },
                );
            pass.push_debug_group("compute_mesh");

            pass.set_bind_group(0, &bind_group, &[]);
            pass.set_pipeline(init_pipeline);
            // we only dispatch 1,1,1 workgroup here, but a real compute shader
            // would take advantage of more and larger size workgroups
            pass.dispatch_workgroups(1, 1, 1);

            pass.pop_debug_group();
        }
    }
}
