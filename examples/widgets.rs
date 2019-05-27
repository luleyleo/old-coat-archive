use coat::backend::winit::{AppEvent, Window};
use coat::*;

fn main() {
    env_logger::init();

    Window::new().title("Widgets").run::<App>();
}

//========================//
//     App Component      //
//========================//

#[derive(Default)]
struct App;

#[derive(PartialEq, Eq)]
enum ActiveButton {
    First,
    Second,
}

struct State {
    active: ActiveButton,
}

enum Msg {
    Activate(ActiveButton),
}

impl Component for App {
    type State = State;
    type Msg = Msg;
    type Event = AppEvent;

    fn init(_props: &Self) -> Self::State {
        State {
            active: ActiveButton::First,
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<State>, _ui: &mut UiUpdate) {
        match msg {
            Msg::Activate(button) => {
                state.active = button;
            }
        }
    }

    fn view(_props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        Padding::new().all(10.0).set(iid!(), ui).add(|| {
            Linear::new()
                .vertical()
                .spacing(10.0)
                .set(iid!(), ui)
                .add(|| {
                    Constrained::new().max_height(185.0).set(iid!(), ui).add(|| {
                        Button::new()
                            .label("Hallo!")
                            .enabled(state.active == ActiveButton::First)
                            .set(iid!(), ui)
                            .on_event(ui, |_| Some(Msg::Activate(ActiveButton::Second)));
                    });

                    Button::new()
                        .label("Welt!")
                        .enabled(state.active == ActiveButton::Second)
                        .set(iid!(), ui)
                        .on_event(ui, |_| Some(Msg::Activate(ActiveButton::First)));
                });
        });
    }
}

//========================//
//    Button Component    //
//========================//

#[derive(Default)]
struct Button<'a> {
    label: &'a str,
    enabled: bool,
}
impl<'a> Button<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

struct ButtonState {
    hovered: bool,
    pressed: bool,
}

enum ButtonMsg {
    Hovered(bool),
    Pressed(bool),
    Activated,
}

enum ButtonEvent {
    Activated,
}

impl<'a> Component for Button<'a> {
    type State = ButtonState;
    type Msg = ButtonMsg;
    type Event = ButtonEvent;

    fn init(_props: &Self) -> Self::State {
        ButtonState {
            hovered: false,
            pressed: false,
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
        match msg {
            ButtonMsg::Activated => {
                ui.emit(ButtonEvent::Activated);
            }
            ButtonMsg::Hovered(hovered) => {
                if hovered != state.hovered {
                    state.hovered = hovered;
                }
            }
            ButtonMsg::Pressed(pressed) => {
                if pressed != state.pressed {
                    state.pressed = pressed;
                }
            }
        }
    }

    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        let background = match 0 {
            _ if !props.enabled => Color::rgb(0.3, 0.3, 0.3),
            _ if state.pressed => Color::rgb(0.2, 0.2, 0.6),
            _ if state.hovered => Color::rgb(0.4, 0.4, 0.8),
            _ => Color::rgb(0.3, 0.3, 0.7),
        };

        TouchArea::new()
            .set(iid!(), ui)
            .on_event(ui, |e| match e {
                TouchAreaEvent::Moved(pos) => Some(ButtonMsg::Hovered(pos.is_some())),
                TouchAreaEvent::Pressed(_) => Some(ButtonMsg::Pressed(true)),
                TouchAreaEvent::Released(_) => Some(ButtonMsg::Pressed(false)),
                TouchAreaEvent::Activated(_) => match props.enabled {
                    true => Some(ButtonMsg::Activated),
                    false => None,
                },
                _ => None,
            })
            .add(|| {
                Stack::new().set(iid!(), ui).add(|| {
                    Rectangle::new().color(background).set(iid!(), ui);

                    Padding::new().all(5.0).set(iid!(), ui).add(|| {
                        Text::new().content(props.label).size(14).set(iid!(), ui);
                    });
                });
            });
    }
}
