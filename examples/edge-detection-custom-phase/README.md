## Edge Detection with Section Textures

![demo](./readme/demo.avif)

This demo uses a custom phase to render a Mesh's custom vertex attribute (`SectionColor`) to a texture.

This render only stores data in the red channel of the texture.

![section-texture-hdr](./readme/section-texture-hdr.avif)

That texture is then used as a source to run a sobel-filter based outline in a post-process pass.

![sobel output](./readme/sobel-output.avif)

The numbers in this texture are in an hdr space, which means they all look pure red in that screenshot. If the output is divided by the number of objects, it looks more like this which shows how the values are different.

![divided section texture](./readme/section-texture-divided.avif)

This results in artist-controllable "outlines" (in quotes because not just outlines, but also any color transition).

The special value 1.0 can be used to fill in an entire section with the outline color, and the special value 0. can be used to render no outline.

## End-user usage

### Camera

The camera requires the `SectionsPrepass` component to generate the section texture, and the `PostProcessSettings` component to configure the post-process that uses the texture.

It doesn't work with Msaa, and must be `hdr`.

```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 7., 14.0)
        .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    Camera {
        clear_color: Color::from(SLATE_950).into(),
        // THIS ONLY WORKS WITH HDR CAMERAS
        hdr: true,
        ..default()
    },
    // disable msaa for simplicity
    Msaa::Off,
    PostProcessSettings {
        stroke_color: Color::WHITE.into(),
        width: 2,
    },
    SectionsPrepass,
));
```

### Meshes

Any entity with a `Mesh3d` that you want to contribute to the section texture must be labelled with the `DrawSection` component. This is not a hard requirement, it is a choice made using the `has_marker` query in the section texture code that could be removed.

Meshes also need to have a custom vertex attribute: `SectionColor`. This can either be done in Blender or in code. This demo uses a code-based approach that makes arbitrary decisions about section ids colors based on normals.
