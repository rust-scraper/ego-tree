//! # ID Tree
//!
//! A tree in which nodes are stored in a `Vec` and links are represented as IDs, which are indexes
//! into the `Vec`.
//!
//! Exhibits these properties:
//!
//! - Each node has zero or more ordered children.
//! - Each node has at most one parent.
//! - Access to a node's parent, next sibling, previous sibling, first child and last child occurs
//!   in O(1) time.
//! - Orphan nodes (without a parent or siblings) are permitted to exist.
//! - IDs can be decoupled from the lifetime of the tree.
//! - Individual nodes cannot be dropped, only detached from the tree. All nodes are dropped when
//!   the tree is dropped.
//!
//! # Examples
//!
//! TODO

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

// Used to ensure that an Id can only be used with the same Tree that created it.
static mut tree_id_seq: AtomicUsize = ATOMIC_USIZE_INIT;

/// A tree.
// TODO: Implement Debug manually.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    id: usize,
    vec: Vec<Node<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<T> {
    parent: Option<Id<T>>,
    prev_sibling: Option<Id<T>>,
    next_sibling: Option<Id<T>>,
    children: Option<(Id<T>, Id<T>)>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            value: value,
        }
    }
}

/// A node ID.
#[derive(Debug)]
pub struct Id<T> {
    tree_id: usize,
    index: usize,
    data: PhantomData<T>,
}

// Manual implementations that don't care about T.
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            tree_id: self.tree_id,
            index: self.index,
            data: PhantomData,
        }
    }
}
impl<T> Copy for Id<T> { }
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_id == other.tree_id && self.index == other.index
    }
}
impl<T> Eq for Id<T> { }

/// A node reference.
#[derive(Debug)]
pub struct Ref<'a, T: 'a> {
    tree: &'a Tree<T>,
    node: &'a Node<T>,
    id: Id<T>,
}

// Manual implementations that don't care about T.
impl<'a, T: 'a> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        Ref {
            tree: self.tree,
            node: self.node,
            id: self.id
        }
    }
}
impl<'a, T: 'a> Copy for Ref<'a, T> { }
impl<'a, T: 'a> PartialEq for Ref<'a, T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.tree as *const _ == other.tree as *const _
            && self.node as *const _ == other.node as *const _
            && self.id == other.id
    }
}
impl<'a, T: 'a> Eq for Ref<'a, T> { }

/// A mutable node reference.
///
/// Note that accesses to the node may be slower than `Ref`, since `RefMut` cannot directly hold a
/// reference to the node.
#[derive(Debug)]
pub struct RefMut<'a, T: 'a> {
    tree: &'a mut Tree<T>,
    id: Id<T>,
}

// Manual implementations that don't care about T.
impl<'a, T: 'a> PartialEq for RefMut<'a, T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.tree as *const _ == other.tree as *const _
            && self.id == other.id
    }
}
impl<'a, T: 'a> Eq for RefMut<'a, T> { }

impl<T> Tree<T> {
    /// Creates a new tree with a root node.
    pub fn new(root: T) -> Self {
        // Safe: atomic.
        Tree {
            id: unsafe { tree_id_seq.fetch_add(1, Ordering::Relaxed) },
            vec: vec![Node::new(root)],
        }
    }

