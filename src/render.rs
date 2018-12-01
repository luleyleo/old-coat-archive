use crate::Component;

mod eventloop;
mod notifier;
mod webrender;

#[derive(Default)]
pub struct AppProps;

pub enum AppEvent {
    SetTitle(String),
    Quit,
}

pub struct Window<State, Msg, Comp>
where
    State: 'static,
    Msg: 'static,
    Comp: Component<Props=AppProps, State=State, Msg=Msg, Event=AppEvent>
{
    title: String,
    app: std::marker::PhantomData<Comp>,
}

impl<State, Msg, Comp> Window<State, Msg, Comp>
where
    State: 'static,
    Msg: 'static,
    Comp: Component<Props=AppProps, State=State, Msg=Msg, Event=AppEvent> + 'static
{
    #[allow(unused_variables)]
    pub fn new(app: Comp) -> Self {
        Window {
            title: String::new(),
            app: std::marker::PhantomData,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn run(self) {
        self::webrender::run(self);
    }
}
