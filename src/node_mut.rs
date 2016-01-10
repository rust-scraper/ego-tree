use super::{NodeMut, Node, NodeId, NodeRef};

impl<'a, T: 'a> NodeMut<'a, T> {
    fn node(&self) -> &Node<T> {
        self.tree.get_node_unchecked(self.index)
    }

    fn node_mut(&mut self) -> &mut Node<T> {
        self.tree.get_node_unchecked_mut(self.index)
    }

    /// Returns the value of this node.
    pub fn value(&mut self) -> &mut T {
        &mut self.node_mut().value
    }

    /// Returns the ID of this node.
    pub fn id(&self) -> NodeId<T> {
        self.tree.node_id(self.index)
    }

    /// Returns a mutator of this node's parent.
    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().parent;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of this node's previous sibling.
    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().prev_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of this node's next sibling.
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        let index = self.node().next_sibling;
        index.map(move |i| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of this node's first child.
    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        let children = self.node().children;
        children.map(move |(i, _)| self.tree.get_unchecked_mut(i))
    }

    /// Returns a mutator of this node's last child.
    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        let children = self.node().children;
        children.map(move |(_, i)| self.tree.get_unchecked_mut(i))
    }

    /// Returns true if this node has siblings.
    pub fn has_siblings(&self) -> bool {
        let node = self.node();
        node.prev_sibling.is_some() || node.next_sibling.is_some()
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        self.node().children.is_some()
    }

    /// Appends a new node to this node's children, returning a mutator of the new node.
    pub fn append(&mut self, value: T) -> NodeMut<T> {
        let index = self.tree.orphan(value).index;
        self.append_unchecked(index)
    }

    /// Prepends a new node to this node's children, returning a mutator of the new node.
    pub fn prepend(&mut self, value: T) -> NodeMut<T> {
        let index = self.tree.orphan(value).index;
        self.prepend_unchecked(index)
    }

    /// Inserts a new sibling before this node, returning a mutator of the new node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_before(&mut self, value: T) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.orphan(value).index;
        self.insert_before_unchecked(index)
    }

    /// Inserts a new sibling before this node, returning a mutator of the new node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_after(&mut self, value: T) -> NodeMut<T> {
        assert!(self.node().parent.is_some());
        let index = self.tree.orphan(value).index;
        self.insert_after_unchecked(index)
    }

    /// Detaches this node from its parent.
    ///
    /// If this node is an orphan, does nothing.
    pub fn detach(&mut self) {
        self.detach_unchecked();
    }

    /// Appends a node to this node's children by ID, returning a mutator of the referenced node.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn append_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        self.append_unchecked(index)
    }

    /// Prepends a node to this node's children by ID, returning a mutator of the referenced node.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn prepend_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        self.prepend_unchecked(index)
    }

    /// Inserts a sibling before this node by ID, returning a mutator of the referenced node.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn insert_id_before(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        self.insert_before_unchecked(index)
    }

    /// Inserts a sibling after this node by ID, returning a mutator of the referenced node.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn insert_id_after(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let index = self.tree.validate_id(id);
        self.insert_after_unchecked(index)
    }

    /// Reparents the children of a node by ID, appending them to this node's children.
    ///
    /// If the referenced node does not have children, does nothing.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn reparent_from_id_append(&mut self, id: NodeId<T>) {
        let index = self.tree.validate_id(id);
        self.reparent_append_unchecked(index);
    }

    /// Reparents the children of a node by ID, prepending them to this node's children.
    ///
    /// If the referenced node does not have children, does nothing.
    ///
    /// May cause cycles, which can cause unsafety in other operations.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub unsafe fn reparent_from_id_prepend(&mut self, id: NodeId<T>) {
        let index = self.tree.validate_id(id);
        self.reparent_prepend_unchecked(index);
    }

    fn append_unchecked(&mut self, new_child_index: usize) -> NodeMut<T> {
        let last_child_index = self.node().children.map(|t| t.1);

        {
            let new_child = self.tree.get_node_unchecked_mut(new_child_index);
            new_child.parent = Some(self.index);
            new_child.prev_sibling = last_child_index;
        }

        if let Some(last_child_index) = last_child_index {
            let last_child = self.tree.get_node_unchecked_mut(last_child_index);
            last_child.next_sibling = Some(new_child_index);
        }

        {
            let node = self.node_mut();
            if let Some((first_child_index, _)) = node.children {
                node.children = Some((first_child_index, new_child_index));
            } else {
                node.children = Some((new_child_index, new_child_index));
            }
        }

        self.tree.get_unchecked_mut(new_child_index)
    }

    fn prepend_unchecked(&mut self, new_child_index: usize) -> NodeMut<T> {
        let first_child_index = self.node().children.map(|t| t.0);

        {
            let new_child = self.tree.get_node_unchecked_mut(new_child_index);
            new_child.parent = Some(self.index);
            new_child.next_sibling = first_child_index;
        }

        if let Some(first_child_index) = first_child_index {
            let first_child = self.tree.get_node_unchecked_mut(first_child_index);
            first_child.prev_sibling = Some(new_child_index);
        }

        {
            let node = self.node_mut();
            if let Some((_, last_child_index)) = node.children {
                node.children = Some((new_child_index, last_child_index));
            } else {
                node.children = Some((new_child_index, new_child_index));
            }
        }

        self.tree.get_unchecked_mut(new_child_index)
    }

    fn insert_before_unchecked(&mut self, new_sibling_index: usize) -> NodeMut<T> {
        let parent_index = self.node().parent;
        let prev_sibling_index = self.node().prev_sibling;

        {
            let new_sibling = self.tree.get_node_unchecked_mut(new_sibling_index);
            new_sibling.parent = parent_index;
            new_sibling.prev_sibling = prev_sibling_index;
            new_sibling.next_sibling = Some(self.index);
        }

        if let Some(prev_sibling_index) = prev_sibling_index {
            let prev_sibling = self.tree.get_node_unchecked_mut(prev_sibling_index);
            prev_sibling.next_sibling = Some(new_sibling_index);
        }

        self.node_mut().prev_sibling = Some(new_sibling_index);

        {
            let parent = self.tree.get_node_unchecked_mut(parent_index.unwrap());
            let (first_child_index, last_child_index) = parent.children.unwrap();
            if first_child_index == self.index {
                parent.children = Some((new_sibling_index, last_child_index));
            }
        }

        self.tree.get_unchecked_mut(new_sibling_index)
    }

    fn insert_after_unchecked(&mut self, new_sibling_index: usize) -> NodeMut<T> {
        let parent_index = self.node().parent;
        let next_sibling_index = self.node().next_sibling;

        {
            let new_sibling = self.tree.get_node_unchecked_mut(new_sibling_index);
            new_sibling.parent = parent_index;
            new_sibling.prev_sibling = Some(self.index);
            new_sibling.next_sibling = next_sibling_index;
        }

        if let Some(next_sibling_index) = next_sibling_index {
            let next_sibling = self.tree.get_node_unchecked_mut(next_sibling_index);
            next_sibling.prev_sibling = Some(new_sibling_index);
        }

        self.node_mut().next_sibling = Some(new_sibling_index);

        {
            let parent = self.tree.get_node_unchecked_mut(parent_index.unwrap());
            let (first_child_index, last_child_index) = parent.children.unwrap();
            if last_child_index == self.index {
                parent.children = Some((first_child_index, new_sibling_index));
            }
        }

        self.tree.get_unchecked_mut(new_sibling_index)
    }

    fn detach_unchecked(&mut self) {
        let parent_index = match self.node().parent {
            Some(index) => index,
            None => return,
        };
        let prev_sibling_index = self.node().prev_sibling;
        let next_sibling_index = self.node().next_sibling;

        {
            let node = self.node_mut();
            node.parent = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }

        if let Some(prev_sibling_index) = prev_sibling_index {
            let prev_sibling = self.tree.get_node_unchecked_mut(prev_sibling_index);
            prev_sibling.next_sibling = next_sibling_index;
        }

        if let Some(next_sibling_index) = next_sibling_index {
            let next_sibling = self.tree.get_node_unchecked_mut(next_sibling_index);
            next_sibling.prev_sibling = prev_sibling_index;
        }

        let parent = self.tree.get_node_unchecked_mut(parent_index);
        let (first_child_index, last_child_index) = parent.children.unwrap();
        if first_child_index == last_child_index {
            parent.children = None;
        } else if first_child_index == self.index {
            parent.children = Some((next_sibling_index.unwrap(), last_child_index));
        } else if last_child_index == self.index {
            parent.children = Some((first_child_index, prev_sibling_index.unwrap()));
        }
    }

    fn reparent_append_unchecked(&mut self, from_index: usize) {
        let new_children = match self.tree.get_node_unchecked_mut(from_index).children.take() {
            Some(indexes) => indexes,
            None => return,
        };

        if self.node().children.is_none() {
            self.node_mut().children = Some(new_children);
            return;
        }

        let last_child_index = self.node().children.unwrap().1;

        {
            let last_child = self.tree.get_node_unchecked_mut(last_child_index);
            last_child.next_sibling = Some(new_children.0);
        }

        {
            let first_new_child = self.tree.get_node_unchecked_mut(new_children.0);
            first_new_child.prev_sibling = Some(last_child_index);
        }

        let node = self.node_mut();
        let first_child_index = node.children.unwrap().0;
        node.children = Some((first_child_index, new_children.1));
    }

    fn reparent_prepend_unchecked(&mut self, from_index: usize) {
        let new_children = match self.tree.get_node_unchecked_mut(from_index).children.take() {
            Some(indexes) => indexes,
            None => return,
        };

        if self.node().children.is_none() {
            self.node_mut().children = Some(new_children);
            return;
        }

        let first_child_index = self.node().children.unwrap().0;

        {
            let first_child = self.tree.get_node_unchecked_mut(first_child_index);
            first_child.prev_sibling = Some(new_children.1);
        }

        {
            let last_new_child = self.tree.get_node_unchecked_mut(new_children.1);
            last_new_child.next_sibling = Some(first_child_index);
        }

        let node = self.node_mut();
        let last_child_index = node.children.unwrap().1;
        node.children = Some((new_children.0, last_child_index));
    }
}

impl<'a, T: 'a> Into<NodeRef<'a, T>> for NodeMut<'a, T> {
    fn into(self) -> NodeRef<'a, T> {
        self.tree.get_unchecked(self.index)
    }
}
