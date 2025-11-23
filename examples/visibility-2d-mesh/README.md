This is an incubating example showing a 2d visibility mesh generated each frame.

> [!CAUTION]
>
> refactors required, code messy
>
> Works with 2d bevy shapes and glTF files

---

- manual, naive raycasting
- optimizations (obvhs, etc)

---

Assumptions:

## "Hull-like"

Each mesh has a series of vertices that defines their outer shape in order.
This means we don't need to compute convex hulls or similar.
The Rhombus invalidates this assumption, as its vertices are bottom, right, left, top, which results in missing bottom-left and top-right edges, while introducing vertical and horizontal internal edges.
Surprisingly, the annulus (ring of circle) and the rhombus seem to be the only built-in 2d shapes that exhibit this kind of behavior.
Rectangles, regular polygons, all the way up to circles work just fine.

And you could built a Rhombus that behaves appropriately with the right vertex order.

> [!NOTE]
> This assumption has to be revisited if the future includes arbitrary glTF mesh support, since most users would not know how to order individual vertices.

## Rotations

In the code, the GlobalTransform.translation is added directly to the Mesh's local vertex position, which is _not_ where it would be in space if the shape's Transform was rotated.

## Concavity

This Polyline results in some odd raycast behavior, ending rays right at vertices.

```rust
Polyline2d::new(vec![
    Vec2::new(-50.0, 50.0),
    Vec2::new(0.0, -50.0),
    Vec2::new(50.0, 50.0),
])
```

## TODO:

- [ ] maybe revisit "hull order" assumption
  - if this is done, it likely add
- [ ] maybe fix rotation handling
- [ ] GPU-based processing
  - Current implementation is CPU-based, with the performance drawbacks of being CPU based. A high count of rays for CPU processing is a low amount for GPU.
