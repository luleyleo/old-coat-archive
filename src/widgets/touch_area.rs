use crate::{Component, Event, MouseButton, Mut, UiInput, UiUpdate};

pub struct TouchArea;

pub enum TouchAreaEvent {
    Pressed(MouseButton),
    Released(MouseButton),
    Activated(MouseButton),

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
        for (event, handled) in ui.input.iter_new_events() {
            match event {
                Event::MouseInput { position, button, pressed} => {
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
                _ => (),
            }
        }
    }
}
