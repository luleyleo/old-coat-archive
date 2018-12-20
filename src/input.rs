use crate::Position;

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
}
