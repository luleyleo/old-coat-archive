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

    fn input(_state: &Self::State, ui: &mut UiInput<Self>) {
        use crate::{ButtonState, MouseEvent, TouchEvent};

        for event in ui.input.iter_spoiled_events() {
            match event {
                Event::Mouse(MouseEvent { position, .. })
                | Event::Cursor(position)
                | Event::Touch(TouchEvent { position, .. }) => {
                    if ui.bounds.contains(position) {
                        // Something else has been interacted with :(
                        // This can happen (theoretically) when another
                        // `TouchArea` appears above this one.
                        ui.messages.send(TouchAreaEvent::Exited);
                    }
                }
                _ => (),
            }
        }
        for (event, handled) in ui.input.iter_fresh_events() {
            match event {
                Event::Mouse(MouseEvent {
                    position,
                    button,
                    state,
                }) => {
                    if ui.bounds.contains(position) {
                        if *state == ButtonState::Pressed {
                            ui.messages.send(TouchAreaEvent::Pressed(*button));
                        } else {
                            ui.messages.send(TouchAreaEvent::Released(*button));
                        }
                        *handled = true;
                    }
                }
                Event::Cursor(position) => {
                    let position = if ui.bounds.contains(position) {
                        *handled = true;
                        Some(*position)
                    } else {
                        None
                    };
                    ui.messages.send(TouchAreaEvent::Moved(position));
                }
                // TODO: Implement `Event::Touch` handling
                _ => (),
            }
        }
    }
}
