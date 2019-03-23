use crate::{Cid, Component, Iid, UiView};
use std::cell::Cell;
use std::rc::Rc;
use std::marker::PhantomData;

pub struct PropsBuilder<C: Component> {
    pub(crate) props: C::Props,
}

pub struct ReactivePropsBuilder<C: Component, T: Component> {
    pub(crate) base: PropsBuilder<C>,
    pub(crate) handler: fn(C::Event) -> Option<T::Msg>,
}

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

    pub fn on(self, ui: &mut UiView<Ancestor>, handler: impl Fn(Comp::Event) -> Option<Ancestor::Msg>) -> Self {
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

impl<C> PropsBuilder<C>
where
    C: Component,
{
    pub fn new(props: C::Props) -> Self {
        PropsBuilder { props }
    }

    pub fn handle<T>(self, handler: fn(C::Event) -> Option<T::Msg>) -> ReactivePropsBuilder<C, T>
    where
        T: Component,
    {
        ReactivePropsBuilder {
            base: self,
            handler,
        }
    }

    pub fn set<T>(self, id: Iid, ui: &mut UiView<T>) -> ContentBuilder<C, T>
    where
        T: Component,
    {
        ui.set(id, self)
    }
}

impl<C, T> ReactivePropsBuilder<C, T>
where
    C: Component,
    T: Component,
{
    pub fn set(self, id: Iid, ui: &mut UiView<T>) -> ContentBuilder<C, T> {
        ui.set_reactive(id, self)
    }
}

impl<C: Component> std::ops::Deref for PropsBuilder<C> {
    type Target = C::Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl<C: Component> std::ops::DerefMut for PropsBuilder<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.props
    }
}

