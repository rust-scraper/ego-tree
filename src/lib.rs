//! Vec-backed mutable tree.
//!
//! # Behaviour
//!
//! - Nodes have zero or more ordered children.
//! - Nodes have at most one parent, i.e. orphan nodes are valid.
//! - Individual nodes are not dropped until the tree is dropped.
//! - A node's parent, next sibling, previous sibling, first child and last child can be accessed
//!   in constant time.
//! - Node IDs act as weak references, i.e. they are not tied to the lifetime of the tree.
//!
//! # Examples
//!
//! ```
//! // TODO
//! ```

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

/// A Vec-backed tree.
///
/// Nodes are allocated in a `Vec` which is only ever pushed to. `NodeId` is an opaque index into
/// the `Vec`.
///
/// Each `Tree` has a unique ID which is also given to each `NodeId` it creates. This is used to
/// bounds check a `NodeId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    id: usize,
    vec: Vec<Node<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<T> {
    parent: Option<NodeId<T>>,
    prev_sibling: Option<NodeId<T>>,
    next_sibling: Option<NodeId<T>>,
    children: Option<(NodeId<T>, NodeId<T>)>,
    value: T,
}

/// A node ID.
///
/// `NodeId` acts as a weak reference which is not tied to the lifetime of the `Tree` that created
/// it.
///
/// With the original `Tree`, a `NodeId` can be used to obtain a `NodeRef` or `NodeMut`.
#[derive(Debug)]
pub struct NodeId<T> {
    tree_id: usize,
    index: usize,
    data: PhantomData<T>,
}

/// A node reference.
#[derive(Debug)]
pub struct NodeRef<'a, T: 'a> {
    tree: &'a Tree<T>,
    node: &'a Node<T>,
    id: NodeId<T>,
}

/// A node mutator.
#[derive(Debug)]
pub struct NodeMut<'a, T: 'a> {
    tree: &'a mut Tree<T>,
    id: NodeId<T>,
}

// Implementations.
mod node_id;
mod node_ref;
mod node_mut;

pub mod iter;

// Used to ensure that an Id can only be used with the same Tree that created it.
static TREE_ID_SEQ: AtomicUsize = ATOMIC_USIZE_INIT;

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

impl<T> Tree<T> {
    /// Creates a new tree with a root node.
    pub fn new(root: T) -> Self {
        Tree {
            id: TREE_ID_SEQ.fetch_add(1, Ordering::Relaxed),
            vec: vec![Node::new(root)],
        }
    }

    /// Creates a new tree of the specified capacity with a root node.
    pub fn with_capacity(root: T, capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.push(Node::new(root));
        Tree {
            id: TREE_ID_SEQ.fetch_add(1, Ordering::Relaxed),
            vec: vec,
        }
    }

    /// Returns a reference to the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub fn get(&self, id: NodeId<T>) -> NodeRef<T> {
        assert_eq!(self.id, id.tree_id);
        self.get_unchecked(id)
    }

    /// Returns a mutator for the specified node.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not refer to a node in this tree.
    pub fn get_mut(&mut self, id: NodeId<T>) -> NodeMut<T> {
        assert_eq!(self.id, id.tree_id);
        self.get_unchecked_mut(id)
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> NodeRef<T> {
        self.get_unchecked(self.node_id(0))
    }

    /// Returns a mutable reference to the root node.
    pub fn root_mut(&mut self) -> NodeMut<T> {
        let id = self.node_id(0);
        self.get_unchecked_mut(id)
    }

    /// Creates an orphan node.
    pub fn orphan(&mut self, value: T) -> NodeMut<T> {
        let id = self.node_id(self.vec.len());
        self.vec.push(Node::new(value));
        self.get_unchecked_mut(id)
    }

    fn node_id(&self, index: usize) -> NodeId<T> {
        NodeId {
            tree_id: self.id,
            index: index,
            data: PhantomData,
        }
    }

    fn get_unchecked(&self, id: NodeId<T>) -> NodeRef<T> {
        NodeRef {
            tree: self,
            node: self.get_node_unchecked(id),
            id: id,
        }
    }

    fn get_unchecked_mut(&mut self, id: NodeId<T>) -> NodeMut<T> {
        NodeMut {
            tree: self,
            id: id,
        }
    }

    fn get_node_unchecked(&self, id: NodeId<T>) -> &Node<T> {
        unsafe { self.vec.get_unchecked(id.index) }
    }

    fn get_node_unchecked_mut(&mut self, id: NodeId<T>) -> &mut Node<T> {
        unsafe { self.vec.get_unchecked_mut(id.index) }
    }
}

impl<T: Default> Default for Tree<T> {
    fn default() -> Self {
        Tree::new(T::default())
    }
}
