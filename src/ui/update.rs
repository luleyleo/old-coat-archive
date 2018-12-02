use crate::UiData;

pub struct UiUpdate(UiData);

impl UiUpdate {
    pub(crate) fn new(data: UiData) -> Self {
        UiUpdate(data)
    }

    pub(crate) fn unwrap(self) -> UiData {
        self.0
    }
}
