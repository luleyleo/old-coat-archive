
struct If;

struct Props {
    pub predicate: bool
}

impl PropsBuilder<If> {
    pub fn pred(mut self, value: bool) -> Self {
        self.predicate = value;
        self
    }
}

impl Component for If {

    type Props = Props;
    type State = ();
    type  Msg  = ();
    type Event = ();

    fn new() -> PropsBuilder<Self> {
        Props {
            predicate: false
        }.into()
    }

    fn init_state(props: Props) -> Self::State {}

    fn update(
        _msg: Self::Msg,
        _props: Self::Props,
        _state: &mut Self::State,
    ) -> Option<Self::Event> { None }

    fn view(props: Self::Props, _state: &Self::State, ui: &mut UI) {
        if props.predicate {
            Children::new()
                .of(ui.this()) // set by default
                .parent(ui.this()) // set by default
                .set(0, ui);
        }
    }

}
