//! Tree iterators.

use std::{fmt, iter, slice, vec};

use super::{Tree, Node, NodeRef};

/// Iterator over node values.
#[derive(Clone)]
pub struct Values<'a, T: 'a> {
    inner: slice::Iter<'a, Node<T>>,
}

impl<'a, T: 'a> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next().map(|n| &n.value)
    }
}

impl<'a, T: 'a> fmt::Debug for Values<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Values").finish()
    }
}

/// Mutable iterator over node values.
pub struct ValuesMut<'a, T: 'a> {
    inner: slice::IterMut<'a, Node<T>>,
}

impl<'a, T: 'a> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.next().map(|n| &mut n.value)
    }
}

impl<'a, T: 'a> fmt::Debug for ValuesMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("ValuesMut").finish()
    }
}

/// Iterator that moves node values out of a tree.
pub struct IntoValues<T> {
    inner: vec::IntoIter<Node<T>>,
}

impl<T> Iterator for IntoValues<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.next().map(|n| n.value)
    }
}

impl<T> fmt::Debug for IntoValues<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("IntoValues").finish()
    }
}

/// Iterator over all nodes.
pub struct Nodes<'a, T: 'a> {
    tree: &'a Tree<T>,
    inner: iter::Enumerate<slice::Iter<'a, Node<T>>>,
}

impl<'a, T: 'a> Iterator for Nodes<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        self.inner.next().map(|(index, node)| {
            NodeRef {
                tree: self.tree,
                node: node,
                index: index,
            }
        })
    }
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Nodes<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Nodes")
            .field("tree", &self.tree)
            .finish()
    }
}

impl<T> Tree<T> {
    /// Returns an iterator over node values in creation order.
    pub fn values(&self) -> Values<T> {
        Values { inner: self.vec.iter() }
    }

    /// Returns a mutable iterator over node values in creation order.
    pub fn values_mut(&mut self) -> ValuesMut<T> {
        ValuesMut { inner: self.vec.iter_mut() }
    }

    /// Returns an iterator that moves node values out of the tree in creation order.
    pub fn into_values(self) -> IntoValues<T> {
        IntoValues { inner: self.vec.into_iter() }
    }

    /// Returns an iterator over all nodes, including orphans, in creation order.
    pub fn nodes(&self) -> Nodes<T> {
        Nodes {
            tree: self,
            inner: self.vec.iter().enumerate(),
        }
    }
}

macro_rules! axis_iterators {
    ($(#[$m:meta] $i:ident($f:path);)*) => {
        $(
            #[$m]
            #[derive(Debug)]
            pub struct $i<'a, T: 'a> {
                node: Option<NodeRef<'a, T>>,
            }

            impl<'a, T: 'a> Iterator for $i<'a, T> {
                type Item = NodeRef<'a, T>;

                fn next(&mut self) -> Option<NodeRef<'a, T>> {
                    let node = self.node.take();
                    self.node = node.as_ref().and_then($f);
                    node
                }
            }

            impl<'a, T: 'a> Copy for $i<'a, T> { }
            impl<'a, T: 'a> Clone for $i<'a, T> {
                fn clone(&self) -> Self { $i { node: self.node } }
            }

            impl<'a, T: 'a> Eq for $i<'a, T> { }
            impl<'a, T: 'a> PartialEq for $i<'a, T> {
                fn eq(&self, other: &Self) -> bool { self.node == other.node }
            }
        )*
    }
}

axis_iterators! {
    #[doc = "Iterator over node ancestors."]
    Ancestors(NodeRef::parent);

    #[doc = "Iterator over node previous siblings."]
    PrevSiblings(NodeRef::prev_sibling);

    #[doc = "Iterator over node next siblings."]
    NextSiblings(NodeRef::next_sibling);

    #[doc = "Iterator over node first children."]
    FirstChildren(NodeRef::first_child);

    #[doc = "Iterator over node last children."]
    LastChildren(NodeRef::last_child);
}

