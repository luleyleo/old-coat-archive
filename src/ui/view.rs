use crate::UiData;

pub struct UiView(UiData);

impl UiView {
    pub(crate) fn new(data: UiData) -> Self {
        UiView(data)
    }

    pub(crate) fn unwrap(self) -> UiData {
        self.0
    }
}
