use std::any::{Any, TypeId};
use fnv::FnvHashMap;
use crate::{WidgetId, Position, Size};

pub struct WidgetData {
    typeid: Vec<TypeId>,
    parent: Vec<WidgetId>,
    children: Vec<FnvHashMap<usize, WidgetId>>,
    position: Vec<Position>,
    size: Vec<Size>,
    state: Vec<Option<Box<Any>>>,
}
