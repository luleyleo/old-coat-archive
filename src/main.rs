include!("lib.rs");

struct DevApp;

impl Component for DevApp {
    type Props = AppProps;
    type State = ();
    type Msg = ();
    type Event = AppEvent;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(AppProps::default())
    }

    fn init_state(props: &Self::Props) -> Self::State {}

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event> { None }

    fn view(args: ViewArgs<Self>) {
        ids!(Background);

        Rectangle::new()
            .color(Color::rgb(0.1, 0.4, 0.1))
            .set(Background, args.ui);
    }
}

fn main() {
    Window::new(DevApp)
        .title("Dev App")
        .run();
}
