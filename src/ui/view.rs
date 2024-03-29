use crate::component::ComponentPointerTrait;
use crate::{
    find_focus_state, Cid, Component, ContentBuilder, FocusState, Iid, Renderer, TypeIds, UiData,
    UiDerive,
};
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
    pub fn focus_state(&self) -> FocusState {
        if let Some(focused) = self.data.focused {
            find_focus_state(self.cid, focused, &self.data.children)
        } else {
            FocusState::None
        }
    }

    pub(crate) fn new(data: &'a mut UiData, renderer: &'a mut Renderer, cid: Cid) -> Self {
        let parent = Rc::new(Cell::new(None));
        UiView {
            data,
            parent,
            cid,
            marker: PhantomData,
            renderer,
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
            log::trace!("Initializing root component with {:?}", app_id);
            data.typeids[app_id.get()] = TypeIds::of::<Comp>();
            data.name[app_id.get()] = "Root";
            data.pointer[app_id.get()] = Comp::pointer();
            data.state[app_id.get()] = Some(Box::new(Comp::init(&props)));
            data.messages[app_id.get()] = Some(Box::new(Vec::<Comp::Msg>::new()));
            data.events[app_id.get()] = Box::new(Vec::<Comp::Event>::new());

            log::trace!("Root component set: {}", data.full_debug_name_of(app_id));
        }

        let state = data.state[app_id.get()].take().unwrap();
        {
            let mut ui = UiView::new(data, renderer, app_id);
            Comp::view(&props, state.downcast_ref().unwrap(), &mut ui);
        }
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
                log::trace!("Initializing component \"{}\" with {:?}", name, cid);
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

                log::trace!(
                    "Initial component set: {}",
                    self.data.full_debug_name_of(cid)
                );

                cid
            });

        let mut state = self.data.state[cid.get()].take().unwrap();
        {
            let current_parent = self.parent.get();
            self.parent.set(Some(cid));

            {
                let state = state.downcast_mut().unwrap();
                let font_manager = &mut self.renderer.font_manager;
                let ui = UiDerive::new(font_manager);
                NewComp::derive_state(&props, state, &ui);
            }

            let mut ui = self.another(cid);
            NewComp::view(&props, state.downcast_ref().unwrap(), &mut ui);

            self.parent.set(current_parent);
        }
        self.data.state[cid.get()] = Some(state);

        ContentBuilder::new(cid, self.parent.clone())
    }

    /// NOTE: This function can fail as `C` is not guaranteed to be the correct type for `emitter`
    /// This will only be called by a `ContentBuilder` to guarantee type safety.
    pub(crate) fn map_events<Emitter, Handler>(&mut self, emitter: Cid, handler: Handler)
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
