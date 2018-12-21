use crate::{Scalar, Position};

mod event;
pub use self::event::*;

pub struct Input {
    pub(crate) pointer: Position,
    pub(crate) events: Vec<(Event, bool)>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            pointer: Position::default(),
            events: Vec::new(),
        }
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        match &event {
            winit::Event::WindowEvent { event, .. } => {
                match event {
                    winit::WindowEvent::CursorMoved { position, .. } => {
                        self.pointer = Position::new(position.x as Scalar, position.y as Scalar);
                    }
                    _ => ()
                }
            }
            _ => ()
        };
        self.events.push((event, false));
    }
}
