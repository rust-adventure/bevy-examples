use std::any::TypeId;

use bevy::{
    asset::{
        AssetLoader, AssetPath, AsyncWriteExt, LoadContext,
        io::{Reader, Writer},
        saver::{AssetSaver, SavedAsset, save_using_saver},
    },
    camera_controller::free_camera::{
        FreeCamera, FreeCameraPlugin,
    },
    dev_tools::infinite_grid::{
        InfiniteGrid, InfiniteGridPlugin,
        InfiniteGridSettings,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    reflect::{
        TypeRegistryArc,
        serde::{
            ReflectDeserializer, ReflectSerializer,
            TypedReflectDeserializer,
        },
    },
    scene::{SceneListPatch, ScenePatch},
    tasks::IoTaskPool,
};
use serde::{Deserialize, Serialize, de::DeserializeSeed};

use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            MeshPickingPlugin,
            InfiniteGridPlugin,
            TransformGizmoPlugin,
            BoxesPlugin,
        ))
        .add_plugins(EguiPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(
                input_toggle_active(true, KeyCode::Escape),
            ),
        )
        .add_systems(
            Startup,
            (startup_scene.spawn(), startup),
        )
        .add_systems(
            Update,
            (
                user_spawn_cube,
                gizmo_mode_keys,
                exit_gizmo_mode,
            ),
        )
        .add_systems(
            PreUpdate,
            perform_save
                .run_if(input_just_pressed(KeyCode::KeyO)),
        )
        .add_observer(on_spawn_cube)
        .add_observer(on_click_select)
        .run()
}

struct BoxesPlugin;
impl Plugin for BoxesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<OneBox>()
            .init_asset::<ManyBoxes>();
    }
    fn finish(&self, app: &mut App) {
        let type_registry = app
            .world()
            .resource::<AppTypeRegistry>()
            .0
            .clone();
        app.register_asset_loader(ManyBoxesLoader {
            type_registry,
        });
    }
}

fn startup(mut commands: Commands) {
    commands.queue_spawn_scene_list(bsn! {
        :"a-test.boxes.json#scene"
    });
}
fn startup_scene() -> impl SceneList {
    bsn_list! {
        Camera3d
        FreeCamera
        TransformGizmoCamera
        template_value(
            Transform::from_xyz(-5.,2., 5.)
                .looking_at(Vec3::new(0.,1.,0.), Vec3::Y)
        ),

        InfiniteGrid
        InfiniteGridSettings,

        DirectionalLight
        template_value(
            Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
                .looking_at(Vec3::ZERO, Vec3::Y)
        )
    }
}

#[derive(Event, Debug)]
struct SpawnCube {
    transform: Transform,
}
impl Default for SpawnCube {
    fn default() -> Self {
        Self {
            transform: Transform::from_xyz(0., 0.5, 0.),
        }
    }
}

fn user_spawn_cube(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        commands.trigger(SpawnCube::default());
    }
}

fn on_spawn_cube(
    spawn: On<SpawnCube>,
    mut commands: Commands,
) {
    info!(?spawn, "spawn cube");
    commands.spawn_scene(bsn! {
        Mesh3d(asset_value(Cuboid::default()))
        MeshMaterial3d::<StandardMaterial>()
        template_value(spawn.transform.clone())
    });
}

fn on_click_select(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    existing: Query<Entity, With<TransformGizmoFocus>>,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    for e in &existing {
        commands.entity(e).remove::<TransformGizmoFocus>();
    }

    commands
        .entity(click.entity)
        .insert(TransformGizmoFocus);
}

fn exit_gizmo_mode(
    input: Res<ButtonInput<KeyCode>>,
    existing: Query<Entity, With<TransformGizmoFocus>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Escape) {
        for e in &existing {
            commands
                .entity(e)
                .remove::<TransformGizmoFocus>();
        }
    }
}
fn gizmo_mode_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<TransformGizmoSettings>,
) {
    // hack to fix macos, which the transform widget
    // doesn't respond on
    if settings.confine_cursor != false {
        settings.confine_cursor = false;
    }
    if keyboard.just_pressed(KeyCode::KeyG) {
        settings.mode = TransformGizmoMode::Translate;
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        settings.mode = TransformGizmoMode::Rotate;
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        settings.mode = TransformGizmoMode::Scale;
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        settings.space = match settings.space {
            TransformGizmoSpace::World => {
                TransformGizmoSpace::Local
            }
            TransformGizmoSpace::Local => {
                TransformGizmoSpace::World
            }
        };
    }
}

