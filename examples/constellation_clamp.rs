//! Demonstrates rigid as well as loose constellation clamp on the ground plane.
//!
//!  1. There is a main viewport (maximap) and a minimap at the right bottom.
//!  2. Both have their own trackball controller.
//!  3. The minimap is additionally sensitive to the controller of the maximap.
//!  5. When orbiting the maximap down by pressing `k`, it will stop as soon as the initially lower
//!     positioned minimap camera hits the ground plane.
//!  6. Toggling from rigid to loose constellation clamp by pressing `Space` while keeping `k`
//!     pressed allows the maximap camera to orbit further until it hits the ground by itself.
//!  7. Press `Return` to reset camera positions and try again.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::f32::consts::PI;

use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::{
		camera::Viewport,
		render_resource::{Extent3d, TextureDimension, TextureFormat},
	},
	window::WindowResized,
};
use bevy_trackball::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, resize_minimap)
		.add_systems(Update, toggle_rigid_loose)
		.run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
	windows: Query<&Window>,
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
		meshes.add(shape::Cube::default().into()),
		meshes.add(shape::Box::default().into()),
		meshes.add(shape::Capsule::default().into()),
		meshes.add(shape::Torus::default().into()),
		meshes.add(shape::Cylinder::default().into()),
		meshes.add(shape::Icosphere::default().try_into().unwrap()),
		meshes.add(shape::UVSphere::default().into()),
	];

	let num_shapes = shapes.len();

	for (i, shape) in shapes.into_iter().enumerate() {
		commands.spawn((
			PbrBundle {
				mesh: shape,
				material: debug_material.clone(),
				transform: Transform::from_xyz(
					(i as f32 / (num_shapes - 1) as f32).mul_add(X_EXTENT, -X_EXTENT / 2.),
					3.0,
					0.0,
				)
				.with_rotation(Quat::from_rotation_x(-PI / 4.)),
				..default()
			},
			Shape,
		));
	}

	// ground plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Plane::from_size(50.0).into()),
		material: materials.add(Color::SILVER.into()),
		..default()
	});
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 9000.0,
			range: 100.,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(8.0, 16.0, 8.0),
		..default()
	});
	// UI
	commands.spawn(
		TextBundle::from_section(
			"Rigid Constellation Clamp (Toggle: Space)",
			TextStyle {
				font_size: 18.0,
				color: Color::WHITE,
				..default()
			},
		)
		.with_style(Style {
			position_type: PositionType::Absolute,
			bottom: Val::Px(10.0),
			left: Val::Px(10.0),
			..default()
		}),
	);
	// cameras and boundary conditions
	let mut bound = Bound::default();
	bound.min_target[1] = 0.0;
	bound.min_eye[1] = 0.3;
	bound.min_distance = 6.0;
	bound.max_distance = 50.0;
	let [target, eye, up] = [Vec3::Y, Vec3::new(0.0, 9.0, 12.0), Vec3::Y];
	let maximap = commands
		.spawn((
			TrackballController::default(),
			TrackballCamera::look_at(target, eye, up).with_clamp(bound.clone()),
			Camera3dBundle::default(),
		))
		.id();
	let window = windows.single();
	let size = window.resolution.physical_width() / 4;
	let eye = Vec3::new(0.0, 3.0, 12.0);
	commands.spawn((
		TrackballController::default(),
		TrackballCamera::look_at(target, eye, Vec3::Y)
			.with_clamp(bound)
			.add_controller(maximap, true),
		Camera3dBundle {
			camera: Camera {
				order: 1,
				viewport: Some(Viewport {
					physical_position: UVec2::new(
						window.resolution.physical_width() - size,
						window.resolution.physical_height() - size,
					),
					physical_size: UVec2::new(size, size),
					..default()
				}),
				..default()
			},
			camera_3d: Camera3d {
				clear_color: ClearColorConfig::None,
				..default()
			},
			..default()
		},
		MinimapCamera,
	));
}

#[derive(Component)]
struct MinimapCamera;

#[allow(clippy::needless_pass_by_value)]
fn resize_minimap(
	windows: Query<&Window>,
	mut resize_events: EventReader<WindowResized>,
	mut minimap: Query<&mut Camera, With<MinimapCamera>>,
) {
	for resize_event in &mut resize_events {
		let window = windows.get(resize_event.window).unwrap();
		let mut minimap = minimap.single_mut();
		let size = window.resolution.physical_width() / 4;
		minimap.viewport = Some(Viewport {
			physical_position: UVec2::new(
				window.resolution.physical_width() - size,
				window.resolution.physical_height() - size,
			),
			physical_size: UVec2::new(size, size),
			..default()
		});
	}
}

#[allow(clippy::needless_pass_by_value)]
fn toggle_rigid_loose(
	mut minimap: Query<&mut TrackballCamera, With<MinimapCamera>>,
	mut text: Query<&mut Text>,
	keycode: Res<Input<KeyCode>>,
) {
	if keycode.just_pressed(KeyCode::Space) {
		let mut text = text.single_mut();
		let text = &mut text.sections[0].value;
		let mut minimap = minimap.single_mut();
		let rigid = minimap.group.values_mut().next().unwrap();
		if *rigid {
			*text = "Loose Constellation Clamp (Toggle: Space)".to_owned();
		} else {
			*text = "Rigid Constellation Clamp (Toggle: Space)".to_owned();
		}
		*rigid = !*rigid;
	}
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
	)
}
