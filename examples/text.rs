use coat::*;
use coat::backend::winit::{AppEvent, Window};

#[derive(Default)]
struct App;

impl Component for App {
    type State = ();
    type Msg = ();
    type Event = AppEvent;

    fn init(_: &Self) -> Self::State {}

    fn view(_: &Self, _: &Self::State, ui: &mut UiView<Self>) {
        Padding::new().all(10.0).set(iid!(), ui).add(|| {
            Text::new()
                .content(LOREM)
                .set(iid!(), ui);
        });
    }
}

fn main() {
    env_logger::init();

    Window::new().title("dev: Text").run::<App>();
}

static LOREM: &str = "Lorem ipsum dolor sit amet, consectetuer adipiscing elit.  Donec hendrerit tempor tellus.  Donec pretium posuere tellus.  Proin quam nisl, tincidunt et, mattis eget, convallis nec, purus.  Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.  Nulla posuere.  Donec vitae dolor.  Nullam tristique diam non turpis.  Cras placerat accumsan nulla.  Nullam rutrum.  Nam vestibulum accumsan nisl.";

