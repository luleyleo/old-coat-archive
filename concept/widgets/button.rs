struct Button;

struct Props {
    label: Option<Str>,
}

impl PropsBuilder<Button> {
    pub fn label(mut self, value: Str) -> Self {
        self.label = value.into();
        self
    }
}

struct State {
    hovered: bool,
}

enum Msg {
    TouchMoved { inside: bool },
    Touched,
}

enum Event {
    Action { count: usize },
}

impl Component for Button {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = Event;

    fn new<T: Component>() -> PropsBuilder<Self, T> {
        Props { label: None }.into()
    }

    fn init_state(props: Props) -> State {
        State { hovered: false }
    }

    fn update(msg: Msg, props: Props, state: &mut State) -> Option<Event> {
        match msg {
            Msg::TouchMoved { inside } => {
                state.hovered = inside;
            }
            Msg::Action(a) => {
                return Some(Event::Action(a));
            }
        }
    }

    fn view(props: Props, state: &State, ui: &mut UI) {
        let background = if state.hovered { BLUE } else { LIGHT_BLUE };

        xml! {
            <Stack>
                <Grow>
                    <Rectangle color=background />
                </Grow>
                <Children />
                <Padding all=5.0>
                    <Label text=props.label/>
                </Padding>
                <Grow>
                    <TouchArea
                        @Moved(e)=Msg::Moved(e.pos.is_some())
                        @Touched(_)=Msg::Touched/>
                </Grow>
            </Stack>
        }
    }
}
