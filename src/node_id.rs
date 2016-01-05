use std::marker::PhantomData;

use super::NodeId;

impl<T> Copy for NodeId<T> { }
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        NodeId {
            tree_id: self.tree_id,
            index: self.index,
            data: PhantomData,
        }
    }
}

impl<T> Eq for NodeId<T> { }
impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_id == other.tree_id && self.index == other.index
    }
}
