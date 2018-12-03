use crate::{Cid, Component, PropsBuilder, UiData, ViewArgs};
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

    pub fn set<C, T>(&mut self, id: usize, builder: PropsBuilder<C, T>)
    where
        C: Component,
        T: Component,
    {
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
