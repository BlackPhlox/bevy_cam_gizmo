# Bevy Camera Gizmo

This is a modified version of [bevy_transform_gizmo](https://github.com/ForesightMiningSoftwareCorporation/bevy_transform_gizmo).

This Bevy plugin adds a transform gizmo to entities in the scene, allowing you to drag and rotate meshes with your mouse.

![image](https://user-images.githubusercontent.com/25123512/128579461-538723fe-f091-4701-85bc-2dafa2a73462.png)

# Demo

Run a minimal implementation of the gizmo by cloning this repository and running:

```shell
cargo run --example minimal
```

# Features

* Prebuilt transform gizmo appears when you select a designated mesh
* Translation handles
* Rotation handles
* Gizmo always renders on top of the main render pass
* Gizmo scales at it moves closer/further from the camera

# Usage

This plugin is built on and relies on [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking) for 3d mouse interaction with the scene.

Add the plugin to the `[dependencies]` in `Cargo.toml`

```toml
bevy_cam_gizmo = { git = "https://github.com/BlackPhlox/bevy_cam_gizmo", branch = "main" }
```

You will need to add the transform gizmo plugin, as well as make sure you have also brought in the picking plugin.

```rust
.add_plugin(bevy_mod_picking::DefaultPickingPlugins)
.add_plugin(bevy_cam_gizmo::TransformGizmoPlugin)
```

Next, you will need to mark your picking camera as your gizmo camera:

```rust
.insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
.insert(bevy_cam_gizmo::GizmoPickSource::default());
```

Finally, mark any meshes you want to be transformed with the gizmo; note they must also be selectable in the picking plugin:

```rust
.insert_bundle(bevy_mod_picking::PickableBundle::default())
.insert(bevy_transform_gizmo::GizmoTransformable);
```

See the [minimal](examples/minimal.rs) demo for an example of a minimal implementation.
