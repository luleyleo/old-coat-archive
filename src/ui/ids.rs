use std::any::TypeId;

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

#[derive(Clone, Copy, Debug)]
pub struct Iid {
    pub(crate) name: &'static str,
    pub(crate) id: TypeId,
}

impl Iid {
    pub fn new(name: &'static str, id: TypeId) -> Self {
        Iid { name, id }
    }
}

impl PartialEq for Iid {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for Iid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[macro_export]
macro_rules! ids {
    ($($id:ident),*,) => {
        ids!($($id),*);
    };
    ($($id:ident),*) => {
        $(
            let $id: Iid = Iid::new(
                stringify!($id),
                {
                    struct $id;
                    std::any::TypeId::of::<$id>()
                }
            );
        )*
    };
}
