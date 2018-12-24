use coat::*;

struct DevApp;

impl Component for DevApp {
    type Props = AppProps;
    type State = ();
    type Msg = ();
    type Event = AppEvent;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(AppProps::default())
    }

    fn init_state(_props: &Self::Props) -> Self::State {}

    fn view(_: &Self::Props, _: &Self::State, ui: &mut UiView<Self>) {
        ids!(Container, Limit, First, Second);

        Linear::new()
            .horizontal()
            .spacing(10.0)
            .set(Container, ui)
            .add(|| {
                Constrained::new()
                    .max_width(200.0)
                    .set(Limit, ui)
                    .add(|| {
                        Rectangle::new()
                            .color(Color::rgb(0.1, 0.4, 0.1))
                            .set(First, ui);
                    });

                Rectangle::new()
                    .color(Color::rgb(0.4, 0.1, 0.1))
                    .set(Second, ui);
            });
    }
}

fn main() {
    env_logger::init();

    Window::new(DevApp)
        .title("Dev App")
        .run();
}
