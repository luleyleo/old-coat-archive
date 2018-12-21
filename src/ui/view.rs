use crate::component::ComponentPointerTrait;
use crate::{
    AppEvent, AppProps, Cid, Component, Named, PropsBuilder, ReactivePropsBuilder, UiData,
};
use log::trace;
use std::any::TypeId;
use std::marker::PhantomData;

pub struct UiView<'a, C: Component> {
    data: &'a mut UiData,
    cid: Cid,
    marker: PhantomData<C>,
}

impl<'a, Comp: Component> UiView<'a, Comp> {
    pub(crate) fn new(data: &'a mut UiData, cid: Cid) -> Self {
        UiView {
            data,
            cid,
            marker: PhantomData,
        }
    }

    pub(crate) fn run(data: &'a mut UiData, app_id: Cid, props: AppProps)
    where
        Comp: Component<Props = AppProps, Event = AppEvent>,
    {
        trace!("Running `UiView`");
        if data.typeid[app_id.get()] == TypeId::of::<()>() {
            trace!("Initializing Root Component with {:?}", app_id);
            data.typeid[app_id.get()] = TypeId::of::<Comp>();
            data.name[app_id.get()] = "Root";
            data.pointer[app_id.get()] = Comp::pointer();
            data.state[app_id.get()] = Some(Box::new(Comp::init_state(&props)));
            data.messages[app_id.get()] = Some(Box::new(Vec::<Comp::Msg>::new()));
            data.events[app_id.get()] = Box::new(Vec::<Comp::Event>::new());
        }

        trace!(
            "Detaching the `state` of Root with {:?} to setup the `view`",
            app_id
        );
        let state = data.state[app_id.get()].take().unwrap();

        {
            let mut ui = UiView::new(data, app_id);
            Comp::view(&props, state.downcast_ref().unwrap(), &mut ui);
        }

        trace!(
            "Reataching the `state` of Root with {:?} after setting up the `view`",
            app_id
        );
        data.state[app_id.get()] = Some(state);
    }

    pub fn set_reactive<ID, C, T>(&mut self, id: ID, builder: ReactivePropsBuilder<C, T>)
    where
        ID: Named + 'static,
        C: Component,
        T: Component,
    {
        self.set(id, builder.base);

        let tid = TypeId::of::<ID>();
        let handler = builder.handler;

        let emitter = self.data.creations[self.cid.get()][&tid];
        let receiver = self.cid;

        let events: &mut Vec<C::Event> = self.data.events[emitter.get()].downcast_mut().unwrap();
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

    pub fn set<ID, C>(&mut self, _id: ID, builder: PropsBuilder<C>)
    where
        ID: Named + 'static,
        C: Component,
    {
        let tid = TypeId::of::<ID>();
        let name = ID::name();
        let cid = self.data.creations[self.cid.get()]
            .get(&tid)
            .cloned()
            .unwrap_or_else(|| {
                let cid = self.data.fresh_id();
                trace!("Initializing {} Component with {:?}", name, cid);
                self.data.creations[self.cid.get()].insert(tid, cid);

                self.data.typeid[cid.get()] = TypeId::of::<C>();
                self.data.name[cid.get()] = name;
                self.data.pointer[cid.get()] = C::pointer();
                self.data.parent[cid.get()] = Some(self.cid);
                self.data.children[self.cid.get()].push(cid);
                self.data.state[cid.get()] = Some(Box::new(C::init_state(&*builder)));
                self.data.messages[cid.get()] = Some(Box::new(Vec::<C::Msg>::new()));
                self.data.events[cid.get()] = Box::new(Vec::<C::Event>::new());

                cid
            });
        trace!("Detaching the `state` of {:?} to setup the `view`", cid);
        let state = self.data.state[cid.get()].take().unwrap();

        {
            let mut ui = UiView::new(self.data, cid);
            C::view(&*builder, state.downcast_ref().unwrap(), &mut ui);
        }

        trace!(
            "Reataching the `state` of {:?} after setting up the `view`",
            cid
        );
        self.data.state[cid.get()] = Some(state);
    }
}
