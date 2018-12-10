use crate::{Component, UiView};
use std::any::TypeId;

pub struct PropsBuilder<C: Component> {
    pub(crate) props: C::Props,
}

pub struct ReactivePropsBuilder<C: Component, T: Component> {
    pub(crate) base: PropsBuilder<C>,
    pub(crate) handler: fn(C::Event) -> T::Msg,
}

impl<C> PropsBuilder<C> where C: Component {
    pub fn new(props: C::Props) -> Self {
        PropsBuilder {
            props,
        }
    }

    pub fn handle<T>(self, handler: fn(C::Event) -> T::Msg) -> ReactivePropsBuilder<C, T> where T: Component {
        ReactivePropsBuilder {
            base: self,
            handler,
        }
    }

    pub fn set<ID>(self, _id: ID, ui: &mut UiView) where ID: 'static {
        ui.set(TypeId::of::<ID>(), self);
    }
}

impl<C, T> ReactivePropsBuilder<C, T> where C: Component, T: Component {
    pub fn set<ID>(self, _id: ID, ui: &mut UiView) where ID: 'static {
        ui.set_reactive(TypeId::of::<ID>(), self);
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
