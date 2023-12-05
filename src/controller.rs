use bevy::{
	input::{
		keyboard::KeyCode,
		mouse::{MouseMotion, MouseWheel},
	},
	prelude::*,
	time::Time,
	window::{CursorGrabMode, CursorIcon, PrimaryWindow},
};
pub use input::{TrackballInput, TrackballVelocity, TrackballWheelUnit};
use key::key;
use mouse::mouse;
use touch::touch;
use trackball::{First, Orbit, Scale, Slide, Touch};
pub use viewport::TrackballViewport;

use super::{TrackballCamera, TrackballEvent};

mod input;
mod key;
mod mouse;
mod touch;
mod viewport;

/// Trackball controller component mainly defined by [`TrackballInput`].
#[derive(Component, Clone, Debug, Default)]
pub struct TrackballController {
	/// Input mappings and settings.
	pub input: TrackballInput,

	first: First<f32>,
	orbit: Orbit<f32>,
	scale: Scale<f32>,
	slide: Slide<f32>,
	touch: Touch<Option<u64>, f32>,

	first_count: usize,
}

impl TrackballController {
	/// Trackball controller using [`TrackballInput::map_esdf`].
	#[must_use]
	pub fn map_esdf() -> Self {
		let mut controller = Self::default();
		controller.input.map_esdf();
		controller
	}
	/// Trackball controller using [`TrackballInput::map_wasd`].
	#[must_use]
	pub fn map_wasd() -> Self {
		let mut controller = Self::default();
		controller.input.map_wasd();
		controller
	}
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
pub fn trackball_controller(
	mut viewport: ResMut<TrackballViewport>,
	time: Res<Time>,
	key_input: Res<Input<KeyCode>>,
	mouse_input: Res<Input<MouseButton>>,
	mut touch_events: EventReader<TouchInput>,
	mut touch_events_clone: EventReader<TouchInput>,
	mut delta_events: EventReader<MouseMotion>,
	mut mouse_events: EventReader<CursorMoved>,
	mut wheel_events: EventReader<MouseWheel>,
	mut primary_windows: Query<&mut Window, With<PrimaryWindow>>,
	mut secondary_windows: Query<&mut Window, Without<PrimaryWindow>>,
	mut cameras: Query<(Entity, &Camera, &TrackballCamera, &mut TrackballController)>,
	mut trackball_events: EventWriter<TrackballEvent>,
) {
	if viewport.was_stolen() {
		touch_events.clear();
		delta_events.clear();
		mouse_events.clear();
		wheel_events.clear();
	}
	let Some((is_changed, mut window, group, camera, trackball, mut controller)) =
		TrackballViewport::select(
			&mut viewport,
			&key_input,
			&mouse_input,
			&mut touch_events_clone,
			&wheel_events,
			&mut primary_windows,
			&mut secondary_windows,
			&mut cameras,
		)
	else {
		return;
	};
	let Some((min, max)) = camera
		.logical_viewport_rect()
		.map(|Rect { min, max }| (min, max - min))
	else {
		return;
	};
	if is_changed {
		controller.first_count = 0;
		controller.first.discard();
		controller.orbit.discard();
		controller.slide.discard();
		controller.touch.discard(None);
		controller.touch.discard(None);
		window.cursor.icon = CursorIcon::Arrow;
		window.cursor.grab_mode = CursorGrabMode::None;
		window.cursor.visible = true;
	}
	let zat = trackball.frame.distance();
	let (_max, upp) = trackball.scope.fov().max_and_upp(zat, &max.into());
	let v = controller.input.velocity.to_linear(zat).into_inner();
	let w = controller.input.velocity.to_angular(zat).into_inner();
	let t = time.delta_seconds();
	key(
		group,
		&mut trackball_events,
		trackball,
		&mut controller,
		&mut window,
		&key_input,
		&mouse_input,
		zat,
		w,
		v,
		t,
	);
	mouse(
		group,
		&mut trackball_events,
		trackball,
		&mut controller,
		&mut window,
		&mouse_input,
		delta_events,
		mouse_events,
		wheel_events,
		zat,
		upp,
		min,
		max,
		w,
	);
	touch(
		group,
		&mut trackball_events,
		trackball,
		&mut controller,
		touch_events,
		upp,
		min,
		max,
	);
}
