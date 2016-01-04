use std::marker::PhantomData;

/// A tree of nodes stored in a `Vec` and referenced by IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    vec: Vec<Node<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<T> {
    value: T,
    parent: Option<Id<T>>,
    prev_sibling: Option<Id<T>>,
    next_sibling: Option<Id<T>>,
    children: Option<(Id<T>, Id<T>)>,
}

/// A node ID.
#[derive(Debug)]
pub struct Id<T> {
    id: usize,
    data: PhantomData<T>,
}

/// A node reference.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ref<'a, T: 'a> {
    tree: &'a Tree<T>,
    node: &'a Node<T>,
    id: Id<T>,
}

/// A mutable node reference.
#[derive(Debug, PartialEq, Eq)]
pub struct RefMut<'a, T: 'a> {
    tree: &'a mut Tree<T>,
    id: Id<T>,
}

impl<T> Tree<T> {
    /// Creates a new tree with a root node.
    pub fn new(root: T) -> Self {
        Tree { vec: vec![Node::new(root)] }
    }

    /// Creates a new tree of the specified capacity with a root node.
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.push(Node::new(root));
        Tree { vec: vec }
    }

    /// Returns a reference to the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exist in this tree.
    pub fn get(&self, id: Id<T>) -> Ref<T> {
        Ref {
            tree: self,
            node: &self.vec[id.id],
            id: id,
        }
    }
    /// Returns a mutable reference to the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exist in this tree.
    pub fn get_mut(&mut self, id: Id<T>) -> RefMut<T> {
        // Check that id is valid.
        {
            &self.vec[id.id];
        }

        RefMut {
            tree: self,
            id: id,
        }
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> Ref<T> {
        self.get(Id { id: 0, data: PhantomData })
    }

    /// Returns a mutable reference to the root node.
    pub fn root_mut(&mut self) -> RefMut<T> {
        self.get_mut(Id { id: 0, data: PhantomData })
    }

    /// Creates a new orphan node.
    pub fn orphan(&mut self, value: T) -> RefMut<T> {
        let id = Id { id: self.vec.len(), data: PhantomData };
        self.vec.push(Node::new(value));
        self.get_mut(id)
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
        }
    }
}

// Manual implementations that don't care about T.
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self { Id { id: self.id, data: PhantomData } }
}
impl<T> Copy for Id<T> { }
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<T> Eq for Id<T> { }

impl<'a, T: 'a> Ref<'a, T> {
    /// Returns the value of the node.
    pub fn value(&self) -> &T {
        &self.node.value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> Id<T> {
        self.id
    }

    /// Returns a reference to the parent node.
    pub fn parent(&self) -> Option<Ref<T>> {
        self.node.parent.map(|id| self.tree.get(id))
    }

    /// Returns a reference to the first child node.
    pub fn first_child(&self) -> Option<Ref<T>> {
        self.node.children.map(|(id, _)| self.tree.get(id))
    }

    /// Returns a reference to the last child node.
    pub fn last_child(&self) -> Option<Ref<T>> {
        self.node.children.map(|(_, id)| self.tree.get(id))
    }

    /// Returns a reference to the next sibling node.
    pub fn next_sibling(&self) -> Option<Ref<T>> {
        self.node.next_sibling.map(|id| self.tree.get(id))
    }

    /// Returns a reference to the previous sibling node.
    pub fn prev_sibling(&self) -> Option<Ref<T>> {
        self.node.prev_sibling.map(|id| self.tree.get(id))
    }
}

impl<'a, T: 'a> RefMut<'a, T> {
    /// Returns the value of the node.
    pub fn value(&mut self) -> &mut T {
        &mut self.tree.vec[self.id.id].value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> Id<T> {
        self.id
    }

    /// Appends an orphan node to this node's children.
    pub fn append_node(&mut self, id: Id<T>) {
        let last_child = self.tree.vec[self.id.id].children.map(|t| t.1);

        // Update new node.
        {
            let node = &mut self.tree.vec[id.id];
            node.parent = Some(self.id);
            node.prev_sibling = last_child;
        }

        // Update previous last child.
        if let Some(child) = last_child {
            let node = &mut self.tree.vec[child.id];
            node.next_sibling = Some(id);
        }

        // Update parent.
        {
            let node = &mut self.tree.vec[self.id.id];
            if let Some((first, _)) = node.children {
                node.children = Some((first, id));
            } else {
                node.children = Some((id, id));
            }
        }
    }

    /// Appends a child node.
    pub fn append(&mut self, value: T) {
        let id = self.tree.orphan(value).id();
        self.append_node(id);
    }
}
