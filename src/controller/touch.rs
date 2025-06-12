use bevy::{input::touch::TouchPhase, prelude::*};
use trackball::{
	Image,
	nalgebra::{Point3, UnitQuaternion},
};

use super::{TrackballCamera, TrackballController, TrackballEvent};

#[allow(clippy::too_many_arguments, clippy::similar_names)]
pub fn touch(
	group: Entity,
	trackball_events: &mut EventWriter<TrackballEvent>,
	trackball: &TrackballCamera,
	controller: &mut TrackballController,
	mut touch_events: EventReader<TouchInput>,
	upp: f32,
	min: Vec2,
	max: Vec2,
) {
	let max = max.into();
	for &touch_event in touch_events.read() {
		let TouchInput {
			id,
			phase,
			position: pos,
			..
		} = touch_event;
		let pos = pos - min;
		match phase {
			TouchPhase::Started | TouchPhase::Moved => {
				if phase == TouchPhase::Started {
					controller.slide.discard();
				}
				if let Some((num, pos, rot, rat)) =
					controller.touch.compute(Some(id), pos.into(), 0)
				{
					if controller.first.enabled() {
						if let Some(vec) = controller.slide.compute(pos) {
							if let Some((pitch, yaw, yaw_axis)) =
								controller.first.compute(&vec, &max)
							{
								let pitch = pitch * controller.input.first_touch_transmission;
								let yaw = yaw * controller.input.first_touch_transmission;
								trackball_events
									.write(TrackballEvent::first(group, pitch, yaw, *yaw_axis));
							}
						}
					} else if num == 1 {
						if let Some(rot) = controller.orbit.compute(&pos, &max) {
							let rot = rot.powf(controller.input.orbit_touch_transmission);
							trackball_events.write(TrackballEvent::orbit(
								group,
								rot,
								Point3::origin(),
							));
						}
					} else {
						if let Some(vec) = controller
							.slide
							.compute(pos)
							.map(|vec| Image::transform_vec(&vec))
						{
							let vec = vec.scale(upp).push(0.0)
								* controller.input.slide_touch_transmission;
							trackball_events.write(TrackballEvent::slide(group, vec));
						}
						if num == 2 {
							let (pos, _max) = Image::transform_pos_and_max_wrt_max(&pos, &max);
							let pos = pos.coords.scale(upp).push(0.0);
							let rot = UnitQuaternion::from_axis_angle(
								&trackball.frame.local_roll_axis(),
								rot * controller.input.screw_touch_transmission,
							);
							let rat = rat * controller.input.scale_touch_transmission;
							trackball_events.write(TrackballEvent::orbit(group, rot, pos.into()));
							trackball_events.write(TrackballEvent::scale(group, rat, pos.into()));
						}
					}
				}
			}
			TouchPhase::Ended | TouchPhase::Canceled => {
				if let Some((_num, pos)) = controller.touch.discard(Some(id)) {
					if controller.input.focus {
						let (pos, _max) = Image::transform_pos_and_max_wrt_max(&pos, &max);
						let vec = pos.coords.scale(upp).push(0.0);
						trackball_events.write(TrackballEvent::slide(group, vec));
					}
				}
				controller.orbit.discard();
				controller.slide.discard();
			}
		}
	}
}
