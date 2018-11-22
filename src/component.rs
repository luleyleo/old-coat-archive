use std::any::Any;
use crate::{Ui, BoxConstraints, WidgetId, Size, PropsBuilder, MsgVec};

pub struct UpdateArgs<'a, Comp: Component> {
    pub msg: Comp::Msg,
    pub props: Comp::Props,
    pub state: &'a mut Comp::State,
}

pub struct ViewArgs<'a, Comp: Component> {
    pub props: &'a Comp::Props,
    pub state: &'a Comp::State,
    pub ui: &'a mut Ui,
}

pub trait Component: Sized {
    type Props: Sized + 'static;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new<T: Component>() -> PropsBuilder<Self, T>;

    fn init_state(props: Self::Props) -> Self::State;

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event>;

    fn view(args: ViewArgs<Self>);

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

    fn input(ui: &Ui) -> MsgVec<Self::Msg>;
}

pub(crate) trait DynComponent {
    fn view(props: &Box<Any>, state: &mut Box<Any>, ui: &mut Ui);
    fn input(ui: &Ui);
}

impl<Comp> DynComponent for Comp
where
    Comp: Component
{
    fn view(props: &Box<Any>, state: &mut Box<Any>, ui: &mut Ui) {
        let props: &Comp::Props = props.downcast_ref().unwrap();
        let state: &mut Comp::State = state.downcast_mut().unwrap();
        let args = ViewArgs { props, state, ui };
        Comp::view(args);
    }

    fn input(ui: &Ui) {

    }
}

pub struct ComponentPointer {
    view: fn(props: &Box<Any>, state: &mut Box<Any>, ui: &mut Ui),
    layout: fn(constraints: BoxConstraints, children: &[WidgetId], ui: &mut Ui) -> Size,
}
