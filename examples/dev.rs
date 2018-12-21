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

    fn update(_: Self::Msg, _: &mut Mut<Self::State>, _: &mut UiUpdate) -> Option<Self::Event> { None }

    fn view(_: &Self::Props, _: &Self::State, ui: &mut UiView<Self>) {
        ids!(Background);

        Rectangle::new()
            .color(Color::rgb(0.1, 0.4, 0.1))
            .set(Background, ui);
    }
}

fn main() {
    env_logger::init();

    Window::new(DevApp)
        .title("Dev App")
        .run();
}
