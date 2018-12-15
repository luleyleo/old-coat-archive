use crate::{Cid, Component, UiData};
use std::any::TypeId;

pub struct UiUpdate<'a> {
    data: &'a mut UiData,
    cid: Cid,
}

impl<'a> UiUpdate<'a> {
    pub(crate) fn new(data: &'a mut UiData, cid: Cid) -> Self {
        UiUpdate { data, cid }
    }

    /// Sends the `msg` to the closest parent of the related `Component`
    pub fn bubble<C>(&mut self, msg: C::Msg)
    where
        C: Component + 'static,
    {
        while let Some(parent) = self.data.parent[self.cid.get()] {
            if self.data.typeid[parent.get()] == TypeId::of::<C>() {
                if let Some(ref mut messages) = self.data.messages[parent.get()] {
                    let messages: &mut Vec<C::Msg> = messages.downcast_mut().unwrap();
                    messages.push(msg);
                } else {
                    // TODO: This should log an error
                }
                return;
            }
        }
    }
}
