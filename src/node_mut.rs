use super::{NodeMut, Node, NodeId, NodeRef};

impl<'a, T: 'a> NodeMut<'a, T> {
    fn node(&self) -> &Node<T> {
        self.tree.get_node_unchecked(self.index)
    }

    fn node_mut(&mut self) -> &mut Node<T> {
        self.tree.get_node_unchecked_mut(self.index)
    }

    /// Returns the value of the node.
    pub fn value(&mut self) -> &mut T {
        &mut self.node_mut().value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> NodeId<T> {
        self.tree.node_id(self.index)
    }

    /// Returns a mutator of the parent node.
    pub fn parent(self) -> Option<NodeMut<'a, T>> {
        let index = self.node().parent;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the previous sibling.
    pub fn prev_sibling(self) -> Option<NodeMut<'a, T>> {
        let index = self.node().prev_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the next sibling.
    pub fn next_sibling(self) -> Option<NodeMut<'a, T>> {
        let index = self.node().next_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the first child.
    pub fn first_child(self) -> Option<NodeMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(i, _)| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the last child.
    pub fn last_child(self) -> Option<NodeMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(_, i)| self.tree.get_unchecked_mut(i))
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
        let index = self.tree.orphan(value).index;
        self.append_unchecked(index);
        self.tree.node_id(index)
    }

    /// Appends a child node by ID.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refere to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    pub fn append_id(&mut self, id: NodeId<T>) {
        let index = self.tree.validate_id(id);
        assert!(self.tree.get_node_unchecked(index).parent.is_none());
        self.append_unchecked(index);
    }

    fn append_unchecked(&mut self, index: usize) {
        let last_child = self.node().children.map(|t| t.1);

        // Update new child.
        {
            let node = self.tree.get_node_unchecked_mut(index);
            node.parent = Some(self.index);
            node.prev_sibling = last_child;
        }

        // Update previous last child.
        if let Some(child_index) = last_child {
            let node = self.tree.get_node_unchecked_mut(child_index);
            node.next_sibling = Some(index);
        }

        // Update parent.
        let node = self.node_mut();
        if let Some((first, _)) = node.children {
            node.children = Some((first, index));
        } else {
            node.children = Some((index, index));
        }
    }
}

impl<'a, T: 'a> Into<NodeRef<'a, T>> for NodeMut<'a, T> {
    fn into(self) -> NodeRef<'a, T> {
        NodeRef {
            tree: self.tree,
            node: self.tree.get_node_unchecked(self.index),
            index: self.index,
        }
    }
}

impl<'a, T: 'a> Eq for NodeMut<'a, T> { }
impl<'a, T: 'a> PartialEq for NodeMut<'a, T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.tree as *const _ == other.tree as *const _
            && self.index == other.index
    }
}
