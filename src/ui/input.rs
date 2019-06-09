use crate::{
    component::ComponentPointer, find_focus_state, Bounds, Cid, Component, FocusState, Input,
    Position, Size, UiData,
};
use std::any::Any;

pub struct Messages<'a, C: Component>(&'a mut Vec<C::Msg>);

impl<'a, C: Component> Messages<'a, C> {
    pub fn send(&mut self, msg: C::Msg) {
        self.0.push(msg);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct UiInput<'a, C: Component> {
    cid: Cid,
    children: &'a Vec<Vec<Cid>>,
    focused: &'a Option<Cid>,
    pub messages: Messages<'a, C>,
    pub input: &'a mut Input,
    pub bounds: Bounds,
}

impl<'a, C> UiInput<'a, C>
where
    C: Component,
{
    pub(crate) fn run(data: &'a mut UiData, input: &'a mut Input, root: Cid) -> bool {
        if data.is_fresh(root) {
            log::trace!("Skipping `UiInput`");
            return true;
        }
        log::trace!("Running `UiInput`");

        let mut ui = UiInputBase::new(data, input);
        ui.visit(root);
        return ui.needs_update;
    }

    pub(crate) fn new(base: &'a mut UiInputBase) -> Self {
        let messages: &mut Vec<C::Msg> = base.messages[base.cid.get()]
            .as_mut()
            .unwrap()
            .downcast_mut()
            .unwrap();
        let messages = Messages(messages);

        // Find absolute position
        // TODO: Maybe we should store the absolute position instead of the relative one
        let mut position = base.position[base.cid.get()];
        let mut parent = base.parent[base.cid.get()];
        while let Some(p) = parent {
            position += base.position[p.get()].to_vector();
            parent = base.parent[p.get()];
        }

        let bounds = Bounds::new(position, base.size[base.cid.get()]);

        UiInput {
            cid: base.cid,
            children: &base.children,
            focused: &base.focused,

            messages,
            input: base.input,
            bounds,
        }
    }

    pub fn focus_state(&self) -> FocusState {
        if let Some(focused) = self.focused {
            find_focus_state(self.cid, *focused, &self.children)
        } else {
            FocusState::None
        }
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
}

pub(crate) struct UiInputBase<'a> {
    pointer: &'a Vec<ComponentPointer>,
    parent: &'a Vec<Option<Cid>>,
    children: &'a Vec<Vec<Cid>>,
    position: &'a Vec<Position>,
    size: &'a Vec<Size>,
    pub state: &'a Vec<Option<Box<Any>>>,
    messages: &'a mut Vec<Option<Box<Any>>>,
    focused: &'a Option<Cid>,

    input: &'a mut Input,
    pub cid: Cid,
    needs_update: bool,
}

impl<'a> UiInputBase<'a> {
    fn new(data: &'a mut UiData, input: &'a mut Input) -> Self {
        UiInputBase {
            pointer: &data.pointer,
            parent: &data.parent,
            children: &data.children,
            position: &data.position,
            size: &data.size,
            state: &data.state,
            messages: &mut data.messages,
            focused: &data.focused,

            input,
            cid: Cid::invalid(),
            needs_update: false,
        }
    }

    pub fn needs_update(&mut self) {
        self.needs_update = true;
    }

    fn visit(&mut self, cid: Cid) {
        let children = &self.children[cid.get()];

        for child in children {
            self.visit(*child);
        }

        self.cid = cid;
        let pointer = self.pointer[cid.get()];
        (pointer.input)(self);
    }
}
