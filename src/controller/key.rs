use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{TrackballCamera, TrackballController, TrackballMessage};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn key(
    group: Entity,
    trackball_messages: &mut MessageWriter<TrackballMessage>,
    trackball: &TrackballCamera,
    controller: &mut TrackballController,
    cursor_options: &mut CursorOptions,
    key_input: &Res<ButtonInput<KeyCode>>,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    zat: f32,
    w: f32,
    v: f32,
    t: f32,
) {
    let pressed = |key: Option<KeyCode>| key.is_some_and(|key| key_input.pressed(key));
    let just_pressed = |key: Option<KeyCode>| key.is_some_and(|key| key_input.just_pressed(key));
    let just_released = |key: Option<KeyCode>| key.is_some_and(|key| key_input.just_released(key));
    let pressed_button =
        |button: Option<MouseButton>| button.is_some_and(|button| mouse_input.pressed(button));
    if just_pressed(controller.input.reset_key) {
        trackball_messages.write(TrackballMessage::reset(group));
    }
    if just_pressed(controller.input.ortho_key) {
        trackball_messages.write(TrackballMessage::ortho(group, None));
    }
    if just_pressed(controller.input.gamer_key) {
        if controller.input.slide_far_key == Some(KeyCode::KeyW) {
            controller.input.map_esdf();
        } else {
            controller.input.map_wasd();
        }
    }
    for (key, vec) in [
        (controller.input.slide_far_key, Vec3::NEG_Z),
        (controller.input.slide_near_key, Vec3::Z),
        (controller.input.slide_left_key, Vec3::NEG_X),
        (controller.input.slide_right_key, Vec3::X),
        (controller.input.slide_up_key, Vec3::Y),
        (controller.input.slide_down_key, Vec3::NEG_Y),
    ] {
        if pressed(key) {
            let v = v * controller.input.slide_key_transmission;
            trackball_messages.write(TrackballMessage::slide(group, vec * v * t));
        }
    }
    for (num, &(key, vec)) in [
        (controller.input.screw_left_key, Vec3::Z),
        (controller.input.screw_right_key, Vec3::NEG_Z),
        (controller.input.orbit_left_key, Vec3::NEG_Y),
        (controller.input.orbit_right_key, Vec3::Y),
        (controller.input.orbit_up_key, Vec3::NEG_X),
        (controller.input.orbit_down_key, Vec3::X),
    ]
    .iter()
    .enumerate()
    {
        if pressed(key) {
            let w = w * if num < 2 {
                controller.input.screw_key_transmission
            } else {
                controller.input.orbit_key_transmission
            };
            trackball_messages.write(TrackballMessage::orbit(
                group,
                Quat::from_axis_angle(vec, w * t),
                Vec3::ZERO,
            ));
        }
    }
    for key in [
        controller.input.first_left_key,
        controller.input.first_right_key,
        controller.input.first_up_key,
        controller.input.first_down_key,
    ] {
        if just_pressed(key) {
            controller.first_count += 1;
        }
        if just_released(key) && controller.first_count != 0 {
            controller.first_count -= 1;
        }
    }
    if !pressed(controller.input.first_key) && !pressed_button(controller.input.first_button) {
        if controller.first_count == 0 {
            controller.first.discard();
        } else if !controller.first.enabled() {
            controller.first.capture(trackball.frame.yaw_axis());
        }
    }
    for (key, vec) in [
        (controller.input.first_left_key, Vec2::Y),
        (controller.input.first_right_key, Vec2::NEG_Y),
        (controller.input.first_up_key, Vec2::X),
        (controller.input.first_down_key, Vec2::NEG_X),
    ] {
        // Yaw axis is discarded while key is still held when viewport changes.
        if pressed(key)
            && let Some(yaw_axis) = controller.first.yaw_axis()
        {
            let w = w * controller.input.first_key_transmission;
            let ang = vec * w * t;
            trackball_messages.write(TrackballMessage::first(group, ang.x, ang.y, yaw_axis));
        }
    }
    if just_pressed(controller.input.first_key) {
        controller.first.capture(trackball.frame.yaw_axis());
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
    if just_released(controller.input.first_key) {
        controller.first.discard();
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
    controller.scale.set_denominator(zat);
    if pressed(controller.input.scale_in_key) {
        let v = v * controller.input.scale_key_transmission;
        trackball_messages.write(TrackballMessage::scale(
            group,
            controller.scale.compute(v * t),
            Vec3::ZERO,
        ));
    }
    if pressed(controller.input.scale_out_key) {
        let v = v * controller.input.scale_key_transmission;
        trackball_messages.write(TrackballMessage::scale(
            group,
            controller.scale.compute(-v * t),
            Vec3::ZERO,
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::{prelude::*, window::CursorOptions};

    use super::key;
    use crate::{TrackballCamera, TrackballController, TrackballMessage};

    #[allow(clippy::needless_pass_by_value)]
    fn system(
        mut messages: MessageWriter<TrackballMessage>,
        mut controllers: Query<(Entity, &TrackballCamera, &mut TrackballController)>,
        key_input: Res<ButtonInput<KeyCode>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
    ) {
        for (group, trackball, mut controller) in &mut controllers {
            key(
                group,
                &mut messages,
                trackball,
                &mut controller,
                &mut CursorOptions::default(),
                &key_input,
                &mouse_input,
                1.0,
                1.0,
                1.0,
                0.01,
            );
        }
    }

    #[test]
    fn held_first_key_does_not_panic_when_viewport_changes() {
        let mut app = App::new();
        app.add_message::<TrackballMessage>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .add_systems(Update, system);
        let camera = TrackballCamera::look_at(Vec3::Z, Vec3::ZERO, Vec3::Y);
        let entity = app
            .world_mut()
            .spawn((camera, TrackballController::default()))
            .id();
        let count = |app: &mut App| {
            app.world_mut()
                .resource_mut::<Messages<TrackballMessage>>()
                .drain()
                .count()
        };
        // Pressing key captures yaw axis.
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);
        app.update();
        assert_eq!(count(&mut app), 1);
        // Key is still held but no longer just pressed when viewport change resets controller.
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        let mut controller = app
            .world_mut()
            .get_mut::<TrackballController>(entity)
            .unwrap();
        controller.first_count = 0;
        controller.first.discard();
        app.update();
        assert_eq!(count(&mut app), 0);
    }
}
