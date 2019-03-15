use crate::{Event, MouseButton, Position, Scalar};

pub struct EventHandler {
    cursor: Position,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            cursor: Position::zero(),
        }
    }

    pub fn convert_winit_event(&mut self, input: winit::Event) -> Option<Event> {
        match input {
            winit::Event::WindowEvent { event, .. } => match event {
                winit::WindowEvent::CursorMoved { position, .. } => {
                    let position = Position::new(position.x as Scalar, position.y as Scalar);
                    self.cursor = position;
                    return Some(Event::CursorMoved { position });
                }
                winit::WindowEvent::MouseInput { state, button, .. } => {
                    let position = self.cursor;
                    let button: MouseButton = button.into();
                    let pressed = state == winit::ElementState::Pressed;
                    return Some(Event::MouseInput {
                        position,
                        button,
                        pressed,
                    });
                }
                _ => (),
            },
            _ => (),
        }

        None
    }
}

impl From<winit::MouseButton> for MouseButton {
    fn from(other: winit::MouseButton) -> MouseButton {
        match other {
            winit::MouseButton::Left => MouseButton::Left,
            winit::MouseButton::Middle => MouseButton::Middle,
            winit::MouseButton::Right => MouseButton::Right,
            winit::MouseButton::Other(n) => MouseButton::Other(n),
        }
    }
}
