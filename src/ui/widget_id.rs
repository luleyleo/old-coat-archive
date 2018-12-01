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
macro_rules! ids {
    ($id:ident, $($ids:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Ids {
            $id = line!() as isize * 1000000,
            $($ids),*
        }
        impl Into<isize> for Ids {
            fn into(self) -> isize {
                self as isize
            }
        }
        // because `use Ids::*` is not possible:
        const $id: Ids = Ids::$id;
        $(
        const $ids: Ids = Ids::$ids;
        )*
    };
}
