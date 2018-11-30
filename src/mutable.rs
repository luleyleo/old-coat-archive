
pub struct Mut<'a, D> {
    data: &'a mut D,
    mutated: bool,
}

impl<'a, D> Mut<'a, D> {
    pub fn new(data: &'a mut D) -> Self {
        Mut {
            data,
            mutated: false,
        }
    }

    pub(crate) fn mutated(&self) -> bool {
        self.mutated
    }
}

impl<'a, D> std::ops::Deref for Mut<'a, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, D> std::ops::DerefMut for Mut<'a, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
