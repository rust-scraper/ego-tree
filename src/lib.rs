//! Vec-backed ID-tree.
//!
//! # Behavior
//!
//! - Trees have at least a root node;
//! - Nodes have zero or more ordered children;
//! - Nodes have at most one parent;
//! - Nodes can be detached (orphaned) but not removed;
//! - Node parent, next sibling, previous sibling, first child and last child
//!   can be accessed in constant time;
//! - All methods perform in constant time;
//! - All iterators perform in linear time.
//!
//! # Examples
//!
//! ```
//! let mut tree = ego_tree::Tree::new('a');
//! let mut root = tree.root_mut();
//! root.append('b');
//! let mut c = root.append('c');
//! c.append('d');
//! c.append('e');
//! ```
//!
//! ```
//! #[macro_use] extern crate ego_tree;
//! # fn main() {
//! let tree = tree!('a' => { 'b', 'c' => { 'd', 'e' } });
//! # }
//! ```

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
)]

use std::marker::PhantomData;
use std::fmt::{self, Debug, Formatter};

/// Vec-backed ID-tree.
///
/// Always contains at least a root node.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tree<T> {
    vec: Vec<Node<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<T> {
    parent: Option<Id>,
    prev_sibling: Option<Id>,
    next_sibling: Option<Id>,
    children: Option<(Id, Id)>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            value,
        }
    }
}

/// Node reference.
#[derive(Debug)]
pub struct NodeRef<'a, T: 'a> {
    id: Id,
    tree: &'a Tree<T>,
    node: &'a Node<T>,
}

/// Node mutator.
#[derive(Debug)]
pub struct NodeMut<'a, T: 'a> {
    id: Id,
    tree: &'a mut Tree<T>,
}

/// Opaque node ID.
///
/// Acts as a weak reference not tied to the lifetime of the tree. Using a
/// `NodeId` is therefore unsafe.
#[derive(Debug)]
pub struct NodeId<T> {
    id: Id,
    marker: PhantomData<T>,
}

// Trait implementations regardless of T.

impl<'a, T: 'a> Copy for NodeRef<'a, T> { }
impl<'a, T: 'a> Clone for NodeRef<'a, T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Copy for NodeId<T> { }
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self { *self }
}

impl<'a, T: 'a> Eq for NodeRef<'a, T> { }
impl<'a, T: 'a> PartialEq for NodeRef<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.tree as *const _ == other.tree as *const _
            && self.node as *const _ == other.node as *const _
    }
}

impl<T> Eq for NodeId<T> { }
impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> std::hash::Hash for NodeId<T> {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        std::hash::Hash::hash(&self.id, state)
    }
}

impl<T> Tree<T> {
    /// Creates a tree with a root node.
    pub fn new(root: T) -> Self {
        Tree { vec: vec![Node::new(root)] }
    }

    /// Creates a tree with a root node and the specified capacity.
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.push(Node::new(root));
        Tree { vec }
    }

    fn get_node(&self, id: Id) -> &Node<T> {
        unsafe { self.vec.get_unchecked(id.0) }
    }

    fn get_node_mut(&mut self, id: Id) -> &mut Node<T> {
        unsafe { self.vec.get_unchecked_mut(id.0) }
    }

    fn node_ref(&self, id: Id) -> NodeRef<T> {
        NodeRef { id, tree: self, node: self.get_node(id) }
    }

    fn node_mut(&mut self, id: Id) -> NodeMut<T> {
        NodeMut { id, tree: self }
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> NodeRef<T> {
        self.node_ref(Id(0))
    }

    /// Returns a mutator of the root node.
    pub fn root_mut(&mut self) -> NodeMut<T> {
        self.node_mut(Id(0))
    }

    /// Creates an orphan node.
    pub fn orphan(&mut self, value: T) -> NodeMut<T> {
        let id = Id(self.vec.len());
        self.vec.push(Node::new(value));
        self.node_mut(id)
    }

    /// Returns a reference to the specified node.
    pub unsafe fn get(&self, id: NodeId<T>) -> NodeRef<T> {
        self.node_ref(id.id)
    }

    /// Returns a mutator of the specified node.
    pub unsafe fn get_mut(&mut self, id: NodeId<T>) -> NodeMut<T> {
        self.node_mut(id.id)
    }
}

