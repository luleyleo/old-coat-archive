#![allow(dead_code)]

use coat::backend::winit::{AppEvent, Window};
use coat::layouts::*;
use coat::widgets::*;
use coat::{
    iid, iids, Buffer, BufferUpdate, Color, Component, MouseButton, Mut, Size, UiUpdate, UiView,
};

#[derive(Default)]
struct DevApp;

struct State {
    hellos: usize,
    hovered: bool,
    text: Buffer,
}

enum Msg {
    SayHello,
    Active(bool),
    Edit(BufferUpdate),
}

impl Component for DevApp {
    type State = State;
    type Msg = Msg;
    type Event = AppEvent;

    fn init(_props: &Self) -> Self::State {
        Self::State {
            hellos: 0,
            hovered: false,
            text: Buffer::from("Woop!".to_owned()),
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, _ui: &mut UiUpdate) {
        match msg {
            Msg::SayHello => {
                state.hellos += 1;
                println!("{}th: {}", state.hellos, state.text.text());
            }
            Msg::Active(active) => {
                state.hovered = active;
            }
            Msg::Edit(update) => {
                state.text.update(update);
            }
        }
    }

    fn view(_: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        iids!(ButtonWrap, InnerRect, HelloText);

        let the_color = if state.hovered {
            Color::rgb(0.4, 0.4, 0.8)
        } else {
            Color::rgb(0.3, 0.3, 0.7)
        };

        Linear::new()
            .horizontal()
            .spacing(10.0)
            .set(iid!(), ui)
            .add(|| {
                Constrained::new().max_width(400.0).set(iid!(), ui).add(|| {
                    Stack::new().set(iid!(), ui).add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.7, 0.3, 0.7))
                            .set(iid!(), ui);

                        let btn_size = Size::new(100.0, 30.0);
                        Constrained::new().max(btn_size).set(iid!(), ui).add(|| {
                            Stack::new().set(iid!(), ui).add(|| {
                                Rectangle::new()
                                    .color(Color::rgb(0.3, 0.7, 0.3))
                                    .set(iid!(), ui);

                                TextEdit::new()
                                    .buffer(&state.text)
                                    .size(14)
                                    .set(iid!(), ui)
                                    .on_event(ui, |event| Some(Msg::Edit(event)));
                            });
                        });
                    });
                });

                Stack::new().set(iid!(), ui).add(|| {
                    Rectangle::new()
                        .color(Color::rgb(0.7, 0.3, 0.3))
                        .set(iid!(), ui);

                    Padding::new().all(20.0).set(iid!(), ui).add(|| {
                        TouchArea::new()
                            .set(ButtonWrap, ui)
                            .on_event(ui, hello_handler)
                            .add(|| {
                                Stack::new().set(iid!(), ui).add(|| {
                                    Rectangle::new().color(the_color).set(InnerRect, ui);

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
        Entered => Some(Msg::Active(true)),
        Exited => Some(Msg::Active(false)),
        Activated(MouseButton::Left) => Some(Msg::SayHello),
        _ => None,
    }
}

fn main() {
    env_logger::init();

    Window::new().title("Dev App").run::<DevApp>();
}
