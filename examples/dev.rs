#![allow(dead_code)]

use coat::backend::winit::{AppEvent, Window};
use coat::*;

struct DevApp;

#[derive(Default)]
struct Props;
impl Properties for Props {
    type Component = DevApp;
}

struct State {
    hellos: usize,
}

enum Msg {
    SayHello,
}

impl Component for DevApp {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = AppEvent;

    fn init(_props: &Self::Props) -> Self::State {
        Self::State { hellos: 0 }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, _ui: &mut UiUpdate) {
        match msg {
            Msg::SayHello => {
                state.hellos += 1;
                println!("{}th hello!", state.hellos);
            }
        }
    }

    fn view(_: &Self::Props, _: &Self::State, ui: &mut UiView<Self>) {
        iids!(FirstRect, SecondRect, InnerRect, HelloText);

        Linear::new()
            .horizontal()
            .spacing(10.0)
            .set(iid!(), ui)
            .add(|| {
                Constrained::new().max_width(200.0).set(iid!(), ui).add(|| {
                    Rectangle::new()
                        .color(Color::rgb(0.3, 0.7, 0.3))
                        .set(FirstRect, ui);
                });

                Stack::new().set(iid!(), ui).add(|| {
                    Rectangle::new()
                        .color(Color::rgb(0.7, 0.3, 0.3))
                        .set(SecondRect, ui);

                    Padding::new().all(50.0).set(iid!(), ui).add(|| {
                        TouchArea::new()
                            .set(iid!(), ui)
                            .on(ui, hello_handler)
                            .add(|| {
                                Stack::new().set(iid!(), ui).add(|| {
                                    Rectangle::new()
                                        .color(Color::rgb(0.3, 0.3, 0.7))
                                        .set(InnerRect, ui);

                                    Text::new()
                                        .content("Hello world!")
                                        .size(14)
                                        .set(HelloText, ui);
                                });
                            });
                    });
                });
            });
    }
}

fn hello_handler(event: TouchAreaEvent) -> Option<Msg> {
    use TouchAreaEvent::*;
    match event {
        Entered => (None, println!("Click now!")).0,
        Activated(MouseButton::Left) => Some(Msg::SayHello),
        _ => None,
    }
}

fn main() {
    env_logger::init();

    Window::new().title("Dev App").run::<DevApp>();
}
