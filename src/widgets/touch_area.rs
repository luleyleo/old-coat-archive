use crate::{Component, Event, Mut, UiInput, UiUpdate};

pub struct TouchArea;

pub enum TouchAreaEvent {
    Pressed,
    Released,
    Activated,

    Entered,
    Moved,
    Exited,
}

impl Component for TouchArea {
    type State = ();
    type Props = ();
    type Msg = TouchAreaEvent;
    type Event = TouchAreaEvent;

    fn init(_: &Self::Props) -> Self::State {}

    fn update(msg: Self::Msg, _state: Mut<Self::State>, ui: &mut UiUpdate) {
        ui.emit(msg);
    }

    fn input(ui: &mut UiInput<Self>) {
        let UiInput {
            messages,
            input,
            bounds,
        } = ui;

        input.for_new_events(|event| {
            match event {
                Event::MouseInput {
                    position,
                    button: _,
                    pressed,
                } => {
                    if bounds.contains(position) {
                        if *pressed {
                            messages.send(TouchAreaEvent::Pressed);
                            return true;
                        } else {
                            messages.send(TouchAreaEvent::Released);
                            return true;
                        }
                    }
                }
                _ => (),
            }
            false
        });
    }
}
