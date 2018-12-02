use crate::UiData;

pub struct UiInput(UiData);

impl UiInput {
    pub(crate) fn new(data: UiData) -> Self {
        UiInput(data)
    }

    pub(crate) fn unwrap(self) -> UiData {
        self.0
    }
}
