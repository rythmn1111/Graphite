use crate::consts::{DRAG_BEYOND_VIEWPORT_MAX_OVEREXTENSION_PIXELS, DRAG_BEYOND_VIEWPORT_SPEED_FACTOR};
use crate::messages::prelude::*;
use crate::messages::tool::tool_messages::tool_prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AutoPanning {
	subscribed_to_animation_frame: bool,
}

impl AutoPanning {
	pub fn start(&mut self, messages: &[Message], responses: &mut VecDeque<Message>) {
		if !self.subscribed_to_animation_frame {
			self.subscribed_to_animation_frame = true;

			for message in messages {
				responses.add(BroadcastMessage::SubscribeEvent {
					on: BroadcastEvent::AnimationFrame,
					send: Box::new(message.clone()),
				});
			}
		}
	}

	pub fn stop(&mut self, messages: &[Message], responses: &mut VecDeque<Message>) {
		if self.subscribed_to_animation_frame {
			self.subscribed_to_animation_frame = false;

			for message in messages {
				responses.add(BroadcastMessage::UnsubscribeEvent {
					on: BroadcastEvent::AnimationFrame,
					message: Box::new(message.clone()),
				});
			}
		}
	}

	pub fn setup_by_mouse_position(&mut self, mouse_position: DVec2, viewport_size: DVec2, messages: &[Message], responses: &mut VecDeque<Message>) {
		let is_pointer_outside_edge = mouse_position.x < 0. || mouse_position.x > viewport_size.x || mouse_position.y < 0. || mouse_position.y > viewport_size.y;

		match is_pointer_outside_edge {
			true => self.start(messages, responses),
			false => self.stop(messages, responses),
		}
	}

	/// Shifts the viewport when the mouse reaches the edge of the viewport.
	///
	/// If the mouse was beyond any edge, it returns the amount shifted. Otherwise it returns None.
	/// The shift is proportional to the distance between edge and mouse. It is also guaranteed to be integral.
	pub fn shift_viewport(mouse_position: DVec2, viewport_size: DVec2, responses: &mut VecDeque<Message>) -> Option<DVec2> {
		let mouse_position = mouse_position.clamp(
			DVec2::ZERO - DVec2::splat(DRAG_BEYOND_VIEWPORT_MAX_OVEREXTENSION_PIXELS),
			viewport_size + DVec2::splat(DRAG_BEYOND_VIEWPORT_MAX_OVEREXTENSION_PIXELS),
		);
		let mouse_position_percent = mouse_position / viewport_size;

		let mut shift_percent = DVec2::ZERO;

		if mouse_position_percent.x < 0. {
			shift_percent.x = -mouse_position_percent.x;
		} else if mouse_position_percent.x > 1. {
			shift_percent.x = 1. - mouse_position_percent.x;
		}

		if mouse_position_percent.y < 0. {
			shift_percent.y = -mouse_position_percent.y;
		} else if mouse_position_percent.y > 1. {
			shift_percent.y = 1. - mouse_position_percent.y;
		}

		if shift_percent.x == 0. && shift_percent.y == 0. {
			return None;
		}

		let delta = (shift_percent * DRAG_BEYOND_VIEWPORT_SPEED_FACTOR * viewport_size).round();
		responses.add(NavigationMessage::TranslateCanvas { delta });
		Some(delta)
	}
}
