/// `Button` == `ButtonProps`
pub struct Button<'a> {
    label: &'a str,
}

pub struct ButtonState {
    hovered: bool
}

pub enum ButtonMsg {
    Hover(bool),
    Touched,
}

pub enum ButtonEvent {
    Action
}

impl<'a> Component for Button<'a> {
    type State = ButtonState;
    type Msg = ButtonMsg;
    type Event = ButtonEvent;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(
            Self {
                label: ""
            }
        )
    }

    fn init(_props: &Self) -> Self::State {
        ButtonState {
            hovered: false
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
        use self::ButtonMsg::*;
        match msg {
            Hover(is_inside) => state.hovered = is_inside,
            Touched => ui.dispatch(ButtonEvent::Action),
        }
    }

    #[xml]
    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        use self::ButtonMsg::*;
       <Stack>
           <Rectangle color=BLUE />
           <Padding all=5.0>
               <Label text=props.label />
           </Padding>
           <TouchArea @Moved(e)=Hover(e.is_inside) @Touched(_)=Touched />
       </Stack>
    }
}

///////////////////////////////////////////////////////////////////////////////////////////

pub trait Component: Sized {
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new() -> PropsBuilder<Self>;

    fn init(props: &Self) -> Self::State;

    #[allow(unused_variables)]
    fn update(msg: Self::Msg, state: Mut<Self::State>, ui: &mut UiUpdate) {}

    #[allow(unused_variables)]
    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {}

    #[allow(unused_variables)]
    fn layout(state: &Self::State, children: &[Cid], constraints: BoxConstraints, ui: &mut UiLayout) -> Size {}

    #[allow(unused_variables)]
    fn input(input: &mut UiInput<Self>) {}

    #[allow(unused_variables)]
    fn derive_state(props: &Self, state: Mut<Self::State>) {}

    #[allow(unused_variables)]
    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {}
}
