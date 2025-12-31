use bevy::{
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
	prelude::*,
	window::{CursorGrabMode, CursorIcon, CursorOptions, SystemCursorIcon},
};
use trackball::{
	Image,
	nalgebra::{Point2, Point3},
};

use super::{TrackballCamera, TrackballController, TrackballMessage};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn mouse(
	commands: &mut Commands,
	group: Entity,
	trackball_events: &mut MessageWriter<TrackballMessage>,
	trackball: &TrackballCamera,
	controller: &mut TrackballController,
	window_id: Entity,
	window: &Window,
	cursor_options: &mut CursorOptions,
	mouse_input: &Res<ButtonInput<MouseButton>>,
	mut delta_events: MessageReader<MouseMotion>,
	mut mouse_events: MessageReader<CursorMoved>,
	mut wheel_events: MessageReader<MouseWheel>,
	zat: f32,
	upp: f32,
	min: Vec2,
	max: Vec2,
	w: f32,
) {
	let pos = Point2::from(window.cursor_position().map_or(max * 0.5, |pos| pos - min));
	let max = max.into();
	let just_pressed_button =
		|button: Option<MouseButton>| button.is_some_and(|button| mouse_input.just_pressed(button));
	let just_released_button = |button: Option<MouseButton>| {
		button.is_some_and(|button| mouse_input.just_released(button))
	};
	if just_pressed_button(controller.input.first_button) {
		controller.first.capture(trackball.frame.yaw_axis());
		cursor_options.grab_mode = CursorGrabMode::Locked;
		cursor_options.visible = false;
	}
	if just_released_button(controller.input.first_button) {
		controller.first.discard();
		cursor_options.grab_mode = CursorGrabMode::None;
		cursor_options.visible = true;
	}
	for delta_event in delta_events.read() {
		if controller.first.enabled()
			&& let Some((pitch, yaw, yaw_axis)) =
				controller.first.compute(&(-delta_event.delta).into(), &max)
		{
			let pitch = pitch * controller.input.first_mouse_transmission;
			let yaw = yaw * controller.input.first_mouse_transmission;
			trackball_events.write(TrackballMessage::first(group, pitch, yaw, *yaw_axis));
		}
	}
	if just_pressed_button(controller.input.orbit_button) {
		controller.touch.compute(None, pos, 0);
		controller.orbit.compute(&pos, &max);
		commands
			.entity(window_id)
			.insert(CursorIcon::from(SystemCursorIcon::Pointer));
	}
	if just_released_button(controller.input.orbit_button) {
		if let Some((_num, pos)) = controller.touch.discard(None)
			&& controller.input.focus
		{
			let (pos, _max) = Image::transform_pos_and_max_wrt_max(&pos, &max);
			let vec = pos.coords.scale(upp).push(0.0);
			trackball_events.write(TrackballMessage::slide(group, vec));
		}
		controller.orbit.discard();
		commands
			.entity(window_id)
			.insert(CursorIcon::from(SystemCursorIcon::Default));
	}
	if just_pressed_button(controller.input.slide_button) {
		controller.slide.compute(pos);
		commands
			.entity(window_id)
			.insert(CursorIcon::from(SystemCursorIcon::Move));
	}
	if just_released_button(controller.input.slide_button) {
		controller.slide.discard();
		commands
			.entity(window_id)
			.insert(CursorIcon::from(SystemCursorIcon::Default));
	}
	for mouse_event in mouse_events.read() {
		let pos = mouse_event.position - min;
		if controller
			.input
			.orbit_button
			.is_some_and(|button| mouse_input.pressed(button))
			&& let Some((_num, pos, _rot, _rat)) = controller.touch.compute(None, pos.into(), 0)
			&& let Some(rot) = controller.orbit.compute(&pos, &max)
		{
			let rot = rot.powf(controller.input.orbit_mouse_transmission);
			trackball_events.write(TrackballMessage::orbit(group, rot, Point3::origin()));
		}
		if controller
			.input
			.slide_button
			.is_some_and(|button| mouse_input.pressed(button))
			&& let Some(vec) = controller
				.slide
				.compute(pos.into())
				.map(|vec| Image::transform_vec(&vec))
		{
			let vec = vec.scale(upp).push(0.0) * controller.input.slide_mouse_transmission;
			trackball_events.write(TrackballMessage::slide(group, vec));
		}
	}
	for &wheel_event in wheel_events.read() {
		let num = match wheel_event.unit {
			MouseScrollUnit::Line => {
				let denominator = controller.input.wheel_unit.denominator(w);
				controller.scale.set_denominator(denominator);
				wheel_event.y
			}
			MouseScrollUnit::Pixel => {
				controller.scale.set_denominator(zat);
				upp * wheel_event.y
			}
		} * controller.input.scale_wheel_transmission;
		let (pos, _max) = Image::transform_pos_and_max_wrt_max(&pos, &max);
		trackball_events.write(TrackballMessage::scale(
			group,
			controller.scale.compute(num),
			pos.coords.scale(upp).push(0.0).into(),
		));
	}
}
