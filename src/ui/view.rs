use crate::{Cid, Component, AppProps, AppEvent, PropsBuilder, ReactivePropsBuilder, UiData, ViewArgs, Named};
use crate::component::ComponentPointerTrait;
use std::any::TypeId;
use log::trace;

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

    pub fn set_reactive<ID, C, T>(&mut self, id: ID, builder: ReactivePropsBuilder<C, T>)
    where
        ID: Named + 'static,
        C: Component,
        T: Component,
    {
        self.set(id, builder.base);
    }

    pub fn set<ID, C>(&mut self, _id: ID, builder: PropsBuilder<C>)
    where
        ID: Named + 'static,
        C: Component,
    {
        let tid = TypeId::of::<ID>();
        let name = ID::name();
        let cid = self.data.creations[self.current.get()]
            .get(&tid)
            .cloned()
            .unwrap_or_else(|| {
                let cid = self.data.fresh_id();
                self.data.creations[self.current.get()].insert(tid, cid);

                self.data.typeid[cid.get()] = TypeId::of::<C>();
                self.data.name[cid.get()] = name;
                self.data.pointer[cid.get()] = C::pointer();
                self.data.parent[cid.get()] = Some(self.current);
                self.data.children[self.current.get()].push(cid);
                self.data.state[cid.get()] = Some(Box::new(C::init_state(&*builder)));

                cid
            });
        trace!("Detaching the `state` of {:?} to setup the `view`", cid);
        let state = self.data.state[cid.get()].take().unwrap();

        C::view(ViewArgs {
            props: &*builder,
            state: state.downcast_ref().unwrap(),
            ui: self,
        });

        trace!("Reataching the `state` of {:?} after setting up the `view`", cid);
        self.data.state[cid.get()] = Some(state);
    }
}
