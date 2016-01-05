use super::{NodeMut, Node, NodeId, NodeRef};

impl<'a, T: 'a> NodeMut<'a, T> {
    fn node(&self) -> &Node<T> {
        self.tree.get_node_unchecked(self.id)
    }

    fn node_mut(&mut self) -> &mut Node<T> {
        self.tree.get_node_unchecked_mut(self.id)
    }

    /// Returns the value of the node.
    pub fn value(&mut self) -> &mut T {
        &mut self.node_mut().value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> NodeId<T> {
        self.id
    }

    /// Returns a mutator of the parent node.
    pub fn parent(self) -> Option<NodeMut<'a, T>> {
        let id = self.node().parent;
        id.map(move |id| self.tree.get_unchecked_mut(id))
    }

    /// Returns a mutator of the previous sibling.
    pub fn prev_sibling(self) -> Option<NodeMut<'a, T>> {
        let id = self.node().prev_sibling;
        id.map(move |id| self.tree.get_unchecked_mut(id))
    }

    /// Returns a mutator of the next sibling.
    pub fn next_sibling(self) -> Option<NodeMut<'a, T>> {
        let id = self.node().next_sibling;
        id.map(move |id| self.tree.get_unchecked_mut(id))
    }

    /// Returns a mutator of the first child.
    pub fn first_child(self) -> Option<NodeMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(id, _)| self.tree.get_unchecked_mut(id))
    }

    /// Returns a mutator of the last child.
    pub fn last_child(self) -> Option<NodeMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(_, id)| self.tree.get_unchecked_mut(id))
    }

    /// Returns true if node has no parent.
    pub fn is_orphan(&self) -> bool {
        self.node().parent.is_none()
    }

    /// Returns true if node has siblings.
    pub fn has_siblings(&self) -> bool {
        let node = self.node();
        node.prev_sibling.is_some() || node.next_sibling.is_some()
    }

    /// Returns true if node has children.
    pub fn has_children(&self) -> bool {
        self.node().children.is_some()
    }

    /// Appends a child node.
    pub fn append(&mut self, value: T) -> NodeId<T> {
        let id = self.tree.orphan(value).id();
        self.append_node(id);
        id
    }

    /// Appends an orphan node to this node's children.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refer to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    pub fn append_node(&mut self, id: NodeId<T>) {
        assert_eq!(self.tree.id, id.tree_id);
        assert!(self.tree.get_node_unchecked(id).parent.is_none());

        let last_child = self.tree.get_node_unchecked(self.id).children.map(|t| t.1);

        // Update new child.
        {
            let node = self.tree.get_node_unchecked_mut(id);
            node.parent = Some(self.id);
            node.prev_sibling = last_child;
        }

        // Update previous last child.
        if let Some(child) = last_child {
            let node = self.tree.get_node_unchecked_mut(child);
            node.next_sibling = Some(id);
        }

        // Update parent.
        let node = self.tree.get_node_unchecked_mut(self.id);
        if let Some((first, _)) = node.children {
            node.children = Some((first, id));
        } else {
            node.children = Some((id, id));
        }
    }
}

impl<'a, T: 'a> Into<NodeRef<'a, T>> for NodeMut<'a, T> {
    fn into(self) -> NodeRef<'a, T> {
        NodeRef {
            tree: self.tree,
            node: self.tree.get_node_unchecked(self.id),
            id: self.id,
        }
    }
}

impl<'a, T: 'a> Eq for NodeMut<'a, T> { }
impl<'a, T: 'a> PartialEq for NodeMut<'a, T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.tree as *const _ == other.tree as *const _
            && self.id == other.id
    }
}
