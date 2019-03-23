use crate::{Cid, Component, UiView};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct ContentBuilder<Comp: Component, Target: Component> {
    /// The component which will receive events
    pub(crate) cid: Cid,
    /// The component which new children will be attached to
    pub(crate) parent: Rc<Cell<Option<Cid>>>,
    /// Types
    comp: PhantomData<Comp>,
    target: PhantomData<Target>,
}

impl<Comp: Component, Ancestor: Component> ContentBuilder<Comp, Ancestor> {
    pub(crate) fn new(cid: Cid, parent: Rc<Cell<Option<Cid>>>) -> Self {
        ContentBuilder {
            cid,
            parent,
            comp: PhantomData,
            target: PhantomData,
        }
    }

    pub fn on(
        self,
        ui: &mut UiView<Ancestor>,
        handler: impl Fn(Comp::Event) -> Option<Ancestor::Msg>,
    ) -> Self {
        if let Some(_parent) = self.parent.get() {
            ui.on::<Comp, _>(self.cid, handler);
        }
        self
    }

    pub fn add(self, mut builder: impl FnMut()) {
        let current_parent = self.parent.get();
        (&*self.parent).set(Some(self.cid));
        builder();
        (&*self.parent).set(current_parent);
    }
}
