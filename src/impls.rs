use super::{Tree, Ref, RefMut};

impl<T: Default> Default for Tree<T> {
    fn default() -> Self {
        Tree::new(T::default())
    }
}

impl<'a, T: 'a> Into<Ref<'a, T>> for RefMut<'a, T> {
    fn into(self) -> Ref<'a, T> {
        self.tree.get_unchecked(self.id)
    }
}
