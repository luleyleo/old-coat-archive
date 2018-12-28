use crate::{
    Bounds, BoxConstraints, Cid, Mut, PropsBuilder, Renderer, Size, UiInput, UiInputBase, UiLayout,
    UiUpdate, UiView,
};
use std::any::Any;

pub trait Component: Sized {
    type Props: Sized;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new() -> PropsBuilder<Self>;

    fn init_state(props: &Self::Props) -> Self::State;

    #[allow(unused_variables)]
    fn update(msg: Self::Msg, state: Mut<Self::State>, ui: &mut UiUpdate) -> Option<Self::Event> {
        None
    }

    fn view(props: &Self::Props, state: &Self::State, ui: &mut UiView<Self>);

    #[allow(unused_variables)]
    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.is_empty() {
            Size::default()
        } else {
            if children.len() > 1 {
                let name = ui.full_debug_name();
                log::error!("The default layout function is beeing applied to {} which hosts multiple children while this layout function only considers the first one", name);
            }
            let child = children[0];
            ui.size(child, constraints)
        }
    }

    #[allow(unused_variables)]
    fn input(input: &mut UiInput<Self>) {}

    #[allow(unused_variables)]
    fn derive_state(props: &Self::Props, state: Mut<Self::State>) {}

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

impl<C> ComponentPointerTrait for C
where
    C: Component,
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
            if let Some(event) = Self::update(msg, state, ui) {
                ui.emit(event);
            }
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

    fn dyn_input(input: &mut UiInputBase) {
        let mut input = UiInput::new(input);
        Self::input(&mut input);
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
    fn default() -> Self {
        ComponentPointer {
            layout: |_, _, _, _| panic!("Called `layout` on default `ComponentPointer`"),
            render: |_, _, _|    panic!("Called `render` on default `ComponentPointer`"),
            input : |_|          panic!("Called `input` on default `ComponentPointer`" ),
            update: |_, _, _|    panic!("Called `update` on default `ComponentPointer`"),
        }
    }
}
