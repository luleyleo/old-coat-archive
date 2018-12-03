use crate::UiData;

pub struct UiInput<'a> {
    data: &'a mut UiData,
}

impl<'a> UiInput<'a> {
    pub(crate) fn new(data: &'a mut UiData) -> Self {
        UiInput { data }
    }
}
