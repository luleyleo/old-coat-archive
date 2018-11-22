use std::any::{Any, TypeId};
use fnv::FnvHashMap;
use crate::{WidgetId, Position, Size};
use crate::component::ComponentPointer;

pub struct WidgetData {
    typeid: Vec<TypeId>,
    component: Vec<ComponentPointer>,
    parent: Vec<WidgetId>,
    children: Vec<FnvHashMap<usize, WidgetId>>,
    position: Vec<Position>,
    size: Vec<Size>,
    props: Vec<Box<Any>>,
    state: Vec<Box<Any>>,
}
