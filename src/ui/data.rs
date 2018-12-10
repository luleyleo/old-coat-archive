use crate::{Cid, Position, Size};
use crate::component::ComponentPointer;
use fnv::FnvHashMap;
use std::any::{Any, TypeId};

/// Contains all data that is necessary for the ui
#[derive(Default)]
pub struct UiData {
    /// The `TypeId` of the `Component` behind a `Cid`.
    pub(crate) typeid: Vec<TypeId>,
    /// A "pointer" to dynamic versions of a `Component`s functions
    pub(crate) pointer: Vec<ComponentPointer>,
    /// The parent of a `Component` might be None if it is
    /// the ui root or something went wrong.
    pub(crate) parent: Vec<Option<Cid>>,
    /// Lists all children of a `Component` as `Cid`s and
    /// is being used to render and layout the comps like a graph.
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
    /// The `Vec<Box<Any>>` is the alternative to a `Vec<Vec<Box<Any>`
    /// to avoid allocating for every message in exchange for a more confusing type.
    pub(crate) messages: Vec<Box<Any>>,

    /// The next `Cid` that will be allocated when needed.
    id_count: usize,
}

impl UiData {
    pub(crate) fn fresh_id(&mut self) -> Cid {
        let id = Cid::new(self.id_count);
        self.id_count += 1;

        self.typeid.push(TypeId::of::<()>());
        self.pointer.push(ComponentPointer::default());
        self.parent.push(None);
        self.children.push(Vec::new());
        self.creations.push(FnvHashMap::default());
        self.position.push(Position::default());
        self.size.push(Size::default());
        self.state.push(None);
        self.messages.push(Box::new(Vec::<()>::new()));

        id
    }
}
