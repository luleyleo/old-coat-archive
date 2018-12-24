use crate::{Cid, Component, ComponentPointer, UiData};
use log::{trace, warn};
use std::any::{Any, TypeId};

pub struct UiUpdate<'a> {
    typeid: &'a Vec<TypeId>,
    pointer: &'a Vec<ComponentPointer>,
    parent: &'a Vec<Option<Cid>>,
    children: &'a Vec<Vec<Cid>>,
    messages: &'a mut Vec<Option<Box<Any>>>,
    events: &'a mut Vec<Box<Any>>,
    state: &'a mut Vec<Option<Box<Any>>>,
    cid: Cid,
    needs_update: bool,
}

impl<'a> UiUpdate<'a> {
    pub(crate) fn run(data: &'a mut UiData, root: Cid) -> bool {
        if data.is_fresh(root) {
            trace!("Skipping `UiUpdate`");
            return true;
        }
        trace!("Running `UiUpdate`");

        let mut ui = UiUpdate {
            typeid: &data.typeid,
            pointer: &data.pointer,
            parent: &data.parent,
            children: &data.children,
            messages: &mut data.messages,
            events: &mut data.events,
            state: &mut data.state,
            cid: root,
            needs_update: false,
        };

        ui.visit(root);
        ui.needs_update
    }

    fn visit(&mut self, cid: Cid) {
        {
            trace!("Detaching the `state` of {:?} in order to `update`", cid);
            let mut messages = self.messages[cid.get()].take().unwrap();
            let mut state = self.state[cid.get()].take().unwrap();

            let pointer = self.pointer[cid.get()];
            (pointer.update)(&mut messages, &mut state, self);

            trace!("Reataching the `state` of {:?} after `update`ing", cid);
            self.messages[cid.get()] = Some(messages);
            self.state[cid.get()] = Some(state);
        }

        let children = &self.children[cid.get()];
        for child in children {
            self.cid = *child;
            self.visit(*child);
        }
    }

    pub(crate) fn emit<E: 'static>(&mut self, event: E) {
        let events = &mut self.events[self.cid.get()];
        if let Some(events) = events.downcast_mut::<Vec<E>>() {
            events.push(event);
        }
    }

    pub(crate) fn needs_update(&mut self) {
        self.needs_update = true;
    }

    /// Sends the `msg` to the closest parent of the related `Component`
    pub fn bubble<C>(&mut self, msg: C::Msg)
    where
        C: Component + 'static,
    {
        while let Some(parent) = self.parent[self.cid.get()] {
            if self.typeid[parent.get()] == TypeId::of::<C>() {
                let messages: &mut Vec<C::Msg> = self.messages[parent.get()]
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap();
                messages.push(msg);
                return;
            }
        }
        warn!("Tried to bubble a message but the targeted Component does not exist");
    }
}
