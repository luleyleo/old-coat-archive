use crate::{Component, Event, MouseButton, Mut, Position, UiInput, UiUpdate};

#[derive(Default)]
pub struct TouchArea;

pub struct TouchAreaState {
    pressed: bool,
    inside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchAreaEvent {
    Pressed(MouseButton),
    Released(MouseButton),
    Activated(MouseButton),

    Entered,
    Moved(Option<Position>),
    Exited,
}

impl Component for TouchArea {
    type State = TouchAreaState;
    type Msg = TouchAreaEvent;
    type Event = TouchAreaEvent;

    fn init(_: &Self) -> Self::State {
        TouchAreaState {
            pressed: false,
            inside: false,
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
        use TouchAreaEvent::*;
        match msg {
            Pressed(_) => {
                state.pressed = true;
                ui.emit(msg);
            }
            Released(button) => {
                ui.emit(msg);
                if state.pressed {
                    state.pressed = false;
                    ui.emit(Activated(button))
                }
            }
            Moved(position) => {
                if position.is_some() {
                    if !state.inside {
                        state.inside = true;
                        ui.emit(Entered);
                    }
                } else {
                    if state.inside {
                        state.inside = false;
                        ui.emit(Exited);
                    }
                }
                ui.emit(msg);
            }
            _ => (),
        }
    }

    fn input(ui: &mut UiInput<Self>) {
        // TODO: This should also emit an `Exited` event for already handled events
        for (event, handled) in ui.input.iter_new_events() {
            match event {
                Event::MouseInput {
                    position,
                    button,
                    pressed,
                } => {
                    if ui.bounds.contains(position) {
                        if *pressed {
                            ui.messages.send(TouchAreaEvent::Pressed(*button));
                            *handled = true;
                        } else {
                            ui.messages.send(TouchAreaEvent::Released(*button));
                            *handled = true;
                        }
                    }
                }
                Event::CursorMoved { position } => {
                    let position = if ui.bounds.contains(position) {
                        Some(*position)
                    } else {
                        None
                    };
                    ui.messages.send(TouchAreaEvent::Moved(position));
                }
                _ => (),
            }
        }
    }
}
