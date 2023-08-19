//! A simple 3D scene with light shining over a cube sitting on a plane and a trackball camera
//! waiting for your input to orbit around.
//!
//! This is an alternative trackball technique using exponential map and parallel transport to
//! preserve distances and angles for inducing coherent and intuitive trackball rotations. For
//! instance, displacements on straight radial lines through the screen’s center are carried to arcs
//! of the same length on great circles of the trackball (e.g., dragging the mouse along an eights
//! of the trackball's circumference rolls the camera by 360/8=45 degrees, dragging the mouse from
//! the screen's center to its further edge *linearly* rotates the camera by 1 [radian], where the
//! trackball's diameter is the maximum of the screen's width and height). This is in contrast to
//! state-of-the-art techniques using orthogonal projection which distorts radial distances further
//! away from the screen’s center (e.g., the rotation accelerates towards the edge).
//!
//! [radian]: https://en.wikipedia.org/wiki/Radian

use bevy::prelude::*;
use bevy_trackball::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.run();
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Plane::from_size(5.0).into()),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands.spawn((
		TrackballController::default(),
		TrackballCamera::look_at(Vec3::Y * 0.25, Vec3::new(-2.0, 2.5, 5.0), Vec3::Y),
		Camera3dBundle::default(),
	));
}
