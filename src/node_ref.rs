use super::{NodeRef, NodeId};

impl<'a, T: 'a> NodeRef<'a, T> {
    /// Returns the value of the node.
    pub fn value(&self) -> &T {
        &self.node.value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> NodeId<T> {
        self.id
    }

    /// Returns a reference to the parent node.
    pub fn parent(&self) -> Option<NodeRef<'a, T>> {
        self.node.parent.map(|id| self.tree.get_unchecked(id))
    }

    /// Returns a reference to the previous sibling.
    pub fn prev_sibling(&self) -> Option<NodeRef<'a, T>> {
        self.node.prev_sibling.map(|id| self.tree.get_unchecked(id))
    }

    /// Returns a reference to the next sibling.
    pub fn next_sibling(&self) -> Option<NodeRef<'a, T>> {
        self.node.next_sibling.map(|id| self.tree.get_unchecked(id))
    }

    /// Returns a reference to the first child.
    pub fn first_child(&self) -> Option<NodeRef<'a, T>> {
        self.node.children.map(|(id, _)| self.tree.get_unchecked(id))
    }

    /// Returns a reference to the last child.
    pub fn last_child(&self) -> Option<NodeRef<'a, T>> {
        self.node.children.map(|(_, id)| self.tree.get_unchecked(id))
    }

    /// Returns true if node has no parent.
    pub fn is_orphan(&self) -> bool {
        self.node.parent.is_none()
    }

    /// Returns true if node has siblings.
    pub fn has_siblings(&self) -> bool {
        self.node.prev_sibling.is_some() || self.node.next_sibling.is_some()
    }

    /// Returns true if node has children.
    pub fn has_children(&self) -> bool {
        self.node.children.is_some()
    }
}

impl<'a, T: 'a> Copy for NodeRef<'a, T> { }
impl<'a, T: 'a> Clone for NodeRef<'a, T> {
    fn clone(&self) -> Self {
        NodeRef {
            tree: self.tree,
            node: self.node,
            id: self.id,
        }
    }
}

impl<'a, T: 'a> Eq for NodeRef<'a, T> { }
impl<'a, T: 'a> PartialEq for NodeRef<'a, T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.tree as *const _ == other.tree as *const _
            && self.node as *const _ == other.node as *const _
            && self.id == other.id
    }
}
