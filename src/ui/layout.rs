use crate::{BoxConstraints, Cid, Size, UiData};
use log::trace;

pub struct UiLayout<'a> {
    data: &'a mut UiData,
    current: Cid,
}

impl<'a> UiLayout<'a> {

    pub(crate) fn run(data: &'a mut UiData, root: Cid, window_size: Size) {
        trace!("Running `UiLayout`");

        let mut ui = UiLayout {
            data,
            current: Cid::invalid(),
        };
        
        ui.size(root, BoxConstraints::tight(window_size));
    }

    pub fn size(&mut self, child: Cid, constraints: BoxConstraints) -> Size {
        let layout = self.data.pointer[child.get()].layout;

        // As long as this function is being called there is no other
        // way to access the `UiData` and thus it should be safe
        // to use a pointer to the children and pass a mutable
        // reference `self` to the layout function
        let children = &self.data.children[child.get()] as *const Vec<Cid>;

        let previous = self.current;
        self.current = child;
        let proposed = layout(constraints, unsafe { &*children }, self);
        self.current = previous;

        let size = constraints.check_size(proposed);

        self.data.size[child.get()] = size;

        size
    }

    pub fn full_debug_name(&self) -> String {
        self.data.full_debug_name_of(self.current)
    }
}
