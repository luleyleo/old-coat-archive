use crate::{
    Bounds, BoxConstraints, Cid, MsgVec, Mut, PropsBuilder, Renderer, Size, UiInput, UiLayout,
    UiUpdate, UiView,
};
use log::warn;
use std::any::Any;
use smallvec::SmallVec;

pub struct UpdateArgs<'a, 'b: 'a, 'c: 'a, Comp: Component> {
    pub msg: Comp::Msg,
    pub state: &'a mut Mut<'b, Comp::State>,
    pub ui: &'a mut UiUpdate<'c>,
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

    fn new() -> PropsBuilder<Self>;

    fn init_state(props: &Self::Props) -> Self::State;

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event>;

    fn view(args: ViewArgs<Self>);

    #[allow(unused_variables)]
    fn layout(constraints: BoxConstraints, children: &[Cid], ui: &mut UiLayout) -> Size {
        if children.is_empty() {
            Size::default()
        } else {
            if children.len() > 1 {
                let name = ui.full_debug_name();
                warn!("The default layout function is beeing applied to {} which hosts multiple children while this layout function only considers the first one", name);
            }
            let child = children[0];
            ui.size(child, constraints)
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
    fn dyn_render(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer);
    fn dyn_update(messages: &mut Box<Any>, state: &mut Box<Any>, ui: &mut UiUpdate);
}

impl<C> ComponentPointerTrait for C where C: Component {
    fn pointer() -> ComponentPointer {
        ComponentPointer {
            layout: Self::layout,
            render: Self::dyn_render,
        }
    }

    fn dyn_render(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer) {
        let state: &Self::State = state.downcast_ref().unwrap();
        Self::render(state, bounds, renderer);
    }

    fn dyn_update(messages: &mut Box<Any>, state: &mut Box<Any>, ui: &mut UiUpdate) {
        let messages: &mut Vec<Self::Msg> = messages.downcast_mut().unwrap();
        let state: &mut Self::State = state.downcast_mut().unwrap();
        let mut state = Mut::new(state);
        let mut events: SmallVec<[Self::Event; 5]> = SmallVec::new();
        for msg in messages.drain(..) {
            if let Some(event) = Self::update(UpdateArgs {
                msg,
                state: &mut state,
                ui,
            }) {
                events.push(event);
            }
        }
        // TODO: Do something with `events`
        if state.mutated() {
            // TODO: Update UI
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ComponentPointer {
    pub layout: fn(constraints: BoxConstraints, children: &[Cid], ui: &mut UiLayout) -> Size,
    pub render: fn(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer),
}

impl Default for ComponentPointer {
    fn default() -> Self {
        ComponentPointer {
            layout: |_, _, _| panic!("Called `layout` on default `ComponentPointer`"),
            render: |_, _, _| panic!("Called `render` on default `ComponentPointer`"),
        }
    }
}
