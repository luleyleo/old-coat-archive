use crate::Component;

//mod eventloop;
//mod notifier;
//mod webrender;

pub enum AppEvent {
    SetTitle(String),
    Quit,
}

pub struct Window<State, Msg, Comp>
where
    State: 'static,
    Msg: 'static,
    Comp: Component<Props=(), State=State, Msg=Msg, Event=AppEvent>
{
    title: String,
    app: Comp,
}

impl<State, Msg, Comp> Window<State, Msg, Comp>
where
    State: 'static,
    Msg: 'static,
    Comp: Component<Props=(), State=State, Msg=Msg, Event=AppEvent>
{
    pub fn new(app: Comp) -> Self {
        Window {
            title: String::new(),
            app
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn run(self) {

    }
}
