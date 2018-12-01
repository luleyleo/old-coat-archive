use std::any::{Any, TypeId};
use fnv::FnvHashMap;
use crate::{WidgetId, Position, Size};

#[derive(Default)]
pub struct WidgetData {
    pub(crate) typeid: Vec<TypeId>,
    pub(crate) parent: Vec<Option<WidgetId>>,
    pub(crate) children: Vec<Vec<WidgetId>>,
    pub(crate) creations: Vec<FnvHashMap<usize, WidgetId>>,
    pub(crate) position: Vec<Position>,
    pub(crate) size: Vec<Size>,
    pub(crate) state: Vec<Option<Box<Any>>>,
    pub(crate) id_count: usize,
}

impl WidgetData {
    pub fn fresh_id(&mut self) -> WidgetId {
        let id = WidgetId::new(self.id_count);
        self.id_count += 1;

        self.typeid.push(TypeId::of::<()>());
        self.parent.push(None);
        self.children.push(Vec::new());
        self.creations.push(FnvHashMap::default());
        self.position.push(Position::default());
        self.size.push(Size::default());
        self.state.push(None);

        id
    }
}
