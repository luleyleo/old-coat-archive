use crate::{component::ComponentPointer, Cid, Component, Input, UiData, Event};
use std::any::Any;
use log::trace;

pub struct UiInput<'a, C: Component> {
    messages: &'a mut Vec<C::Msg>,
    input: &'a mut Input,
}

impl<'a, C> UiInput<'a, C>
where
    C: Component,
{
    pub(crate) fn run(data: &'a mut UiData, input: &'a mut Input, root: Cid) {
        if data.is_fresh(root) {
            trace!("Skipping `UiInput`");
            return
        }
        trace!("Running `UiInput`");

        let mut ui = UiInputBase::new(data, input);
        ui.visit(root);
    }

    pub(crate) fn new(base: &'a mut UiInputBase) -> Self {
        let messages: &mut Vec<C::Msg> = base.messages[base.cid.get()]
            .as_mut()
            .unwrap()
            .downcast_mut()
            .unwrap();
        
        UiInput {
            messages: messages,
            input: base.input,
        }
    }

    pub fn send(&mut self, msg: C::Msg) {
        self.messages.push(msg);
    }

    pub fn for_all_events(&mut self, mut handler: impl FnMut(bool, &Event) -> bool) {
        for (ref event, ref mut handled) in &mut self.input.events {
            if handler(*handled, event) {
                *handled = true;
            }
        }
    }

    pub fn for_new_events(&mut self, mut handler: impl FnMut(&Event) -> bool) {
        for (ref event, ref mut handled) in &mut self.input.events {
            if !(*handled) {
                if handler(event) {
                    *handled = true;
                }
            }
        }
    }
}

pub(crate) struct UiInputBase<'a> {
    pointer: &'a Vec<ComponentPointer>,
    children: &'a Vec<Vec<Cid>>,
    messages: &'a mut Vec<Option<Box<Any>>>,
    input: &'a mut Input,
    cid: Cid,
}

impl<'a> UiInputBase<'a> {
    fn new(data: &'a mut UiData, input: &'a mut Input) -> Self {
        UiInputBase {
            pointer: &data.pointer,
            children: &data.children,
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