impl<'a, T: 'a> NodeRef<'a, T> {
    /// Returns the value of this node.
    pub fn value(&self) -> &'a T {
        &self.node.value
    }

    /// Returns the tree containing this node.
    pub fn tree(&self) -> &'a Tree<T> {
        self.tree
    }

    /// Returns the ID of this node.
    pub fn id(&self) -> NodeId<T> {
        NodeId { id: self.id, marker: PhantomData }
    }

    /// Returns the parent of this node.
    pub fn parent(&self) -> Option<Self> {
        self.node.parent.map(|id| self.tree.node_ref(id))
    }

    /// Returns the previous sibling of this node.
    pub fn prev_sibling(&self) -> Option<Self> {
        self.node.prev_sibling.map(|id| self.tree.node_ref(id))
    }

    /// Returns the next sibling of this node.
    pub fn next_sibling(&self) -> Option<Self> {
        self.node.next_sibling.map(|id| self.tree.node_ref(id))
    }

    /// Returns the first child of this node.
    pub fn first_child(&self) -> Option<Self> {
        self.node.children.map(|(id, _)| self.tree.node_ref(id))
    }

    /// Returns the last child of this node.
    pub fn last_child(&self) -> Option<Self> {
        self.node.children.map(|(_, id)| self.tree.node_ref(id))
    }

    /// Returns true if this node has siblings.
    pub fn has_siblings(&self) -> bool {
        self.node.prev_sibling.is_some() || self.node.next_sibling.is_some()
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        self.node.children.is_some()
    }
}

