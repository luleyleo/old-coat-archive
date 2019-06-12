use crate::{
    Bounds, BoxConstraints, Cid, ContentBuilder, Iid, Mut, Renderer, Size, UiDerive, UiInput,
    UiInputBase, UiLayout, UiUpdate, UiView,
};
use std::any::Any;

pub trait Component: Default + Sized {
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new() -> Self {
        Self::default()
    }

    fn set<Ancestor: Component>(
        self,
        id: Iid,
        ui: &mut UiView<Ancestor>,
    ) -> ContentBuilder<Self, Ancestor> {
        ui.add(self, id)
    }

    fn init(props: &Self) -> Self::State;

    #[allow(unused_variables)]
    fn derive_state(props: &Self, state: &mut Self::State, ui: &UiDerive) {}

    #[allow(unused_variables)]
    fn update(msg: Self::Msg, state: Mut<Self::State>, ui: &mut UiUpdate) {}

    #[allow(unused_variables)]
    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {}

    #[allow(unused_variables)]
    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.is_empty() {
            Size::zero()
        } else {
            if children.len() > 1 {
                let name = ui.full_debug_name();
                log::error!("The default layout function is being applied to {} which hosts multiple children while this layout function only considers the first one", name);
            }
            let child = children[0];
            ui.size(child, constraints)
        }
    }

    #[allow(unused_variables)]
    fn input(state: &Self::State, input: &mut UiInput<Self>) {}

    #[allow(unused_variables)]
    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {}
}

pub(crate) trait ComponentPointerTrait: Component {
    fn pointer() -> ComponentPointer;
    fn dyn_update(messages: &mut Box<Any>, state: &mut Box<Any>, ui: &mut UiUpdate);
    fn dyn_layout(
        state: &Box<Any>,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size;
    fn dyn_render(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer);
    fn dyn_input(input: &mut UiInputBase);
}

impl<Comp> ComponentPointerTrait for Comp
where
    Comp: Component,
{
    fn pointer() -> ComponentPointer {
        ComponentPointer {
            layout: Self::dyn_layout,
            render: Self::dyn_render,
            input: Self::dyn_input,
            update: Self::dyn_update,
        }
    }

    fn dyn_update(messages: &mut Box<Any>, state: &mut Box<Any>, ui: &mut UiUpdate) {
        let messages: &mut Vec<Self::Msg> = messages.downcast_mut().unwrap();
        let state: &mut Self::State = state.downcast_mut().unwrap();
        let mut mutated = false;
        for msg in messages.drain(..) {
            let state = Mut::new(state, &mut mutated);
            Self::update(msg, state, ui);
        }
        if mutated {
            ui.needs_update();
        }
    }

    fn dyn_layout(
        state: &Box<Any>,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        let state: &Self::State = state.downcast_ref().unwrap();
        Self::layout(state, children, constraints, ui)
    }

    fn dyn_render(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer) {
        let state: &Self::State = state.downcast_ref().unwrap();
        Self::render(state, bounds, renderer);
    }

    fn dyn_input(base: &mut UiInputBase) {
        let state: &Box<Any> = base.state[base.cid.get()].as_ref().unwrap();
        let state: &Self::State = state.downcast_ref().unwrap();
        let mut input = UiInput::new(base);
        Self::input(state, &mut input);
        if !input.messages.is_empty() {
            base.needs_update();
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ComponentPointer {
    pub layout: fn(
        state: &Box<Any>,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size,
    pub render: fn(state: &Box<Any>, bounds: Bounds, renderer: &mut Renderer),
    pub input: fn(input: &mut UiInputBase),
    pub update: fn(messages: &mut Box<Any>, state: &mut Box<Any>, ui: &mut UiUpdate),
}

impl Default for ComponentPointer {
    #[rustfmt::skip]
    fn default() -> Self {
        ComponentPointer {
            layout: |_, _, _, _| panic!("Called `layout` on default `ComponentPointer`"),
            render: |_, _, _|    panic!("Called `render` on default `ComponentPointer`"),
            input : |_|          panic!("Called `input` on default `ComponentPointer`" ),
            update: |_, _, _|    panic!("Called `update` on default `ComponentPointer`"),
        }
    }
}
