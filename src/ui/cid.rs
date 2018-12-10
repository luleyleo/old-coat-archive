#[derive(Clone, Copy, Debug)]
pub struct Cid(usize);

impl Cid {
    pub(crate) fn new(id: usize) -> Self {
        Cid(id)
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
        )*
    };
}
