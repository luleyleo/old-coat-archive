use crate::{
    iid, ButtonState, Component, Event, Mut, TouchArea, TouchAreaEvent, UiInput, UiUpdate, UiView,
    VirtualKeyCode,
};

#[derive(Default)]
pub struct TextInputArea;

pub struct TextInputAreaState {
    // TODO: Try to evaluate if this is good enough
    //       If it is, then there would be no need for a
    //       centalized "focused_component" or such.
    focused: bool,
}

#[derive(Debug)]
pub enum TextInputAreaMsg {
    Focus,
    Unfocus,
    Edit(TextInputAreaEvent),
}

#[derive(Debug)]
pub enum TextInputAreaEvent {
    Add(char),
    Backspace,
    Delete,
}

impl Component for TextInputArea {
    type State = TextInputAreaState;
    type Msg = TextInputAreaMsg;
    type Event = TextInputAreaEvent;

    fn init(_: &Self) -> Self::State {
        TextInputAreaState { focused: false }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
        use TextInputAreaMsg::*;
        match msg {
            Focus => state.focused = true,
            Unfocus => state.focused = false,
            Edit(event) => {
                if state.focused {
                    ui.emit(event);
                }
            }
        }
    }

    fn view(_props: &Self, _state: &Self::State, ui: &mut UiView<Self>) {
        use TextInputAreaMsg::*;
        TouchArea::new()
            .set(iid!(), ui)
            .on_event(ui, |event| match event {
                TouchAreaEvent::Pressed(_) => Some(Focus),
                _ => None,
            });
    }

    fn input(ui: &mut UiInput<Self>) {
        for (event, _) in ui.input.iter_all_events() {
            match event {
                // TODO: Should pressing ESC also unfocus?
                Event::MouseInput { position, .. } | Event::Touch { position, .. } => {
                    if !ui.bounds.contains(position) {
                        ui.messages.send(TextInputAreaMsg::Unfocus);
                    }
                }
                _ => (),
            }
        }

        for (event, consumed) in ui.input.iter_fresh_events() {
            use TextInputAreaEvent::*;
            use TextInputAreaMsg::Edit;

            match event {
                Event::CharacterInput(c) => {
                    if !c.is_control() {
                        ui.messages.send(Edit(Add(*c)));
                        *consumed = true;
                    }
                }
                Event::Keyboard { state, keycode, .. } => {
                    if state == &ButtonState::Pressed {
                        match keycode {
                            Some(VirtualKeyCode::Backspace) => {
                                ui.messages.send(Edit(Backspace));
                                *consumed = true;
                            }
                            Some(VirtualKeyCode::Delete) => {
                                ui.messages.send(Edit(Delete));
                                *consumed = true;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
