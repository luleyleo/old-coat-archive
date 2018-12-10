use crate::{Cid, Component, AppProps, AppEvent, PropsBuilder, ReactivePropsBuilder, UiData, ViewArgs};
use crate::component::ComponentPointerTrait;
use std::any::TypeId;

pub struct UiView<'a> {
    data: &'a mut UiData,
    current: Cid,
}

impl<'a> UiView<'a> {
    pub(crate) fn new(data: &'a mut UiData, start: Cid) -> Self {
        UiView {
            data,
            current: start,
        }
    }

    pub(crate) fn start<Root>(&mut self, props: AppProps) where Root: Component<Props=AppProps, Event=AppEvent> {
        let root = self.current;
        if self.data.typeid[root.get()] == TypeId::of::<()>() {
            self.data.typeid[root.get()] = TypeId::of::<Root>();
            self.data.pointer[root.get()] = Root::pointer();
            self.data.parent[root.get()] = Some(self.current);
            self.data.state[root.get()] = Some(Box::new(Root::init_state(&props)));
        }

        let state = self.data.state[root.get()].take().unwrap();

        Root::view(ViewArgs {
            props: &props,
            state: state.downcast_ref().unwrap(),
            ui: self,
        });

        self.data.state[root.get()] = Some(state);
    }

    pub fn set_reactive<C, T>(&mut self, id: TypeId, builder: ReactivePropsBuilder<C, T>)
    where
        C: Component,
        T: Component,
    {
        self.set(id, builder.base);
    }

    pub fn set<C>(&mut self, id: TypeId, builder: PropsBuilder<C>) where C: Component {
        let cid = self.data.creations[self.current.get()]
            .get(&id)
            .cloned()
            .unwrap_or_else(|| {
                let cid = self.data.fresh_id();
                self.data.creations[self.current.get()].insert(id, cid);

                self.data.typeid[cid.get()] = TypeId::of::<C>();
                self.data.pointer[cid.get()] = C::pointer();
                self.data.parent[cid.get()] = Some(self.current);
                self.data.state[cid.get()] = Some(Box::new(C::init_state(&*builder)));

                cid
            });
        let state = self.data.state[cid.get()].take().unwrap();

        C::view(ViewArgs {
            props: &*builder,
            state: state.downcast_ref().unwrap(),
            ui: self,
        });

        self.data.state[cid.get()] = Some(state);
    }
}
