/// A wrapper to track wether a mutable reference has actually been mutated.
/// This is being used to avoid rendering when there was no state change.
/// It implements both `Deref` and `DerefMut`.
/// When `Deref` is used it won't trigger rendering but as soon as
/// its content is dereferenced mutably it will.
pub struct Mut<'a, D> {
    data: &'a mut D,
    mutated: &'a mut bool,
}

impl<'a, D> Mut<'a, D> {
    pub fn new(data: &'a mut D, mutated: &'a mut bool) -> Self {
        Mut { data, mutated }
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
        *self.mutated = true;
        self.data
    }
}
