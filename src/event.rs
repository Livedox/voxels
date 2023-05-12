use crate::{input::{InputEvent, self}};
use glium::glutin::{self, event, event::Event, event::WindowEvent, event::DeviceEvent};


pub fn match_event(event: Event<()>, input_event: &mut InputEvent) -> Option<glutin::event_loop::ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                return Some(glutin::event_loop::ControlFlow::Exit);
            },
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(code) = input.virtual_keycode {
                    input_event.press_key(code as usize,
                      if input.state == event::ElementState::Pressed {input::State::Jpressed}
                      else {input::State::Jreleased}
                    );
                }
                None
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let id: Option<usize> = match button {
                    event::MouseButton::Left => Some(0),
                    event::MouseButton::Right => Some(1),
                    event::MouseButton::Middle => Some(2),
                    event::MouseButton::Other(_) => None,
                };
                if let Some(id) = id {
                    input_event.press_mouse(id,
                      if state == glutin::event::ElementState::Pressed {input::State::Jpressed}
                      else {input::State::Jreleased}
                    );
                }
                None
            },
            _ => None,
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                input_event.set_delta(delta.0 as f32, delta.1 as f32);
                None
            },
            _ => None,
        },
        _ => None,
    }
}