impl<'a, T: 'a> NodeMut<'a, T> {
    /// Returns the value of this node.
    pub fn value(&mut self) -> &mut T {
        &mut self.tree.get_node_mut(self.id).value
    }

    /// Returns the tree containing this node.
    pub fn tree(&mut self) -> &mut Tree<T> {
        self.tree
    }

    /// Returns the ID of this node.
    pub fn id(&self) -> NodeId<T> {
        NodeId { id: self.id, marker: PhantomData }
    }

    /// Returns the parent of this node.
    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        let id = self.tree.get_node(self.id).parent;
        id.map(move |id| self.tree.node_mut(id))
    }

    /// Returns the previous sibling of this node.
    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        let id = self.tree.get_node(self.id).prev_sibling;
        id.map(move |id| self.tree.node_mut(id))
    }

    /// Returns the next sibling of this node.
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        let id = self.tree.get_node(self.id).next_sibling;
        id.map(move |id| self.tree.node_mut(id))
    }

    /// Returns the first child of this node.
    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        let ids = self.tree.get_node(self.id).children;
        ids.map(move |(id, _)| self.tree.node_mut(id))
    }

    /// Returns the last child of this node.
    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        let ids = self.tree.get_node(self.id).children;
        ids.map(move |(_, id)| self.tree.node_mut(id))
    }

    /// Returns true if this node has siblings.
    pub fn has_siblings(&self) -> bool {
        let node = self.tree.get_node(self.id);
        node.prev_sibling.is_some() || node.next_sibling.is_some()
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        self.tree.get_node(self.id).children.is_some()
    }

    /// Appends a new child to this node.
    pub fn append(&mut self, value: T) -> NodeMut<T> {
        let id = self.tree.orphan(value).id();
        unsafe { self.append_id(id) }
    }

    /// Prepends a new child to this node.
    pub fn prepend(&mut self, value: T) -> NodeMut<T> {
        let id = self.tree.orphan(value).id();
        unsafe { self.prepend_id(id) }
    }

    /// Inserts a new sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_before(&mut self, value: T) -> NodeMut<T> {
        let id = self.tree.orphan(value).id();
        unsafe { self.insert_id_before(id) }
    }

    /// Inserts a new sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_after(&mut self, value: T) -> NodeMut<T> {
        let id = self.tree.orphan(value).id();
        unsafe { self.insert_id_after(id) }
    }

    /// Detaches this node from its parent.
    pub fn detach(&mut self) {
        let (parent_id, prev_sibling_id, next_sibling_id) = {
            let node = self.tree.get_node(self.id);
            let parent = match node.parent { Some(id) => id, None => return };
            (parent, node.prev_sibling, node.next_sibling)
        };

        {
            let node = self.tree.get_node_mut(self.id);
            node.parent = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }

        if let Some(id) = prev_sibling_id {
            self.tree.get_node_mut(id).next_sibling = next_sibling_id;
        }
        if let Some(id) = next_sibling_id {
            self.tree.get_node_mut(id).prev_sibling = prev_sibling_id;
        }

        let parent = self.tree.get_node_mut(parent_id);
        let (first_child_id, last_child_id) = parent.children.unwrap();
        if first_child_id == last_child_id {
            parent.children = None;
        } else if first_child_id == self.id {
            parent.children = Some((next_sibling_id.unwrap(), last_child_id));
        } else if last_child_id == self.id {
            parent.children = Some((first_child_id, prev_sibling_id.unwrap()));
        }
    }

    /// Appends a child to this node.
    pub unsafe fn append_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let new_child_id = id.id;
        let last_child_id = self.tree.get_node(self.id).children.map(|(_, id)| id);

        {
            let new_child = self.tree.get_node_mut(new_child_id);
            new_child.parent = Some(self.id);
            new_child.prev_sibling = last_child_id;
        }

        if let Some(id) = last_child_id {
            self.tree.get_node_mut(id).next_sibling = Some(new_child_id);
        }

        {
            let node = self.tree.get_node_mut(self.id);
            if let Some((first_child_id, _)) = node.children {
                node.children = Some((first_child_id, new_child_id));
            } else {
                node.children = Some((new_child_id, new_child_id));
            }
        }

        self.tree.node_mut(new_child_id)
    }

    /// Prepends a child to this node.
    pub unsafe fn prepend_id(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let new_child_id = id.id;
        let first_child_id = self.tree.get_node(self.id).children.map(|(id, _)| id);

        {
            let new_child = self.tree.get_node_mut(new_child_id);
            new_child.parent = Some(self.id);
            new_child.next_sibling = first_child_id;
        }

        if let Some(id) = first_child_id {
            self.tree.get_node_mut(id).prev_sibling = Some(new_child_id);
        }

        {
            let node = self.tree.get_node_mut(self.id);
            if let Some((_, last_child_id)) = node.children {
                node.children = Some((new_child_id, last_child_id));
            } else {
                node.children = Some((new_child_id, new_child_id));
            }
        }

        self.tree.node_mut(new_child_id)
    }

    /// Inserts a sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub unsafe fn insert_id_before(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let new_sibling_id = id.id;
        let parent_id = self.tree.get_node(self.id).parent.unwrap();
        let prev_sibling_id = self.tree.get_node(self.id).prev_sibling;

        {
            let new_sibling = self.tree.get_node_mut(new_sibling_id);
            new_sibling.parent = Some(parent_id);
            new_sibling.prev_sibling = prev_sibling_id;
            new_sibling.next_sibling = Some(self.id);
        }

        if let Some(id) = prev_sibling_id {
            self.tree.get_node_mut(id).next_sibling = Some(new_sibling_id);
        }

        self.tree.get_node_mut(self.id).prev_sibling = Some(new_sibling_id);

        {
            let parent = self.tree.get_node_mut(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if first_child_id == self.id {
                parent.children = Some((new_sibling_id, last_child_id));
            }
        }

        self.tree.node_mut(new_sibling_id)
    }

    /// Inserts a sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub unsafe fn insert_id_after(&mut self, id: NodeId<T>) -> NodeMut<T> {
        let new_sibling_id = id.id;
        let parent_id = self.tree.get_node(self.id).parent.unwrap();
        let next_sibling_id = self.tree.get_node(self.id).next_sibling;

        {
            let new_sibling = self.tree.get_node_mut(new_sibling_id);
            new_sibling.parent = Some(parent_id);
            new_sibling.prev_sibling = Some(self.id);
            new_sibling.next_sibling = next_sibling_id;
        }

        if let Some(id) = next_sibling_id {
            self.tree.get_node_mut(id).prev_sibling = Some(new_sibling_id);
        }

        self.tree.get_node_mut(self.id).next_sibling = Some(new_sibling_id);

        {
            let parent = self.tree.get_node_mut(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if last_child_id == self.id {
                parent.children = Some((first_child_id, new_sibling_id));
            }
        }

        self.tree.node_mut(new_sibling_id)
    }

    /// Reparents the children of a node, appending them to this node.
    pub unsafe fn reparent_from_id_append(&mut self, id: NodeId<T>) {
        let new_child_ids = match self.tree.get_node_mut(id.id).children.take() {
            Some(ids) => ids,
            None => return,
        };

        self.tree.get_node_mut(new_child_ids.0).parent = Some(self.id);
        self.tree.get_node_mut(new_child_ids.1).parent = Some(self.id);

        if self.tree.get_node(self.id).children.is_none() {
            self.tree.get_node_mut(self.id).children = Some(new_child_ids);
            return;
        }

        let old_child_ids = self.tree.get_node(self.id).children.unwrap();
        self.tree.get_node_mut(old_child_ids.1).next_sibling = Some(new_child_ids.0);
        self.tree.get_node_mut(new_child_ids.0).prev_sibling = Some(old_child_ids.1);

        self.tree.get_node_mut(self.id).children =
            Some((old_child_ids.0, new_child_ids.1));
    }

    /// Reparents the children of a node, prepending them to this node.
    pub unsafe fn reparent_from_id_prepend(&mut self, id: NodeId<T>) {
        let new_child_ids = match self.tree.get_node_mut(id.id).children.take() {
            Some(ids) => ids,
            None => return,
        };

        self.tree.get_node_mut(new_child_ids.0).parent = Some(self.id);
        self.tree.get_node_mut(new_child_ids.1).parent = Some(self.id);

        if self.tree.get_node(self.id).children.is_none() {
            self.tree.get_node_mut(self.id).children = Some(new_child_ids);
            return;
        }

        let old_child_ids = self.tree.get_node(self.id).children.unwrap();
        self.tree.get_node_mut(old_child_ids.0).prev_sibling = Some(new_child_ids.1);
        self.tree.get_node_mut(new_child_ids.1).next_sibling = Some(old_child_ids.0);

        self.tree.get_node_mut(self.id).children =
            Some((new_child_ids.0, old_child_ids.1))
    }
}

