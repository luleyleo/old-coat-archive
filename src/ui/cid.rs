#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cid(usize);

impl Cid {
    pub(crate) fn new(id: usize) -> Self {
        Cid(id)
    }

    pub(crate) fn invalid() -> Self {
        Cid(std::usize::MAX)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[macro_export]
macro_rules! ids {
    ($($id:ident),*,) => {
        ids!($($id),*);
    };
    ($($id:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            struct $id;
            impl crate::Named for $id {
                fn name() -> &'static str {
                    stringify!($id)
                }
            }
        )*
    };
}
