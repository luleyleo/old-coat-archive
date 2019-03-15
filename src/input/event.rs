// pub use winit::Event;
// pub use winit::ModifiersState;
// pub use winit::VirtualKeyCode as Key;

use crate::Position;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ModifiersState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

pub enum Event {
    CursorMoved {
        position: Position,
    },
    MouseInput {
        position: Position,
        button: MouseButton,
        pressed: bool,
    },
    Touch {
        phase: TouchPhase,
        position: Position,
        index: u64,
    },
}
