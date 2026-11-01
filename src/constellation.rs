use bevy::prelude::*;

use super::{TrackballCamera, TrackballMessage, TrackballSetup};

const LOOPS: usize = 100;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
pub fn trackball_constellation(
    mut trackball_events: MessageReader<TrackballMessage>,
    mut cameras: Query<(Entity, &mut TrackballCamera)>,
) {
    for trackball_event in trackball_events.read() {
        let mut min_delta = trackball_event.delta;
        let mut loops = 0;
        loop {
            let mut bound = false;
            for (group, trackball) in &cameras {
                if (group != trackball_event.group
                    && !trackball.group.contains_key(&trackball_event.group))
                    || !is_rigid(trackball, trackball_event.group)
                {
                    continue;
                }
                if let Some(clamp) = &trackball.clamp
                    && let Some((delta, loops)) =
                        clamp.compute(trackball.frame, trackball.scope, min_delta)
                {
                    if loops == clamp.loops() {
                        debug!("Stopping camera clamp after {loops} loops (entity {group:?})");
                    } else {
                        debug!("Found camera clamp after {loops} loops (entity {group:?})");
                    }
                    bound = true;
                    min_delta = delta;
                    break;
                }
            }
            if bound {
                if loops == LOOPS {
                    warn!("Using partial clamp after {LOOPS} loops");
                    break;
                }
                loops += 1;
            } else {
                debug!("Found common clamp after {loops} loops");
                break;
            }
        }
        for (group, mut trackball) in &mut cameras {
            if group != trackball_event.group
                && !trackball.group.contains_key(&trackball_event.group)
            {
                continue;
            }
            let delta = if is_rigid(&trackball, trackball_event.group) {
                min_delta
            } else {
                // Follows active controller irrespective of the clamps of rigid cameras.
                trackball
                    .clamp
                    .as_ref()
                    .and_then(|clamp| {
                        clamp.compute(trackball.frame, trackball.scope, trackball_event.delta)
                    })
                    .map_or(trackball_event.delta, |(delta, _loops)| delta)
            };
            trackball.frame = delta.transform(trackball.frame);
            trackball.frame.renormalize();
            if let Some(setup) = trackball_event.setup {
                match setup {
                    TrackballSetup::Ortho(ortho) => {
                        let ortho = ortho.unwrap_or_else(|| !trackball.scope.ortho());
                        trackball.scope.set_ortho(ortho);
                    }
                    TrackballSetup::Reset => {
                        trackball.frame = trackball.reset;
                    }
                }
            }
        }
    }
}

/// Whether camera clamps active controller of `group` and hence all rigid cameras of this group.
fn is_rigid(trackball: &TrackballCamera, group: Entity) -> bool {
    trackball.group.get(&group).copied().unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use trackball::{Bound, Delta};

    use super::trackball_constellation;
    use crate::{TrackballCamera, TrackballMessage};

    /// Slides down active and loose camera at different heights above ground plane.
    fn slide_down(rigid: bool) -> (f32, f32) {
        let camera = |height: f32| {
            let mut bound = Bound::default();
            bound.min_target[1] = 0.0;
            let target = Vec3::new(0.0, height, 0.0);
            TrackballCamera::look_at(target + Vec3::Z * 10.0, target, Vec3::Y).with_clamp(bound)
        };
        let mut app = App::new();
        app.add_message::<TrackballMessage>()
            .add_systems(Update, trackball_constellation);
        let active = app.world_mut().spawn(camera(1.0)).id();
        let follower = app
            .world_mut()
            .spawn(camera(2.0).add_controller(active, rigid))
            .id();
        app.world_mut().write_message(TrackballMessage {
            group: active,
            delta: Delta::Slide {
                vector: Vec3::NEG_Y * 3.0,
            },
            setup: None,
        });
        app.update();
        let height = |entity: Entity| {
            app.world()
                .get::<TrackballCamera>(entity)
                .unwrap()
                .frame
                .target()
                .y
        };
        (height(active), height(follower))
    }

    #[test]
    fn loose_camera_clamps_itself_when_active_camera_is_clamped() {
        let (active, loose) = slide_down(false);
        assert!(active.abs() < 1e-3, "active camera at {active}");
        assert!(loose.abs() < 1e-3, "loose camera at {loose}");
    }

    #[test]
    fn rigid_camera_keeps_distance_when_active_camera_is_clamped() {
        let (active, rigid) = slide_down(true);
        assert!(active.abs() < 1e-3, "active camera at {active}");
        assert!((rigid - 1.0).abs() < 1e-3, "rigid camera at {rigid}");
    }
}
