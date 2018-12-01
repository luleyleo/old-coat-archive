use crate::{Ui, BoxConstraints, WidgetId, Size, PropsBuilder, MsgVec, Mut, Renderer};

pub struct UpdateArgs<'a, Comp: Component> {
    pub msg: Comp::Msg,
    pub state: Mut<'a, Comp::State>,
    pub ui: &'a mut Ui,
}

pub struct ViewArgs<'a, Comp: Component> {
    pub props: &'a Comp::Props,
    pub state: &'a Comp::State,
    pub ui: &'a mut Ui,
}

pub trait Component: Sized {
    type Props: Sized;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new<T: Component>() -> PropsBuilder<Self, T>;

    fn init_state(props: &Self::Props) -> Self::State;

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event>;

    fn view(args: ViewArgs<Self>);

    #[allow(unused_variables)]
    fn layout(constraints: BoxConstraints, children: &[WidgetId], ui: &mut Ui) -> Size {
        if children.is_empty() {
            Size::default()
        } else {
            assert_eq!(children.len(), 1);
            let child = children[0];
            //ui.size(child, constraints)
            Size::default()
        }
    }

    fn input(ui: &Ui) -> MsgVec<Self::Msg> { MsgVec::default() }

    fn derive_state(props: &Self::Props, state: Mut<Self::State>) {}

    #[allow(unused_variables)]
    fn render(state: &Self::State, renderer: Renderer) {}
}
