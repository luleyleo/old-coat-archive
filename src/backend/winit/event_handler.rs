use crate::{Event, KeyboardEvent, MouseButton, MouseEvent, Position, Scalar};

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
                    return Some(Event::Cursor(position));
                }
                winit::WindowEvent::MouseInput { state, button, .. } => {
                    let position = self.cursor;
                    let button: MouseButton = button.into();
                    let state = unsafe { std::mem::transmute(state) };
                    return Some(Event::Mouse(MouseEvent {
                        position,
                        button,
                        state,
                    }));
                }
                winit::WindowEvent::KeyboardInput { input, .. } => {
                    // TODO: Those are the same .... for now
                    let state = unsafe { std::mem::transmute(input.state) };
                    let keycode = unsafe { std::mem::transmute(input.virtual_keycode) };
                    let modifiers = unsafe { std::mem::transmute(input.modifiers) };

                    return Some(Event::Keyboard(KeyboardEvent {
                        scancode: input.scancode,
                        state,
                        keycode,
                        modifiers,
                    }));
                }
                winit::WindowEvent::ReceivedCharacter(c) => {
                    return Some(Event::Character(c));
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
