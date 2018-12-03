use crate::{BoxConstraints, Cid, Size, UiData};

pub struct UiLayout<'a> {
    data: &'a mut UiData,
}

impl<'a> UiLayout<'a> {
    pub(crate) fn new(data: &'a mut UiData) -> Self {
        UiLayout { data }
    }

    pub fn size(&mut self, child: Cid, constraints: BoxConstraints) -> Size {
        let layout = self.data.pointer[child.get()].layout;

        // As long as this function is being called there is no other
        // way to access the `UiData` and thus it should be safe
        // to use a pointer to the children and pass a mutable
        // reference `self` to the layout function
        let children = &self.data.children[child.get()] as *const Vec<Cid>;

        let proposed = layout(constraints, unsafe { &*children }, self);

        constraints.check_size(proposed)
    }
}
