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
        iids!(FirstRect, SecondRect, InnerRect, HelloText);

        Linear::new()
            .horizontal()
            .spacing(10.0)
            .set(iid!(), ui)
            .add(|| {
                Constrained::new()
                    .max_width(200.0)
                    .set(iid!(), ui)
                    .add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.1, 0.4, 0.1))
                            .set(FirstRect, ui);
                    });

                Stack::new()
                    .set(iid!(), ui)
                    .add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.4, 0.1, 0.1))
                            .set(SecondRect, ui);
                        
                        Padding::new()
                            .all(50.0)
                            .set(iid!(), ui)
                            .add(|| {
                                Stack::new()
                                    .set(iid!(), ui)
                                    .add(|| {
                                        Rectangle::new()
                                            .color(Color::rgb(0.1, 0.1, 0.4))
                                            .set(InnerRect, ui);

                                        Text::new()
                                            .content("Hello world!")
                                            .size(14)
                                            .set(HelloText, ui);
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
