//! Demonstrates gliding on the ground plane (currently, only implemented for orbit and slide
//! operations).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::f32::consts::PI;

use bevy::{
	prelude::*,
	render::{
		render_asset::RenderAssetUsages,
		render_resource::{Extent3d, TextureDimension, TextureFormat},
	},
};
use bevy_trackball::prelude::*;

fn main() {
	App::new()
		.add_plugins(
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						canvas: Some("#bevy".to_owned()),
						..default()
					}),
					..default()
				}),
		)
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 12.0;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut images: ResMut<Assets<Image>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let debug_material = materials.add(StandardMaterial {
		base_color_texture: Some(images.add(uv_debug_texture())),
		..default()
	});

	let shapes = [
		meshes.add(Cuboid::default()),
		meshes.add(Capsule3d::default()),
		meshes.add(Torus::default()),
		meshes.add(Cylinder::default()),
		meshes.add(Sphere::default().mesh().ico(5).unwrap()),
		meshes.add(Sphere::default().mesh().uv(32, 18)),
	];

	let num_shapes = shapes.len();

	for (i, shape) in shapes.into_iter().enumerate() {
		commands.spawn((
			PbrBundle {
				mesh: shape,
				material: debug_material.clone(),
				transform: Transform::from_xyz(
					(i as f32 / (num_shapes - 1) as f32).mul_add(X_EXTENT, -X_EXTENT / 2.),
					2.0,
					0.0,
				)
				.with_rotation(Quat::from_rotation_x(-PI / 4.)),
				..default()
			},
			Shape,
		));
	}

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			shadows_enabled: true,
			intensity: 10_000_000.,
			range: 100.0,
			..default()
		},
		transform: Transform::from_xyz(8.0, 16.0, 8.0),
		..default()
	});

	// ground plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
		material: materials.add(Color::SILVER),
		..default()
	});

	// camera
	let [target, eye, up] = [Vec3::Y, Vec3::new(0.0, 6.0, 12.0), Vec3::Y];
	commands.spawn((
		TrackballController::default(),
		TrackballCamera::look_at(target, eye, up).with_clamp({
			let mut bound = Bound::default();
			bound.min_target[1] = 0.0;
			bound.min_eye[1] = 0.3;
			bound.min_distance = 6.0;
			bound.max_distance = 50.0;
			bound
		}),
		Camera3dBundle::default(),
	));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
	const TEXTURE_SIZE: usize = 8;

	let mut palette: [u8; 32] = [
		255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
		198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
	];

	let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
	for y in 0..TEXTURE_SIZE {
		let offset = TEXTURE_SIZE * y * 4;
		texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
		palette.rotate_right(4);
	}

	Image::new_fill(
		Extent3d {
			width: TEXTURE_SIZE as u32,
			height: TEXTURE_SIZE as u32,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		&texture_data,
		TextureFormat::Rgba8UnormSrgb,
		RenderAssetUsages::RENDER_WORLD,
	)
}
