# Bevy Blockout

![demo](readme/demo.avif)

A utility package that provides blockout utilities. Currently this includes a triplanar blockout mesh that uses worldspace coordinates to texture meshes using a grid where 1 box equals 1 world unit.

> [!NOTE]  
> This crate is in development. Expect changes.

## Quick Start

```rust
commands.spawn((
    Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(
        10., 4., 20.,
    )))),
    Transform::from_xyz(10.0, 2., 0.0),
    MeshMaterial3d(materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: SKY_400.into(),
            ..default()
        },
        extension: BlockoutMaterialExt::default(),
    })),
));
```

## TODO

- [ ] Smaller squares could combine better on angled surfaces
