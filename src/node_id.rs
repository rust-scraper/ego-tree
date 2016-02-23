#![allow(expl_impl_clone_on_copy)]

use super::NodeId;

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
