use crate::{Position, VirtualKeyCode};

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
pub enum ButtonState {
    Pressed,
    Released,
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
    // TODO: Think of a better name (MouseClick?)
    MouseInput {
        position: Position,
        button: MouseButton,
        pressed: bool,
    },
    Touch {
        position: Position,
        phase: TouchPhase,
        index: u64,
    },
    CharacterInput(char),
    Keyboard {
        scancode: u32,
        state: ButtonState,
        keycode: VirtualKeyCode,
        modifiers: ModifiersState,
    },
    SomeFutureEventPleaseUseAn_,
}
