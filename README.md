# bevy_trackball

Coherent virtual trackball controller/camera plugin for Bevy

[![Build][]](https://github.com/qu1x/bevy_trackball/actions/workflows/build.yml)
[![Downloads][]](https://crates.io/crates/bevy_trackball)
[![Version][]](https://crates.io/crates/bevy_trackball)
[![Rust][]](https://www.rust-lang.org)
[![License][]](https://opensource.org/licenses)

[Build]: https://github.com/qu1x/bevy_trackball/actions/workflows/build.yml/badge.svg
[Downloads]: https://img.shields.io/crates/d/bevy_trackball.svg
[Version]: https://img.shields.io/crates/v/bevy_trackball.svg
[Rust]: https://img.shields.io/badge/rust-v1.71-brightgreen.svg
[License]: https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg

[Documentation](https://qu1x.github.io/bevy_trackball/doc/bevy_trackball)

Run simple and advanced [examples] in your browser using [WebAssembly] and [WebGL].

[WebAssembly]: https://en.wikipedia.org/wiki/WebAssembly
[WebGL]: https://en.wikipedia.org/wiki/WebGL

## Camera Modes

Supports multiple camera modes:

  * Trackball mode rotates camera around target.
  * First-Person mode rotates target around camera.
  * Spectator mode translates target and camera.

## Coherence Features

This is an alternative trackball technique using exponential map and parallel transport to
preserve distances and angles for inducing coherent and intuitive trackball rotations. For
instance, displacements on straight radial lines through the screen’s center are carried to arcs
of the same length on great circles of the trackball (e.g., dragging the mouse along an eights
of the trackball's circumference rolls the camera by 360/8=45 degrees, dragging the mouse from
the screen's center to its further edge *linearly* rotates the camera by 1 [radian], where the
trackball's diameter is the maximum of the screen's width and height). This is in contrast to
state-of-the-art techniques using orthogonal projection which distorts radial distances further
away from the screen’s center (e.g., the rotation accelerates towards the edge).

[radian]: https://en.wikipedia.org/wiki/Radian

  * Coherent and intuitive orbiting via the exponential map, see the underlying [`trackball`]
    crate which follows the recipe given in the paper of Stantchev, G.. “Virtual Trackball
    Modeling and the Exponential Map.”. [S2CID] [44199608]. See the [`exponential_map`] example.
  * Coherent first-person mode aka free look or mouse look with the world trackball centered at
    eye instead of target.
  * Coherent scaling by translating mouse wheel device units, see [`TrackballWheelUnit`]. Scales
    eye distance from current cursor position or centroid of finger positions projected onto
    focus plane.
  * Coherent linear/angular [`TrackballVelocity`] for sliding/orbiting or free look by
    time-based input (e.g., pressed key). By default, the linear velocity is deduced from the
    angular velocity (where target and eye positions define the world radius) which in turn is
    defined in units of vertical field of view per seconds and hence independent of the world
    unit scale.

[S2CID]: https://en.wikipedia.org/wiki/S2CID_(identifier)
[44199608]: https://api.semanticscholar.org/CorpusID:44199608

[`trackball`]: https://qu1x.github.io/bevy_trackball/doc/trackball/index.html
[`TrackballWheelUnit`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/enum.TrackballWheelUnit.html
[`TrackballVelocity`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/enum.TrackballVelocity.html

## Additional Features

  * Time-free multi-touch gesture recognition for orbit, scale, slide, and focus (i.e., slide to
    cursor/finger position) operations.
  * Smoothing of movement implemented as fps-agnostic exponential easy-out.
  * Gimbal lock-free using quaternion instead of Euler angles.
  * Gliding clamp (experimental): The movement of a camera can be restricted to user-defined
    boundary conditions (e.g., to not orbit below the ground plane). When the movement is not
    orthogonal to a boundary plane, it is changed such that the camera glides along the boundary
    plane. Currently, only implemented for orbit and slide operations, see the [`gliding_clamp`]
    example.
  * Camera constellation: A camera is decoupled from its input controller and instead multiple
    cameras can be sensitive to zero or multiple selected controllers (e.g., a minimap
    controlled by the same controller of the main viewport).
  * Constellation clamp: Cameras sensitive to the same controller are referred to as a group
    and can be configured to clamp the movement for the whole group whenever a group member
    crosses a boundary condition (e.g., rigid and loose constellation clamp), see the
    [`constellation_clamp`] example.
  * Viewport stealing: This allows UI system (e.g., egui behind `bevy_egui` feature gate) to
    steal the viewport and hence capture the input instead, see the [`egui`] example.
  * Scale-preserving transitioning between orthographic and perspective projection mode.
  * Converting between scaling modes (i.e., fixed vertical or horizontal field of view or fixed
    unit per pixels). This defines whether the scene scales or the corresponding vertical or
    horizontal field of view adjusts whenever the height or width of the viewport is resized,
    see the [`scaling_modes`] example.
  * Object inspection mode scaling clip plane distances by measuring from target instead of eye.
    This benefits the precision of the depth map. Applicable, whenever the extend of the object
    to inspect is known and hence the near clip plane can safely be placed just in front of it.
  * `f64`-ready for large worlds (e.g., solar system scale) whenever Bevy is, see issue [#1680].

[#1680]: https://github.com/bevyengine/bevy/issues/1680

See the [release history](RELEASES.md) to keep track of the development.

## Input Mappings

Following mappings are the defaults which can be customized, see [`TrackballInput`].

Mouse (Buttons)         | Touch (Fingers)         | Keyboard | Operation
----------------------- | ----------------------- | -------- | ---------------------------------
Left Press + Drag       | One + Drag              | `ijkl`   | Orbits around target.
↳ at trackball's border | Two + Roll              | `uo`     | Rolls about view direction.
Middle Press + Drag     | Any + Drag + Left Shift | `↑←↓→`   | First-person mode.
Right Press + Drag      | Two + Drag              | `esdf`   | Slides trackball on focus plane.
&nbsp;                  | &nbsp;                  | `gv`     | Slides trackball in/out.
Scroll In/Out           | Two + Pinch Out/In      | `hn`     | Scales distance zooming in/out.
Left Press + Release    | Any + Release           | &nbsp;   | Slides to cursor/finger position.
&nbsp;                  | &nbsp;                  | `m`      | Toggle `esdf`/`wasd` mapping.
&nbsp;                  | &nbsp;                  | `p`      | Toggle orthographic/perspective.
&nbsp;                  | &nbsp;                  | `Return` | Reset camera transform.

Alternatively, [`TrackballInput::map_wasd`] maps `wasd`/`Space`/`ControlLeft` to slide
operations where `wd` slides in/out and `Space`/`ControlLeft` slides up/down (jump/crouch).

[`TrackballInput`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/struct.TrackballInput.html
[`TrackballInput::map_wasd`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/struct.TrackballInput.html#method.map_wasd

## Usage

Add the [`TrackballPlugin`] followed by spawning a [`TrackballController`] together with a
[`TrackballCamera`] and a [`Camera3dBundle`] or see simple and advanced [examples].

```rust
use bevy::prelude::*;
use bevy_trackball::prelude::*;

// Add the trackball plugin.
fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.run();
}

// Add a trackball controller and trackball camera to a camera 3D bundle.
fn setup(mut commands: Commands) {
	let [target, eye, up] = [Vec3::ZERO, Vec3::Z * 10.0, Vec3::Y];
	commands.spawn((
		TrackballController::default(),
		TrackballCamera::look_at(target, eye, up),
		Camera3dBundle::default(),
	));

	// Set up your scene...
}
```

[`TrackballPlugin`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/struct.TrackballPlugin.html
[`TrackballController`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/struct.TrackballController.html
[`TrackballCamera`]: https://qu1x.github.io/bevy_trackball/doc/bevy_trackball/struct.TrackballCamera.html
[`Camera3dBundle`]: https://qu1x.github.io/bevy_trackball/doc/bevy/core_pipeline/core_3d/struct.Camera3dBundle.html

[examples]: https://qu1x.github.io/bevy_trackball/examples
[`exponential_map`]: https://qu1x.github.io/bevy_trackball/examples/exponential_map.html
[`gliding_clamp`]: https://qu1x.github.io/bevy_trackball/examples/gliding_clamp.html
[`constellation_clamp`]: https://qu1x.github.io/bevy_trackball/examples/constellation_clamp.html
[`egui`]: https://qu1x.github.io/bevy_trackball/examples/egui.html
[`scaling_modes`]: https://github.com/qu1x/bevy_trackball/blob/main/examples/scaling_modes.rs

# License

Copyright © 2023 Rouven Spreckels <rs@qu1x.dev>

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSES/Apache-2.0](LICENSES/Apache-2.0) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSES/MIT](LICENSES/MIT) or https://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
