use crate::component::ComponentPointerTrait;
use crate::{Cid, Component, ContentBuilder, Iid, TypeIds, UiData, UiDerive, Renderer};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct UiView<'a, Comp: Component> {
    data: &'a mut UiData,
    /// The `Cid` of the component to which new ones will be added.
    parent: Rc<Cell<Option<Cid>>>,
    /// The `Cid` of the component which's `view` function is currently being run.
    /// This is often the `ancestor`.
    /// The type of the associated component is `Comp`.
    cid: Cid,
    /// Type of the component behind `cid`.
    marker: PhantomData<Comp>,
    renderer: &'a mut Renderer,
}

impl<'a, Comp: Component> UiView<'a, Comp> {
    pub(crate) fn new(data: &'a mut UiData, renderer: &'a mut Renderer, cid: Cid) -> Self {
        let parent = Rc::new(Cell::new(None));
        UiView {
            data,
            parent,
            cid,
            marker: PhantomData,
            renderer
        }
    }

    /// This will construct a `UiView` with the same date but for a different component.
    fn another<AComp: Component>(&mut self, cid: Cid) -> UiView<AComp> {
        UiView {
            data: self.data,
            parent: self.parent.clone(),
            cid: cid,
            marker: PhantomData,
            renderer: self.renderer,
        }
    }

    pub(crate) fn run(data: &'a mut UiData, renderer: &'a mut Renderer, app_id: Cid, props: Comp) {
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
            let mut ui = UiView::new(data, renderer, app_id);
            Comp::view(&props, state.downcast_ref().unwrap(), &mut ui);
        }

        log::trace!(
            "Reataching the `state` of Root with {:?} after setting up the `view`",
            app_id
        );
        data.state[app_id.get()] = Some(state);
    }

    /// Adds a new component to the tree.
    /// Instead of this `Properties::set` can be used for a nicer builder pattern.
    pub fn add<NewComp: Component>(
        &mut self,
        props: NewComp,
        iid: Iid,
    ) -> ContentBuilder<NewComp, Comp> {
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

                self.data.typeids[cid.get()] = TypeIds::of::<NewComp>();
                self.data.name[cid.get()] = name;
                self.data.pointer[cid.get()] = NewComp::pointer();
                self.data.parent[cid.get()] = Some(parent);
                self.data.children[parent.get()].push(cid);
                self.data.state[cid.get()] = Some(Box::new(NewComp::init(&props)));
                self.data.messages[cid.get()] = Some(Box::new(Vec::<NewComp::Msg>::new()));
                self.data.events[cid.get()] = Box::new(Vec::<NewComp::Event>::new());

                log::trace!("View set: {}", self.data.full_debug_name_of(cid));

                cid
            });
        log::trace!("Detaching the `state` of {:?} to setup the `view`", cid);
        let mut state = self.data.state[cid.get()].take().unwrap();

        {
            let current_parent = self.parent.get();
            self.parent.set(Some(cid));

            {
                let events = self.data.events[cid.get()].downcast_mut().unwrap();
                let state = state.downcast_mut().unwrap();
                let renderer = &mut self.renderer.font_manager;
                let mut ui = UiDerive::new(events, renderer);
                NewComp::derive_state(&props, state, &mut ui);
            }

            let mut ui = self.another(cid);
            NewComp::view(&props, state.downcast_ref().unwrap(), &mut ui);

            self.parent.set(current_parent);
        }

        log::trace!(
            "Reataching the `state` of {:?} after setting up the `view`",
            cid
        );
        self.data.state[cid.get()] = Some(state);

        ContentBuilder::new(cid, self.parent.clone())
    }

    /// Note: This function can fail as `C` is not guaranteed to be the correct type for `emitter`
    /// This will only be called by a `ContentBuilder` to guarantee type safety.
    pub(crate) fn on_event<Emitter, Handler>(&mut self, emitter: Cid, handler: Handler)
    where
        Emitter: Component,
        Handler: Fn(Emitter::Event) -> Option<Comp::Msg>,
    {
        let events: &mut Vec<Emitter::Event> =
            self.data.events[emitter.get()].downcast_mut().unwrap();
        let messages: &mut Vec<Comp::Msg> = self.data.messages[self.cid.get()]
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
}
