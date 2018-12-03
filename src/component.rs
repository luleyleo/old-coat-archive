use crate::{
    Bounds, BoxConstraints, Cid, MsgVec, Mut, PropsBuilder, Renderer, Size, UiInput, UiLayout,
    UiUpdate, UiView,
};
use std::any::Any;

pub struct UpdateArgs<'a, 'b: 'a, Comp: Component> {
    pub msg: Comp::Msg,
    pub state: Mut<'a, Comp::State>,
    pub ui: &'a mut UiUpdate<'b>,
}

pub struct ViewArgs<'a, 'b: 'a, Comp: Component> {
    pub props: &'a Comp::Props,
    pub state: &'a Comp::State,
    pub ui: &'a mut UiView<'b>,
}

pub trait Component: Sized + 'static {
    type Props: Sized;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new<T: Component>() -> PropsBuilder<Self, T>;

    fn init_state(props: &Self::Props) -> Self::State;

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event>;

    fn view(args: ViewArgs<Self>);

    #[allow(unused_variables)]
    fn layout(constraints: BoxConstraints, children: &[Cid], ui: &mut UiLayout) -> Size {
        if children.is_empty() {
            Size::default()
        } else {
            assert_eq!(children.len(), 1);
            let child = children[0];
            ui.size(child, constraints);
            Size::default()
        }
    }

    #[allow(unused_variables)]
    fn input(ui: &UiInput) -> MsgVec<Self::Msg> {
        MsgVec::default()
    }

    #[allow(unused_variables)]
    fn derive_state(props: &Self::Props, state: Mut<Self::State>) {}

    #[allow(unused_variables)]
    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {}
}

pub(crate) trait ComponentPointerTrait: Component {
    fn pointer() -> ComponentPointer;
}

impl<C> ComponentPointerTrait for C where C: Component {
    fn pointer() -> ComponentPointer {
        ComponentPointer {
            layout: Self::layout,
        }
    }
}

pub(crate) struct ComponentPointer {
    pub layout: fn(constraints: BoxConstraints, children: &[Cid], ui: &mut UiLayout) -> Size,
}

impl Default for ComponentPointer {
    fn default() -> Self {
        ComponentPointer {
            layout: |_, _, _| panic!("Called `layout` on default `ComponentPointer`"),
        }
    }
}
