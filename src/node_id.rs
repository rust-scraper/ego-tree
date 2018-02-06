#![allow(expl_impl_clone_on_copy)]

use super::NodeId;
use std::hash::Hash;
use std::hash::Hasher;

impl<T> Hash for NodeId<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_usize(self.tree_id);
        state.write_usize(self.index);
    }
}

impl<T> Copy for NodeId<T> { }
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Eq for NodeId<T> { }
impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_id == other.tree_id && self.index == other.index
    }
}
