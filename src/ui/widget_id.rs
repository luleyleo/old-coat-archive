#[derive(Clone, Copy, Debug)]
pub struct WidgetId(u32);

impl WidgetId {
    pub(crate) fn new(id: u32) -> Self {
        WidgetId(id)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[macro_export]
macro_rules! ids_next {
    ($val:expr, $id:ident) => {
        const $id: u32 = $val + 1;
    };
    ($val:expr, $id:ident, $($ids:ident),+) => {
        const $id: u32 = $val + 1;
        ids_next!($val + 1, $($ids),+);
    };
}

#[macro_export]
macro_rules! ids {
    ($($ids:ident),*) => {
        ids_next!((line!() * 1000000), $($ids),+);
    };
}
