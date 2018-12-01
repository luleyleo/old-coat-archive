
include!("lib.rs");

struct DevApp;

impl Component for DevApp {
    type Props = AppProps;
    type State = ();
    type Msg = ();
    type Event = AppEvent;

    fn new<T: Component>() -> PropsBuilder<Self, T> {
        PropsBuilder::new(AppProps::default())
    }

    fn init_state(props: &Self::Props) -> Self::State {}

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event> { None }

    fn view(args: ViewArgs<Self>) {}
}

fn main() {
    Window::new(DevApp)
        .title("Dev App")
        .run();
}
