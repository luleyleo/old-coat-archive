struct Switcher;

struct SwitcherProps<'a> {
    options: &'a[&'a str],
    active: usize,
    closeable: bool,
}

impl PropsBuilder<Switcher> {
    fn options(...);
    fn active(...);
}

enum SwitcherMsg {
    Activate(usize),
    Close(usize),
}

type SwitcherEvent = SwitcherMsg;

impl Component for Switcher {
    type Props = SwitcherProps;
    type State = ();
    type Msg   = SwitcherMsg;
    type Event = SwitcherEvent;

    fn update(msg: Self::Msg, state: &mut Self::State, ui: &mut UiUpdate) {
        ui.fire(msg);
    }

    fn view(props: &Self::Props, state: &Self::State, ui: &mut UiView<Self>) {
        Linear::new()
            .horizontal()
            .spacing(5.0)
            .set(iid!(), ui)
            .add(|| {
                for (index, option) in props.options.iter().enumerate() {
                    Button::new()
                        .key(option) // either any `T` like this
                        .key(index)  // or only `usize`
                        .text(option)
                        .on_action(|e| Self::Msg::Activate(index))
                        .set(iid!(), ui);
                }
            });
    }
}