    /// Creates a new tree of the specified capacity with a root node.
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.push(Node::new(root));
        // Safe: atomic.
        Tree {
            id: unsafe { tree_id_seq.fetch_add(1, Ordering::Relaxed) },
            vec: vec,
        }
    }

    /// Returns a reference to the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub fn get(&self, id: Id<T>) -> Ref<T> {
        assert_eq!(self.id, id.tree_id);
        // Safe: We created the Id, so its index is valid.
        Ref {
            tree: self,
            node: unsafe { self.vec.get_unchecked(id.index) },
            id: id,
        }
    }

    /// Returns a mutable reference to the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub fn get_mut(&mut self, id: Id<T>) -> RefMut<T> {
        assert_eq!(self.id, id.tree_id);
        RefMut {
            tree: self,
            id: id,
        }
    }

    fn id_for(&self, index: usize) -> Id<T> {
        Id {
            tree_id: self.id,
            index: index,
            data: PhantomData,
        }
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> Ref<T> {
        self.get(self.id_for(0))
    }

    /// Returns a mutable reference to the root node.
    pub fn root_mut(&mut self) -> RefMut<T> {
        let id = self.id_for(0);
        self.get_mut(id)
    }

    /// Creates an orphan node.
    pub fn orphan(&mut self, value: T) -> RefMut<T> {
        let id = self.id_for(self.vec.len());
        self.vec.push(Node::new(value));
        self.get_mut(id)
    }
}

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
    pub fn parent(&self) -> Option<Ref<'a, T>> {
        self.node.parent.map(|id| self.tree.get(id))
    }

    /// Returns a reference to the previous sibling.
    pub fn prev_sibling(&self) -> Option<Ref<'a, T>> {
        self.node.prev_sibling.map(|id| self.tree.get(id))
    }

    /// Returns a reference to the next sibling.
    pub fn next_sibling(&self) -> Option<Ref<'a, T>> {
        self.node.next_sibling.map(|id| self.tree.get(id))
    }

    /// Returns a reference to the first child.
    pub fn first_child(&self) -> Option<Ref<'a, T>> {
        self.node.children.map(|(id, _)| self.tree.get(id))
    }

    /// Returns a reference to the last child.
    pub fn last_child(&self) -> Option<Ref<'a, T>> {
        self.node.children.map(|(_, id)| self.tree.get(id))
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

impl<'a, T: 'a> Into<Ref<'a, T>> for RefMut<'a, T> {
    fn into(self) -> Ref<'a, T> {
        // Safe: RefMut can only be created by Tree, which validates the Id.
        Ref {
            tree: self.tree,
            node: unsafe { self.tree.vec.get_unchecked(self.id.index) },
            id: self.id,
        }
    }
}

impl<'a, T: 'a> RefMut<'a, T> {
    fn node(&self) -> &Node<T> {
        // Safe: RefMut can only be created by Tree, which validates the Id.
        unsafe { self.tree.vec.get_unchecked(self.id.index) }
    }

    fn node_mut(&mut self) -> &mut Node<T> {
        // Safe: RefMut can only be created by Tree, which validates the Id.
        unsafe { self.tree.vec.get_unchecked_mut(self.id.index) }
    }

    /// Returns the value of the node.
    pub fn value(&mut self) -> &mut T {
        &mut self.node_mut().value
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> Id<T> {
        self.id
    }

    /// Returns a reference to the parent node.
    pub fn parent(self) -> Option<RefMut<'a, T>> {
        let id = self.node().parent;
        id.map(move |id| self.tree.get_mut(id))
    }

    /// Returns a reference to the previous sibling.
    pub fn prev_sibling(self) -> Option<RefMut<'a, T>> {
        let id = self.node().prev_sibling;
        id.map(move |id| self.tree.get_mut(id))
    }

    /// Returns a reference to the next sibling.
    pub fn next_sibling(self) -> Option<RefMut<'a, T>> {
        let id = self.node().next_sibling;
        id.map(move |id| self.tree.get_mut(id))
    }

    /// Returns a reference to the first child.
    pub fn first_child(self) -> Option<RefMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(id, _)| self.tree.get_mut(id))
    }

    /// Returns a reference to the last child.
    pub fn last_child(self) -> Option<RefMut<'a, T>> {
        let children = self.node().children;
        children.map(move |(_, id)| self.tree.get_mut(id))
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
    pub fn append(&mut self, value: T) -> Id<T> {
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
    pub fn append_node(&mut self, id: Id<T>) {
        assert_eq!(self.tree.id, id.tree_id);
        // Safe: Id created by same Tree.
        assert!(unsafe { self.tree.vec.get_unchecked(id.index).parent.is_none() });

        // TODO: Uncheck all this.

        let last_child = self.tree.vec[self.id.index].children.map(|t| t.1);

        // Update new child.
        {
            let node = &mut self.tree.vec[id.index];
            node.parent = Some(self.id);
            node.prev_sibling = last_child;
        }

        // Update previous last child.
        if let Some(child) = last_child {
            let node = &mut self.tree.vec[child.index];
            node.next_sibling = Some(id);
        }

        // Update parent.
        let node = &mut self.tree.vec[self.id.index];
        if let Some((first, _)) = node.children {
            node.children = Some((first, id));
        } else {
            node.children = Some((id, id));
        }
    }
}

pub mod iter;
