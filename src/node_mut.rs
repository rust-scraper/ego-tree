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
    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().parent;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the previous sibling.
    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().prev_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the next sibling.
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().next_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the first child.
    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        let children = self.node().children;
        children.map(move |(i, _)| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of the last child.
    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
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
    pub fn append(&mut self, value: T) -> NodeMut<T> {
        let index = self.tree.orphan(value).index;
        self.append_unchecked(index)
    }

    /// Prepends a child node.
    pub fn prepend(&mut self, value: T) -> NodeMut<T> {
        let index = self.tree.orphan(value).index;
        self.prepend_unchecked(index)
    }

    /// Inserts a sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if node is an orphan.
    pub fn insert_before(&mut self, value: T) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.orphan(value).index;
        self.insert_before_unchecked(index)
    }

    /// Inserts a sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if node is an orphan.
    pub fn insert_after(&mut self, value: T) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.orphan(value).index;
        self.insert_before_unchecked(index)
    }

    /// Detaches from parent.
    ///
    /// If node has no parent, does nothing.
    pub fn detach(&mut self) {
        self.detach_unchecked();
    }

    /// Appends a child node by ID.
    ///
    /// May cause a cycle.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refer to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    pub unsafe fn append_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        assert!(self.tree.get_node_unchecked(index).parent.is_none());
        self.append_unchecked(index)
    }

    /// Prepends a child node by ID.
    ///
    /// May cause a cycle.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refer to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    pub unsafe fn prepend_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        assert!(self.tree.get_node_unchecked(index).parent.is_none());
        self.prepend_unchecked(index)
    }

    /// Inserts a sibling by ID before this node.
    ///
    /// May cause a cycle.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refer to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    /// - Panics if this node is an orphan.
    pub unsafe fn insert_id_before(&mut self, id: NodeId<T>) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.validate_id(id);
        assert!(self.tree.get_node_unchecked(index).parent.is_none());
        self.insert_before_unchecked(index)
    }

    /// Inserts a sibling by ID after this node.
    ///
    /// May cause a cycle.
    ///
    /// # Panics
    ///
    /// - Panics if `id` does not refer to a node in this tree.
    /// - Panics if the node referenced by `id` is not an orphan.
    /// - Panics if this node is an orphan.
    pub unsafe fn insert_id_after(&mut self, id: NodeId<T>) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.validate_id(id);
        assert!(self.tree.get_node_unchecked(index).parent.is_none());
        self.insert_after_unchecked(index)
    }

    fn append_unchecked(&mut self, index: usize) -> NodeMut<T> {
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
        {
            let node = self.node_mut();
            if let Some((first, _)) = node.children {
                node.children = Some((first, index));
            } else {
                node.children = Some((index, index));
            }
        }

        self.tree.get_unchecked_mut(index)
    }

    fn prepend_unchecked(&mut self, index: usize) -> NodeMut<T> {
        let first_child = self.node().children.map(|t| t.0);

        // Update new child.
        {
            let node = self.tree.get_node_unchecked_mut(index);
            node.parent = Some(self.index);
            node.next_sibling = first_child;
        }

        // Update previous first child.
        if let Some(child_index) = first_child {
            let node = self.tree.get_node_unchecked_mut(child_index);
            node.prev_sibling = Some(index);
        }

        // Update parent.
        {
            let node = self.node_mut();
            if let Some((_, last)) = node.children {
                node.children = Some((index, last));
            } else {
                node.children = Some((index, index));
            }
        }

        self.tree.get_unchecked_mut(index)
    }

    fn insert_before_unchecked(&mut self, index: usize) -> NodeMut<T> {
        let parent = self.node().parent;
        let prev_sibling = self.node().prev_sibling;

        // Update new sibling.
        {
            let node = self.tree.get_node_unchecked_mut(index);
            node.parent = parent;
            node.next_sibling = Some(self.index);
            node.prev_sibling = prev_sibling;
        }

        // Update previous sibling.
        if let Some(prev_sibling) = prev_sibling {
            let node = self.tree.get_node_unchecked_mut(prev_sibling);
            node.next_sibling = Some(index);
        }

        // Update self.
        self.node_mut().prev_sibling = Some(index);

        // Update parent.
        {
            let node = self.tree.get_node_unchecked_mut(parent.unwrap());
            let (first_child, last_child) = node.children.unwrap();
            if first_child == self.index {
                node.children = Some((index, last_child));
            }
        }

        self.tree.get_unchecked_mut(index)
    }

    fn insert_after_unchecked(&mut self, index: usize) -> NodeMut<T> {
        let parent = self.node().parent;
        let next_sibling = self.node().next_sibling;

        // Update new sibling.
        {
            let node = self.tree.get_node_unchecked_mut(index);
            node.parent = parent;
            node.prev_sibling = Some(self.index);
            node.next_sibling = next_sibling;
        }

        // Update next sibling.
        if let Some(next_sibling) = next_sibling {
            let node = self.tree.get_node_unchecked_mut(next_sibling);
            node.prev_sibling = Some(index);
        }

        // Update self.
        self.node_mut().next_sibling = Some(index);

        // Update parent.
        {
            let node = self.tree.get_node_unchecked_mut(parent.unwrap());
            let (first_child, last_child) = node.children.unwrap();
            if last_child == self.index {
                node.children = Some((first_child, index));
            }
        }

        self.tree.get_unchecked_mut(index)
    }

    fn detach_unchecked(&mut self) {
        let parent = match self.node().parent {
            Some(index) => index,
            None => return,
        };

        let next_sibling = self.node().next_sibling;
        let prev_sibling = self.node().prev_sibling;

        // Update self.
        {
            let node = self.node_mut();
            node.parent = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }

        // Update previous sibling.
        if let Some(index) = prev_sibling {
            let node = self.tree.get_node_unchecked_mut(index);
            node.next_sibling = next_sibling;
        }

        // Update next sibling.
        if let Some(index) = next_sibling {
            let node = self.tree.get_node_unchecked_mut(index);
            node.prev_sibling = prev_sibling;
        }

        // Update parent.
        let parent = self.tree.get_node_unchecked_mut(parent);
        let (first_child, last_child) = parent.children.unwrap();
        if first_child == last_child {
            parent.children = None;
        } else if first_child == self.index {
            parent.children = Some((next_sibling.unwrap(), last_child));
        } else if last_child == self.index {
            parent.children = Some((first_child, prev_sibling.unwrap()));
        }
    }
}

impl<'a, T: 'a> Into<NodeRef<'a, T>> for NodeMut<'a, T> {
    fn into(self) -> NodeRef<'a, T> {
        self.tree.get_unchecked(self.index)
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