impl<'a, T: 'a> From<NodeMut<'a, T>> for NodeRef<'a, T> {
    fn from(node: NodeMut<'a, T>) -> Self {
        node.tree.node_ref(node.id)
    }
}

/// Iterators.
pub mod iter;

/// Creates a tree from expressions.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate ego_tree;
/// # fn main() {
/// let tree = tree!("root");
/// # }
/// ```
///
/// ```
/// #[macro_use] extern crate ego_tree;
/// # fn main() {
/// let tree = tree! {
///     "root" => {
///         "child a",
///         "child b" => {
///             "grandchild a",
///             "grandchild b",
///         },
///         "child c",
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! tree {
    (@ $n:ident { }) => { };

    // Last leaf.
    (@ $n:ident { $value:expr }) => {
        { $n.append($value); }
    };

    // Leaf.
    (@ $n:ident { $value:expr, $($tail:tt)* }) => {
        {
            $n.append($value);
            tree!(@ $n { $($tail)* });
        }
    };

    // Last node with children.
    (@ $n:ident { $value:expr => $children:tt }) => {
        {
            let mut node = $n.append($value);
            tree!(@ node $children);
        }
    };

    // Node with children.
    (@ $n:ident { $value:expr => $children:tt, $($tail:tt)* }) => {
        {
            {
                let mut node = $n.append($value);
                tree!(@ node $children);
            }
            tree!(@ $n { $($tail)* });
        }
    };

    ($root:expr) => { $crate::Tree::new($root) };

    ($root:expr => $children:tt) => {
        {
            let mut tree = $crate::Tree::new($root);
            {
                let mut node = tree.root_mut();
                tree!(@ node $children);
            }
            tree
        }
    };
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use iter::Edge;
        if f.alternate() {
            write!(f, "Tree {{")?;
            for edge in self.root().traverse() {
                match edge {
                    Edge::Open(node) if node.has_children() => {
                        write!(f, " {:?} => {{", node.value())?;
                    },
                    Edge::Open(node) if node.next_sibling().is_some() => {
                        write!(f, " {:?},", node.value())?;
                    },
                    Edge::Open(node) => {
                        write!(f, " {:?}", node.value())?;
                    },
                    Edge::Close(node) if node.has_children() => {
                        if node.next_sibling().is_some() {
                            write!(f, " }},")?;
                        } else {
                            write!(f, " }}")?;
                        }
                    },
                    _ => {},
                }
            }
            write!(f, " }}")
        } else {
            f.debug_struct("Tree").field("vec", &self.vec).finish()
        }
    }
}
