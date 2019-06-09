use crate::{Component, FocusState, KeyboardEvent, Mut, UiDerive, UiInput, UiUpdate};

pub struct KeyArea {
    filter: KeyAreaFilter,
}

impl Default for KeyArea {
    fn default() -> Self {
        KeyArea { filter: |_| false }
    }
}

impl KeyArea {
    /// This defines what events are interesting. Doing this is necessary in
    /// order to avoid consuming all keyboard event, regardless of whether or
    /// not they are actually being handled. The filter should return `true` for
    /// all events that should be consumed.
    pub fn filter(mut self, filter: KeyAreaFilter) -> Self {
        self.filter = filter;
        self
    }
}

pub type Interest = bool;
pub type KeyAreaFilter = fn(event: &KeyboardEvent) -> Interest;

pub struct KeyAreaState {
    filter: KeyAreaFilter,
}

#[derive(Debug)]
pub enum KeyAreaMsg {
    Focus,
    Unfocus,
    Key(KeyboardEvent),
    Text(char),
}

#[derive(Debug)]
pub enum KeyAreaEvent {
    Focus(bool),
    Key(KeyboardEvent),
    Text(char),
}

impl Component for KeyArea {
    type State = KeyAreaState;
    type Msg = KeyAreaMsg;
    type Event = KeyAreaEvent;

    fn init(props: &Self) -> Self::State {
        KeyAreaState {
            filter: props.filter,
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, _ui: &mut UiDerive<Self>) {
        state.filter = props.filter;
    }

    fn update(msg: Self::Msg, _state: Mut<Self::State>, ui: &mut UiUpdate) {
        use KeyAreaMsg::*;
        match msg {
            Focus => {
                ui.aquire_focus();
                ui.emit(KeyAreaEvent::Focus(true));
            }
            Unfocus => {
                ui.lose_focus();
                ui.emit(KeyAreaEvent::Focus(false));
            }
            Key(event) => {
                ui.emit(KeyAreaEvent::Key(event));
            }
            Text(ch) => {
                ui.emit(KeyAreaEvent::Text(ch));
            }
        }
    }

    fn input(state: &Self::State, ui: &mut UiInput<Self>) {
        use crate::{Event, MouseEvent, TouchEvent};

        let focused = ui.focus_state() == FocusState::Owns;

        for (event, _) in ui.input.iter_all_events() {
            match event {
                // TODO: Should pressing ESC also unfocus?
                Event::Mouse(MouseEvent { position, .. })
                | Event::Touch(TouchEvent { position, .. }) => {
                    if ui.bounds.contains(position) {
                        if !focused {
                            ui.messages.send(KeyAreaMsg::Focus);
                        }
                    } else if focused {
                        ui.messages.send(KeyAreaMsg::Unfocus);
                    }
                }
                _ => (),
            }
        }

        if focused {
            for (event, consumed) in ui.input.iter_fresh_events() {
                match event {
                    Event::Character(ch) => {
                        // TODO: Are there other categories that should be excluded?
                        if !ch.is_control() {
                            ui.messages.send(KeyAreaMsg::Text(*ch));
                            *consumed = true;
                        }
                    }
                    Event::Keyboard(event) => {
                        if (state.filter)(event) {
                            ui.messages.send(KeyAreaMsg::Key(*event));
                            *consumed = true;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
