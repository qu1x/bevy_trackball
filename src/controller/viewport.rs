use bevy::prelude::*;
use bevy::{
	input::{keyboard::KeyCode, mouse::MouseWheel, touch::TouchPhase},
	render::camera::RenderTarget,
	window::{PrimaryWindow, Window, WindowRef},
};

use super::{TrackballCamera, TrackballController};

/// Trackball viewport currently focused and hence capturing input events.
///
///  * Enables multiple viewports/windows with individual controllers/cameras.
///  * Enables UI systems to steal the viewport in order to capture input events.
#[derive(Resource, Clone, Debug, PartialEq, Eq, Default)]
pub struct TrackballViewport {
	entity: Option<Entity>,
	stolen: usize,
}

impl TrackballViewport {
	/// Whether the viewport has been stolen.
	#[allow(clippy::needless_pass_by_value)]
	#[must_use]
	pub fn stolen(viewport: Res<Self>) -> bool {
		viewport.stolen != 0
	}
	/// Steals the viewport or gives it back.
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_stolen(&mut self, stolen: bool) {
		if stolen {
			self.entity = None;
			self.stolen = 2;
		} else if self.stolen != 0 {
			self.stolen -= 1;
		}
	}
	#[allow(clippy::too_many_arguments)]
	#[allow(clippy::type_complexity)]
	pub(super) fn select<'a>(
		viewport: &mut ResMut<Self>,
		key_input: &Res<Input<KeyCode>>,
		mouse_input: &Res<Input<MouseButton>>,
		touch_events: &mut EventReader<TouchInput>,
		wheel_events: &EventReader<MouseWheel>,
		primary_windows: &'a mut Query<&mut Window, With<PrimaryWindow>>,
		secondary_windows: &'a mut Query<&mut Window, Without<PrimaryWindow>>,
		cameras: &'a mut Query<(Entity, &Camera, &TrackballCamera, &mut TrackballController)>,
	) -> Option<(
		bool,
		Mut<'a, Window>,
		Entity,
		&'a Camera,
		&'a TrackballCamera,
		Mut<'a, TrackballController>,
	)> {
		let touch = touch_events
			.iter()
			.filter_map(|touch| (touch.phase == TouchPhase::Started).then_some(touch.position))
			.last();
		let input = !wheel_events.is_empty()
			|| key_input.get_just_pressed().len() != 0
			|| mouse_input.get_just_pressed().len() != 0;
		let mut new_viewport = viewport.clone();
		let mut max_order = 0;
		for (entity, camera, _trackball, _controller) in cameras.iter() {
			let RenderTarget::Window(window_ref) = camera.target else {
				continue;
			};
			let window = match window_ref {
				WindowRef::Primary => primary_windows.get_single().ok(),
				WindowRef::Entity(entity) => secondary_windows.get(entity).ok(),
			};
			let Some(window) = window else {
				continue;
			};
			let Some(pos) = touch
				.filter(|_pos| window.focused)
				.or_else(|| window.cursor_position().filter(|_pos| input))
			else {
				continue;
			};
			let Some(Rect { min, max }) = camera.logical_viewport_rect() else {
				continue;
			};
			let contained = (min.x..max.x).contains(&pos.x) && (min.y..max.y).contains(&pos.y);
			if contained && camera.order >= max_order {
				new_viewport.entity = Some(entity);
				max_order = camera.order;
			}
		}
		let is_changed = viewport.entity != new_viewport.entity;
		if is_changed {
			viewport.entity = new_viewport.entity;
		}
		let camera = viewport
			.entity
			.and_then(|entity| cameras.get_mut(entity).ok());
		let Some((entity, camera, trackball, controller)) = camera else {
			viewport.entity = None;
			return None;
		};
		let RenderTarget::Window(window_ref) = camera.target else {
			return None;
		};
		let window = match window_ref {
			WindowRef::Primary => primary_windows.get_single_mut().ok(),
			WindowRef::Entity(entity) => secondary_windows.get_mut(entity).ok(),
		}?;
		Some((is_changed, window, entity, camera, trackball, controller))
	}
}
