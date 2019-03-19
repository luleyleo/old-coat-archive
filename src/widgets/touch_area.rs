use crate::{Component, Event, Properties, MouseButton, Mut, Position, UiInput, UiUpdate};

pub struct TouchArea;

#[derive(Default)]
pub struct TouchAreaProps;
impl Properties for TouchAreaProps {
    type Component = TouchArea;
}

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
    type Props = TouchAreaProps;
    type State = TouchAreaState;
    type Msg = TouchAreaEvent;
    type Event = TouchAreaEvent;

    fn init(_: &Self::Props) -> Self::State {
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
                    ui.emit(msg);
                } else {
                    if state.inside {
                        state.inside = false;
                    }
                    ui.emit(Exited);
                }
            }
            _ => (),
        }
    }

    fn input(ui: &mut UiInput<Self>) {
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
