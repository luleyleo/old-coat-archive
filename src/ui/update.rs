use crate::{Cid, Component, ComponentPointer, Font, Renderer, TypeIds, UiData};
use std::any::{Any, TypeId};

pub struct UiUpdate<'a> {
    typeids: &'a Vec<TypeIds>,
    pointer: &'a Vec<ComponentPointer>,
    parent: &'a Vec<Option<Cid>>,
    children: &'a Vec<Vec<Cid>>,
    messages: &'a mut Vec<Option<Box<Any>>>,
    events: &'a mut Vec<Box<Any>>,
    state: &'a mut Vec<Option<Box<Any>>>,
    focused: &'a mut Option<Cid>,
    renderer: &'a mut Renderer,
    cid: Cid,
    needs_update: bool,
}

impl<'a> UiUpdate<'a> {
    pub fn emit<E: 'static>(&mut self, event: E) {
        if self.typeids[self.cid.get()].event == TypeId::of::<E>() {
            let events = &mut self.events[self.cid.get()];
            let events = events.downcast_mut::<Vec<E>>().unwrap();
            events.push(event);
            self.needs_update();
        } else {
            log::error!("Tried to emit a event of the wrong type");
            // TODO: This should not be possible
        }
    }

    pub fn aquire_focus(&mut self) {
        *self.focused = Some(self.cid);
    }

    pub fn lose_focus(&mut self) {
        *self.focused = None;
    }

    /// TODO: Can this be removed?
    /// Sends the `msg` to the closest parent of the related `Component`
    pub fn bubble<Target: Component>(&mut self, msg: Target::Msg) {
        while let Some(parent) = self.parent[self.cid.get()] {
            if self.typeids[parent.get()] == TypeIds::of::<Target>() {
                let messages: &mut Vec<Target::Msg> = self.messages[parent.get()]
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap();
                messages.push(msg);
                self.needs_update();
                return;
            }
        }
        log::warn!("Tried to bubble a message but the targeted Component does not exist");
    }

    pub fn add_font(&mut self, font: &Font, data: impl Into<Vec<u8>>) {
        self.renderer.add_font(font.clone(), data.into());
        self.needs_update();
    }

    pub fn remove_font(&mut self, font: &Font) {
        self.renderer.remove_font(font);
        self.needs_update();
    }
}

impl<'a> UiUpdate<'a> {
    pub(crate) fn needs_update(&mut self) {
        self.needs_update = true;
    }

    pub(crate) fn run(data: &'a mut UiData, renderer: &'a mut Renderer, root: Cid) -> bool {
        if data.is_fresh(root) {
            log::trace!("Skipping `UiUpdate`");
            return true;
        }
        log::trace!("Running `UiUpdate`");

        let mut ui = UiUpdate {
            typeids: &data.typeids,
            pointer: &data.pointer,
            parent: &data.parent,
            children: &data.children,
            messages: &mut data.messages,
            events: &mut data.events,
            state: &mut data.state,
            focused: &mut data.focused,
            renderer: renderer,
            cid: root,
            needs_update: false,
        };

        ui.visit(root);
        ui.needs_update
    }

    fn visit(&mut self, cid: Cid) {
        {
            let mut messages = self.messages[cid.get()].take().unwrap();
            let mut state = self.state[cid.get()].take().unwrap();

            let pointer = self.pointer[cid.get()];
            (pointer.update)(&mut messages, &mut state, self);

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