const ASSET_PATH: &str = "a-test.boxes.json";

fn perform_save(
    cubes: Query<(
        &Mesh3d,
        &Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    asset_server: Res<AssetServer>,
    app_type_registry: Res<AppTypeRegistry>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let asset_server = asset_server.clone();
    let type_registry = app_type_registry.0.clone();

    info!(cubes_len = cubes.count());

    let boxes = ManyBoxes {
        boxes: cubes
            .iter()
            .map(|(_mesh, transform, material)| OneBox {
                transform: transform.clone(),
                color: materials
                    .get(&material.0)
                    .unwrap()
                    .base_color,
            })
            .collect(),
    };
    info!(box_len = boxes.boxes.len());

    IoTaskPool::get()
        .spawn(async move {
            match save_using_saver(
                asset_server.clone(),
                &ManyBoxesSaver { type_registry },
                &ASSET_PATH.into(),
                SavedAsset::from_asset(&boxes),
                // &ImageSaverSettings::default(),
                &(),
            )
            .await
            {
                Ok(()) => {
                    info!("Completed save of {ASSET_PATH}")
                }
                Err(err) => {
                    error!("Failed to save asset: {err}")
                }
            }
        })
        .detach();
}

#[derive(Asset, Clone, Reflect)]
struct OneBox {
    transform: Transform,
    color: Color,
}

#[derive(Asset, Reflect)]
struct ManyBoxes {
    boxes: Vec<OneBox>,
}

#[derive(TypePath)]
struct ManyBoxesSaver {
    type_registry: TypeRegistryArc,
}

impl AssetSaver for ManyBoxesSaver {
    type Asset = ManyBoxes;
    type Error = BevyError;
    type OutputLoader = ManyBoxesLoader;
    type Settings = ();

    async fn save(
        &self,
        writer: &mut Writer,
        asset: SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: AssetPath<'_>,
    ) -> Result<(), Self::Error> {
        let json = {
            let type_registry = self.type_registry.read();

            let value = &*asset;

            // By default, all derived `Reflect` types can be serialized using serde. No need to derive
            // Serialize!
            let serializer = ReflectSerializer::new(
                value,
                &type_registry,
            );
            let json = serde_json::to_string(&serializer)?;
            json
        };
        writer.write_all(json.as_bytes()).await?;

        Ok(())
    }
}

/// An asset loader for loading [`ManyBoxes`] assets.
#[derive(TypePath)]
struct ManyBoxesLoader {
    type_registry: TypeRegistryArc,
}

impl AssetLoader for ManyBoxesLoader {
    type Asset = ManyBoxes;
    type Error = BevyError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        // a serde_json::Value that might have come from an API
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).unwrap();

        let type_registry = self.type_registry.read();
        let registration = type_registry
            .get(TypeId::of::<ManyBoxes>())
            .unwrap();

        // alternatively, `TypedReflectDeserializer` can be used if the type
        // is known.
        let deserializer = ReflectDeserializer::new(
            // registration,
            &type_registry,
        );
        // deserialize
        let reflect_value =
            deserializer.deserialize(value).unwrap();

        let many_boxes: ManyBoxes =
            <ManyBoxes as FromReflect>::from_reflect(
                reflect_value.as_partial_reflect(),
            )
            .unwrap();

        info!("loader_len" = many_boxes.boxes.len());
        let children = many_boxes
            .boxes
            .iter()
            .map(|one_box| {
                info!("loading box");
                bsn! {
                    Mesh3d(asset_value(Cuboid::default()))
                    MeshMaterial3d::<StandardMaterial>(asset_value(StandardMaterial{
                        base_color: one_box.color,
                        ..default()
                    }))
                    template_value(one_box.transform.clone())
                }
            })
            .collect::<Vec<_>>();

        let boxes = bsn_list![{ children }];

        let scene2 = Box::new(bsn! {
            #TestRoot
            Visibility
            Transform
            Children [
                {boxes}
            ]
        });
        let scene_patch = ScenePatch {
            scene: Some(scene2),
            dependencies: vec![],
            resolved: None,
        };
        // load_context
        //     .add_labeled_asset("scene", scene_list_patch);
        load_context
            .add_labeled_asset("scene", scene_patch);
        Ok(many_boxes)
    }

    fn extensions(&self) -> &[&str] {
        &["boxes.json"]
    }
}
