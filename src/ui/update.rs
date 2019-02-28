use crate::{Cid, Component, ComponentPointer, Font, Renderer, TypeIds, UiData};
use log::{trace, warn};
use std::any::{Any, TypeId};

pub struct UiUpdate<'a> {
    typeids: &'a Vec<TypeIds>,
    pointer: &'a Vec<ComponentPointer>,
    parent: &'a Vec<Option<Cid>>,
    children: &'a Vec<Vec<Cid>>,
    messages: &'a mut Vec<Option<Box<Any>>>,
    events: &'a mut Vec<Box<Any>>,
    state: &'a mut Vec<Option<Box<Any>>>,
    renderer: &'a mut Renderer,
    cid: Cid,
    needs_update: bool,
}

impl<'a> UiUpdate<'a> {
    pub fn emit<E: 'static>(&mut self, event: E) {
        if self.typeids[self.cid.get()].message == TypeId::of::<E>() {
            let events = &mut self.events[self.cid.get()];
            let events = events.downcast_mut::<Vec<E>>().unwrap();
            events.push(event);
        }
    }

    /// Sends the `msg` to the closest parent of the related `Component`
    pub fn bubble<Comp: Component>(&mut self, msg: Comp::Msg) {
        while let Some(parent) = self.parent[self.cid.get()] {
            if self.typeids[parent.get()] == TypeIds::of::<Comp>() {
                let messages: &mut Vec<Comp::Msg> = self.messages[parent.get()]
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

    pub fn add_font(&mut self, font: &Font, data: impl Into<Vec<u8>>) {
        self.renderer.add_font(font.clone(), data.into());
    }

    pub fn remove_font(&mut self, font: &Font) {
        self.renderer.remove_font(font);
    }
}

impl<'a> UiUpdate<'a> {
    pub(crate) fn needs_update(&mut self) {
        self.needs_update = true;
    }

    pub(crate) fn run(data: &'a mut UiData, renderer: &'a mut Renderer, root: Cid) -> bool {
        if data.is_fresh(root) {
            trace!("Skipping `UiUpdate`");
            return true;
        }
        trace!("Running `UiUpdate`");

        let mut ui = UiUpdate {
            typeids: &data.typeids,
            pointer: &data.pointer,
            parent: &data.parent,
            children: &data.children,
            messages: &mut data.messages,
            events: &mut data.events,
            state: &mut data.state,
            renderer: renderer,
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
}
