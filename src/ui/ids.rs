use std::any::TypeId;

/// A "Component Identifier"
///
/// This is the id used to reference a components data
/// stored in the `UiData` struct.
///
/// It can be different every time an app runs and gets
/// generated at runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cid(usize);

impl Cid {
    pub(crate) fn new(id: usize) -> Self {
        Cid(id)
    }

    pub(crate) fn invalid() -> Self {
        Cid(std::usize::MAX)
    }

    pub(crate) fn get(&self) -> usize {
        self.0
    }
}

/// A "Intermediate Identifier"
///
/// This is used when `set`ing a component to identify it.
///
/// It has to be **unique per component** and gets generated
/// at compile time. Usually the `ids!()` macro takes care of this.
///
/// ```ignore
/// ids!(A, B, C);
///
/// let a: Iid = A;
/// let b: Iid = B;
///
/// Button::new()
///     ...
///     .set(A, ui);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Iid {
    pub(crate) name: &'static str,
    pub(crate) id: TypeId,
}

impl Iid {
    /// **Don't use this**
    /// 
    /// `Iid`s should be created using `ids!()` but this has to
    /// be public to make the `ids!()` macro work from other crates
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

/// This macro simplifies creating `Iid`s.
///
/// ```ignore
/// ids!(Container, Count, AddButton, SubButton);
///
/// Button::new()
///     ...
///     .set(AddButton, ui);
/// ```
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