/// Iterator over node children.
#[derive(Debug)]
pub struct Children<'a, T: 'a> {
    front: Option<NodeRef<'a, T>>,
    back: Option<NodeRef<'a, T>>,
}

impl<'a, T: 'a> Iterator for Children<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(NodeRef::next_sibling);
            node
        }
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Children<'a, T> {
    fn next_back(&mut self) -> Option<NodeRef<'a, T>> {
        if self.front == self.back {
            let node = self.back.take();
            self.front = None;
            node
        } else {
            let node = self.back.take();
            self.back = node.as_ref().and_then(NodeRef::prev_sibling);
            node
        }
    }
}

impl<'a, T: 'a> Copy for Children<'a, T> { }
impl<'a, T: 'a> Clone for Children<'a, T> {
    fn clone(&self) -> Self {
        Children {
            front: self.front,
            back: self.back,
        }
    }
}

impl<'a, T: 'a> Eq for Children<'a, T> { }
impl<'a, T: 'a> PartialEq for Children<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.front == other.front && self.back == other.back
    }
}

/// An open or close edge of a node.
#[derive(Debug)]
pub enum Edge<'a, T: 'a> {
    /// Open.
    Open(NodeRef<'a, T>),

    /// Close.
    Close(NodeRef<'a, T>),
}

impl<'a, T: 'a> Copy for Edge<'a, T> { }
impl<'a, T: 'a> Clone for Edge<'a, T> {
    fn clone(&self) -> Self {
        match *self {
            Edge::Open(node) => Edge::Open(node),
            Edge::Close(node) => Edge::Close(node),
        }
    }
}

impl<'a, T: 'a> Eq for Edge<'a, T> { }
impl<'a, T: 'a> PartialEq for Edge<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Edge::Open(a), Edge::Open(b)) => a == b,
            (Edge::Close(a), Edge::Close(b)) => a == b,
            _ => false,
        }
    }
}

/// Iterator which traverses a subtree.
#[derive(Debug)]
pub struct Traverse<'a, T: 'a> {
    root: NodeRef<'a, T>,
    edge: Option<Edge<'a, T>>,
}

impl<'a, T: 'a> Iterator for Traverse<'a, T> {
    type Item = Edge<'a, T>;

    fn next(&mut self) -> Option<Edge<'a, T>> {
        match self.edge {
            None => {
                self.edge = Some(Edge::Open(self.root));
            },
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node));
                }
            },
            Some(Edge::Close(node)) => {
                if node == self.root {
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            },
        }
        self.edge
    }
}

impl<'a, T: 'a> Copy for Traverse<'a, T> { }
impl<'a, T: 'a> Clone for Traverse<'a, T> {
    fn clone(&self) -> Self {
        Traverse {
            root: self.root,
            edge: self.edge,
        }
    }
}

impl<'a, T: 'a> Eq for Traverse<'a, T> { }
impl<'a, T: 'a> PartialEq for Traverse<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.edge == other.edge
    }
}

impl<'a, T: 'a> NodeRef<'a, T> {
    /// Returns an iterator over this node's ancestors.
    pub fn ancestors(&self) -> Ancestors<'a, T> {
        Ancestors { node: self.parent() }
    }

    /// Returns an iterator over this node's previous siblings.
    pub fn prev_siblings(&self) -> PrevSiblings<'a, T> {
        PrevSiblings { node: self.prev_sibling() }
    }

    /// Returns an iterator over this node's next siblings.
    pub fn next_siblings(&self) -> NextSiblings<'a, T> {
        NextSiblings { node: self.next_sibling() }
    }

    /// Returns an iterator over this node's children.
    pub fn children(&self) -> Children<'a, T> {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator over this node's first children.
    pub fn first_children(&self) -> FirstChildren<'a, T> {
        FirstChildren { node: self.first_child() }
    }

    /// Returns an iterator over this node's last children.
    pub fn last_children(&self) -> LastChildren<'a, T> {
        LastChildren { node: self.last_child() }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    pub fn traverse(&self) -> Traverse<'a, T> {
        Traverse {
            root: *self,
            edge: None,
        }
    }
}
