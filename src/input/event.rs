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

pub struct MouseEvent {
    pub position: Position,
    pub button: MouseButton,
    pub state: ButtonState,
}

pub struct TouchEvent {
    pub position: Position,
    pub phase: TouchPhase,
    pub index: u64,
}

pub struct KeyboardEvent {
    pub scancode: u32,
    pub keycode: Option<VirtualKeyCode>,
    pub state: ButtonState,
    pub modifiers: ModifiersState,
}

pub enum Event {
    Cursor(Position),
    Mouse(MouseEvent),
    Touch(TouchEvent),
    Keyboard(KeyboardEvent),
    Character(char),

    /// There are probably going to be more events
    /// and #[not_exhaustive] is not stable yet
    SomeFutureEventPleaseUseAn_,
}
