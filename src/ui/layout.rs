use crate::{BoxConstraints, Cid, Size, UiData};

pub struct UiLayout<'a> {
    data: &'a mut UiData,
}

impl<'a> UiLayout<'a> {
    pub(crate) fn new(data: &'a mut UiData) -> Self {
        UiLayout { data }
    }

    pub fn size(&mut self, child: Cid, constraints: BoxConstraints) -> Size {
        Size::default()
    }
}
