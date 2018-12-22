use crate::{Cid, Component, Named, UiView};
use std::cell::Cell;
use std::rc::Rc;

pub struct PropsBuilder<C: Component> {
    pub(crate) props: C::Props,
}

pub struct ReactivePropsBuilder<C: Component, T: Component> {
    pub(crate) base: PropsBuilder<C>,
    pub(crate) handler: fn(C::Event) -> Option<T::Msg>,
}

pub struct ContentBuilder {
    pub(crate) cid: Cid,
    pub(crate) parent: Rc<Cell<Option<Cid>>>,
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

    pub fn set<ID, T>(self, id: ID, ui: &mut UiView<T>) -> ContentBuilder
    where
        ID: Named + 'static,
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
    pub fn set<ID>(self, id: ID, ui: &mut UiView<T>) -> ContentBuilder
    where
        ID: Named + 'static,
    {
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

impl ContentBuilder {
    pub fn add(self, mut builder: impl FnMut()) {
        let current_parent = self.parent.get();
        self.parent.set(Some(self.cid));
        builder();
        self.parent.set(current_parent);
    }
}
