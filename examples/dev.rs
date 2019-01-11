use coat::*;
use coat::backend::winit::{Window, AppProps, AppEvent};

struct DevApp;

impl Component for DevApp {
    type Props = AppProps;
    type State = ();
    type Msg = ();
    type Event = AppEvent;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(AppProps::default())
    }

    fn init(_props: &Self::Props) -> Self::State {}

    fn view(_: &Self::Props, _: &Self::State, ui: &mut UiView<Self>) {
        ids!(container, limit, first, second, stack, pad, inner, stack2, text);

        Linear::new()
            .horizontal()
            .spacing(10.0)
            .set(container, ui)
            .add(|| {
                Constrained::new()
                    .max_width(200.0)
                    .set(limit, ui)
                    .add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.1, 0.4, 0.1))
                            .set(first, ui);
                    });

                Stack::new()
                    .set(stack, ui)
                    .add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.4, 0.1, 0.1))
                            .set(second, ui);
                        
                        Padding::new()
                            .all(50.0)
                            .set(pad, ui)
                            .add(|| {
                                Stack::new()
                                    .set(stack2, ui)
                                    .add(|| {
                                        Rectangle::new()
                                            .color(Color::rgb(0.1, 0.1, 0.4))
                                            .set(inner, ui);

                                        Text::new()
                                            .content("Hello world!")
                                            .size(14)
                                            .set(text, ui);
                                    });
                            });
                    });
            });
    }
}

fn main() {
    env_logger::init();

    Window::new()
        .title("Dev App")
        .run::<DevApp>();
}
