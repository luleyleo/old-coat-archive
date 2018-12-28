
struct ThemeProvider {
    .. some props ..
}

struct Theme {
    some_color: Color
}

enum ThemeProviderMsg {
    ChangeTheColor(Color)
}

impl Component for ThemeProvider {
    type State = Theme;
    type Msg = ThemeProviderMsg;
    type Event = ();

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new( .. some default props .. )
    }

    fn init(props: &Self) -> Self::State { .. }
}

///////////////////////////////////////////////////////////////////////////////////////////

struct ContextConsumer {
    .. some props ..
}

struct ContextConsumerState {
    .. some state ..
}

impl Component for ContextConsumer {
    type State = ContextConsumerState;
    type Msg = ();
    type Event = ();


    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new( .. some default props .. )
    }

    fn init(props: &Self) -> Self::State { .. }

    fn update(msg: Self::Msg, state: Mut<Self::State>, ui: &mut UiUpdate) {
        let theme = ui.context::<ThemeProvider>().unwrap();
        let new_color = theme.some_color.invert();
        ui.bubble(ThemeProviderMsg::ChangeTheColor(new_color));
    }

    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        let theme = ui.context::<ThemeProvider>().unwrap();
        let theme: Theme = ui.context().unwrap();
        ..
    }
}
