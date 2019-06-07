use crate::{component::ComponentPointer, Bounds, Cid, Component, Input, Position, Size, UiData};
use std::any::Any;

pub struct Messages<'a, C: Component>(&'a mut Vec<C::Msg>);

impl<'a, C: Component> Messages<'a, C> {
    pub fn send(&mut self, msg: C::Msg) {
        self.0.push(msg);
    }
}

pub struct UiInput<'a, C: Component> {
    pub messages: Messages<'a, C>,
    pub input: &'a mut Input,
    pub bounds: Bounds,
}

impl<'a, C> UiInput<'a, C>
where
    C: Component,
{
    pub(crate) fn run(data: &'a mut UiData, input: &'a mut Input, root: Cid) {
        if data.is_fresh(root) {
            log::trace!("Skipping `UiInput`");
            return;
        }
        log::trace!("Running `UiInput`");

        let mut ui = UiInputBase::new(data, input);
        ui.visit(root);
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
            messages,
            input: base.input,
            bounds,
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
    input: &'a mut Input,
    pub cid: Cid,
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
            input,
            cid: Cid::invalid(),
        }
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
