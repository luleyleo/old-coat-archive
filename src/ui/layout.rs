use crate::UiData;

pub struct UiLayout(UiData);

impl UiLayout {
    pub(crate) fn new(data: UiData) -> Self {
        UiLayout(data)
    }

    pub(crate) fn unwrap(self) -> UiData {
        self.0
    }
}
