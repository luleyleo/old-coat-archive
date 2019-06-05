use crate::{Cid, Component, UiView};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct ContentBuilder<Parent: Component, Ancestor: Component> {
    /// The component which will receive events
    pub(crate) ancestor: Cid,
    /// The component which new children will be attached to
    pub(crate) parent: Rc<Cell<Option<Cid>>>,
    /// Types
    comp: PhantomData<Parent>,
    target: PhantomData<Ancestor>,
}

impl<Parent: Component, Ancestor: Component> ContentBuilder<Parent, Ancestor> {
    pub(crate) fn new(ancestor: Cid, parent: Rc<Cell<Option<Cid>>>) -> Self {
        ContentBuilder {
            ancestor,
            parent,
            comp: PhantomData,
            target: PhantomData,
        }
    }

    pub fn map_events(
        self,
        ui: &mut UiView<Ancestor>,
        handler: impl Fn(Parent::Event) -> Option<Ancestor::Msg>,
    ) -> Self {
        if let Some(_parent) = self.parent.get() {
            ui.map_events::<Parent, _>(self.ancestor, handler);
        }
        self
    }

    pub fn add(self, mut builder: impl FnMut()) {
        let current_parent = self.parent.get();
        (&*self.parent).set(Some(self.ancestor));
        builder();
        (&*self.parent).set(current_parent);
    }
}
