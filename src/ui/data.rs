use crate::{Cid, Component, ComponentPointer, Position, Size, FontQueue};
use fnv::FnvHashMap;
use smallvec::SmallVec;
use std::any::{Any, TypeId};

/// Contains all data that is necessary for the ui
#[derive(Default)]
pub(crate) struct UiData {
    /// The `TypeId` of the `Component` behind a `Cid`
    pub(crate) typeids: Vec<TypeIds>,
    /// A `stringify!()`ed version of the `creations` id
    pub(crate) name: Vec<&'static str>,
    /// A "pointer" to dynamic versions of a `Component`s functions
    pub(crate) pointer: Vec<ComponentPointer>,
    /// The parent of a `Component` might be None if it is
    /// the ui root or something went wrong.
    pub(crate) parent: Vec<Option<Cid>>,
    /// Lists all children of a `Component` as `Cid`s and
    /// is being used to render and layout the comps like a graph
    pub(crate) children: Vec<Vec<Cid>>,
    /// Similar to `UiData::children` but maps the per-component
    /// child identifier to the associated `Cid` used to index the `UiData`.
    pub(crate) creations: Vec<FnvHashMap<TypeId, Cid>>,
    /// Stores a `Component`s `Position` relative to its parent
    pub(crate) position: Vec<Position>,
    /// Stores the `Component`s final `Size` that fits its `BoxConstraints`
    pub(crate) size: Vec<Size>,
    /// Holds a `Component`s state. It will be moved out of this struct
    /// whenever it gets passed to one of the `Component`s functions.
    /// This and to delay initialization are the reasons why it is a `Option`
    pub(crate) state: Vec<Option<Box<Any>>>,
    /// Holds all messages of the `Component`s.
    /// The `Vec<Box<Any>>` is the alternative to a `Vec<Vec<Box<Any>>>`
    /// to avoid allocating for every message in exchange for a more confusing type.
    /// Thus this resolves to a `Vec<Option<Vec<Component::Msg>>>`
    pub(crate) messages: Vec<Option<Box<Any>>>,
    /// Cache events. Similar to `messages` this is a `Vec<Vec<Component::Event>>`
    pub(crate) events: Vec<Box<Any>>,

    pub(crate) font_queue: FontQueue,
    /// The next `Cid` that will be allocated when needed
    id_count: usize,
}

impl UiData {
    pub(crate) fn fresh_id(&mut self) -> Cid {
        let id = Cid::new(self.id_count);
        log::trace!("Allocated {:?}", id);
        self.id_count += 1;

        self.typeids.push(TypeIds::void());
        self.name.push("");
        self.pointer.push(ComponentPointer::default());
        self.parent.push(None);
        self.children.push(Vec::new());
        self.creations.push(FnvHashMap::default());
        self.position.push(Position::default());
        self.size.push(Size::default());
        self.state.push(None);
        self.messages.push(Some(Box::new(Vec::<()>::new())));
        self.events.push(Box::new(Vec::<()>::new()));

        id
    }

    pub(crate) fn is_fresh(&self, id: Cid) -> bool {
        self.typeids[id.get()] == TypeIds::void()
    }

    pub(crate) fn full_debug_name_of(&self, id: Cid) -> String {
        full_debug_name_of(&self.parent, &self.name, id)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct TypeIds {
    pub state: TypeId,
    pub message: TypeId,
    pub event: TypeId,
}

impl TypeIds {
    pub fn of<Comp: Component>() -> Self {
        TypeIds {
            state: TypeId::of::<Comp::State>(),
            message: TypeId::of::<Comp::Msg>(),
            event: TypeId::of::<Comp::Event>(),
        }
    }
    pub fn void() -> Self {
        let void = TypeId::of::<()>();
        TypeIds {
            state: void,
            message: void,
            event: void,
        }
    }
}

pub(crate) fn full_debug_name_of(
    parent: &Vec<Option<Cid>>,
    name: &Vec<&'static str>,
    id: Cid,
) -> String {
    let mut names: SmallVec<[&'static str; 10]> = SmallVec::new();
    names.push(name[id.get()]);
    let mut current = id;
    while let Some(parent) = parent[current.get()] {
        current = parent;
        names.push(name[parent.get()]);
    }
    names
        .iter()
        .rev()
        .fold(String::new(), |acc, n| acc + "/" + n)
}
