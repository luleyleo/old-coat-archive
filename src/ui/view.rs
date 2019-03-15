use crate::component::ComponentPointerTrait;
use crate::{
    Cid, Component, ContentBuilder, Iid, PropsBuilder, ReactivePropsBuilder, TypeIds, UiData,
};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct UiView<'a, C: Component> {
    data: &'a mut UiData,
    parent: Rc<Cell<Option<Cid>>>,
    cid: Cid,
    marker: PhantomData<C>,
}

impl<'a, Comp: Component> UiView<'a, Comp> {
    pub(crate) fn new(data: &'a mut UiData, cid: Cid) -> Self {
        let parent = Rc::new(Cell::new(None));
        UiView {
            data,
            parent,
            cid,
            marker: PhantomData,
        }
    }

    fn another<'b, AComp: Component>(&'b mut self, cid: Cid) -> UiView<'b, AComp> {
        UiView {
            data: self.data,
            parent: self.parent.clone(),
            cid: cid,
            marker: PhantomData,
        }
    }

    pub(crate) fn run(data: &'a mut UiData, app_id: Cid, props: Comp::Props) {
        log::trace!("Running `UiView`");
        if data.typeids[app_id.get()] == TypeIds::void() {
            log::trace!("Initializing Root Component with {:?}", app_id);
            data.typeids[app_id.get()] = TypeIds::of::<Comp>();
            data.name[app_id.get()] = "Root";
            data.pointer[app_id.get()] = Comp::pointer();
            data.state[app_id.get()] = Some(Box::new(Comp::init(&props)));
            data.messages[app_id.get()] = Some(Box::new(Vec::<Comp::Msg>::new()));
            data.events[app_id.get()] = Box::new(Vec::<Comp::Event>::new());

            log::trace!("View set: {}", data.full_debug_name_of(app_id));
        }

        log::trace!(
            "Detaching the `state` of Root with {:?} to setup the `view`",
            app_id
        );
        let state = data.state[app_id.get()].take().unwrap();

        {
            let mut ui = UiView::new(data, app_id);
            Comp::view(&props, state.downcast_ref().unwrap(), &mut ui);
        }

        log::trace!(
            "Reataching the `state` of Root with {:?} after setting up the `view`",
            app_id
        );
        data.state[app_id.get()] = Some(state);
    }

    pub fn set_reactive<C, T>(
        &mut self,
        iid: Iid,
        builder: ReactivePropsBuilder<C, T>,
    ) -> ContentBuilder
    where
        C: Component,
        T: Component,
    {
        let cid = self.cid;
        let tid = iid.id;

        let content_builder = self.set(iid, builder.base);
        let handler = builder.handler;

        if let Some(emitter) = self.data.creations[cid.get()].get(&tid) {
            let receiver = cid;

            let events: &mut Vec<C::Event> =
                self.data.events[emitter.get()].downcast_mut().unwrap();
            let messages: &mut Vec<T::Msg> = self.data.messages[receiver.get()]
                .as_mut()
                .unwrap()
                .downcast_mut()
                .unwrap();

            for event in events.drain(..) {
                if let Some(msg) = handler(event) {
                    messages.push(msg);
                }
            }
        }

        content_builder
    }

    pub fn set<C>(&mut self, iid: Iid, builder: PropsBuilder<C>) -> ContentBuilder
    where
        C: Component,
    {
        let tid = iid.id;
        let name = iid.name.unwrap_or("Unnamed");
        let cid = self.data.creations[self.cid.get()]
            .get(&tid)
            .cloned()
            .unwrap_or_else(|| {
                let cid = self.data.fresh_id();
                log::trace!("Initializing {} Component with {:?}", name, cid);
                self.data.creations[self.cid.get()].insert(tid, cid);

                let parent = self.parent.get().unwrap_or(self.cid);

                self.data.typeids[cid.get()] = TypeIds::of::<C>();
                self.data.name[cid.get()] = name;
                self.data.pointer[cid.get()] = C::pointer();
                self.data.parent[cid.get()] = Some(parent);
                self.data.children[parent.get()].push(cid);
                self.data.state[cid.get()] = Some(Box::new(C::init(&*builder)));
                self.data.messages[cid.get()] = Some(Box::new(Vec::<C::Msg>::new()));
                self.data.events[cid.get()] = Box::new(Vec::<C::Event>::new());

                log::trace!("View set: {}", self.data.full_debug_name_of(cid));

                cid
            });
        log::trace!("Detaching the `state` of {:?} to setup the `view`", cid);
        let mut state = self.data.state[cid.get()].take().unwrap();

        {
            let current_parent = self.parent.get();
            self.parent.set(Some(self.cid));

            C::derive_state(&*builder, state.downcast_mut().unwrap());

            let mut ui = self.another(cid);
            C::view(&*builder, state.downcast_ref().unwrap(), &mut ui);

            self.parent.set(current_parent);
        }

        log::trace!(
            "Reataching the `state` of {:?} after setting up the `view`",
            cid
        );
        self.data.state[cid.get()] = Some(state);

        ContentBuilder {
            cid,
            parent: self.parent.clone(),
        }
    }
}
