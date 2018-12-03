use crate::{Component, UiView};

pub struct PropsBuilder<C: Component, T: Component> {
    props: C::Props,
    handler: Option<fn(C::Event) -> T::Msg>,
    parent: usize,
}

impl<C, T> PropsBuilder<C, T>
where
    C: Component,
    T: Component,
{
    pub fn new(props: C::Props) -> Self {
        PropsBuilder {
            props,
            handler: None,
            parent: 0,
        }
    }

    pub fn handle(mut self, handler: fn(C::Event) -> T::Msg) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn parent(mut self, parent: usize) -> Self {
        self.parent = parent;
        self
    }

    pub fn set(self, id: usize, ui: &mut UiView) {
        ui.set(id, self);
    }
}

impl<C: Component, T: Component> std::ops::Deref for PropsBuilder<C, T> {
    type Target = C::Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl<C: Component, T: Component> std::ops::DerefMut for PropsBuilder<C, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.props
    }
}
