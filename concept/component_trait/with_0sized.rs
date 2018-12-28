//! This would require GATs, which currently are not even available on nightly :(
use crate::*;

pub struct Button;

pub struct ButtonProps<'a> {
    label: &'a str,
}

impl<'a> PropsBuilder<Button<'a>> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }
}

pub struct ButtonState {
    hovered: bool,
}

pub enum ButtonMsg {
    TouchMoved { inside: bool },
    Touched,
}

pub enum ButtonEvent {
    Activated,
}

impl Component for Button {
    type Props<'a> = ButtonProps<'a>;
    type State = ButtonState;
    type Msg = ButtonMsg;
    type Event = ButtonEvent;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(Self::Props { label: "" })
    }

    fn init_state(props: &Self::Props) -> Self::State {
        ButtonState { hovered: false }
    }

    fn view(props: &Self::Props, state: &Self::State, ui: &mut UiView<Self>) {}

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
