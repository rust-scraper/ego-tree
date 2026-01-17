use std::iter::FusedIterator;
use std::ops::Range;
use std::{slice, vec};

use crate::{ID, Node, NodeId, NodeRef, Tree};

/// Iterator that moves out of a tree in insert order.
#[derive(Debug)]
pub struct IntoIter<T, I: ID = NodeId>(vec::IntoIter<Node<T, I>>);
impl<T, I: ID> ExactSizeIterator for IntoIter<T, I> {}
impl<T, I: ID> FusedIterator for IntoIter<T, I> {}
impl<T, I: ID> Iterator for IntoIter<T, I> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<T, I: ID> DoubleEndedIterator for IntoIter<T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| node.value)
    }
}

/// Iterator over values in insert order.
#[derive(Debug)]
pub struct Values<'a, T: 'a, I: ID = NodeId>(slice::Iter<'a, Node<T, I>>);
impl<'a, T: 'a, I: ID> Clone for Values<'a, T, I> {
    fn clone(&self) -> Self {
        Values(self.0.clone())
    }
}
impl<'a, T: 'a, I: ID> ExactSizeIterator for Values<'a, T, I> {}
impl<'a, T: 'a, I: ID> FusedIterator for Values<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for Values<'a, T, I> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| &node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T: 'a, I: ID> DoubleEndedIterator for Values<'a, T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| &node.value)
    }
}

/// Mutable iterator over values in insert order.
#[derive(Debug)]
pub struct ValuesMut<'a, T: 'a, I: ID = NodeId>(slice::IterMut<'a, Node<T, I>>);
impl<'a, T: 'a, I: ID> ExactSizeIterator for ValuesMut<'a, T, I> {}
impl<'a, T: 'a, I: ID> FusedIterator for ValuesMut<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for ValuesMut<'a, T, I> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| &mut node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T: 'a, I: ID> DoubleEndedIterator for ValuesMut<'a, T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| &mut node.value)
    }
}

/// Iterator over nodes in insert order.
#[derive(Debug)]
pub struct Nodes<'a, T: 'a, I: ID = NodeId> {
    tree: &'a Tree<T, I>,
    iter: Range<usize>,
}
impl<'a, T: 'a, I: ID> Clone for Nodes<'a, T, I> {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree,
            iter: self.iter.clone(),
        }
    }
}
impl<'a, T: 'a, I: ID> ExactSizeIterator for Nodes<'a, T, I> {}
impl<'a, T: 'a, I: ID> FusedIterator for Nodes<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for Nodes<'a, T, I> {
    type Item = NodeRef<'a, T, I>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| unsafe { self.tree.get_unchecked(I::from_index(i)) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, T: 'a, I: ID> DoubleEndedIterator for Nodes<'a, T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|i| unsafe { self.tree.get_unchecked(I::from_index(i)) })
    }
}

impl<T, I: ID> IntoIterator for Tree<T, I> {
    type Item = T;
    type IntoIter = IntoIter<T, I>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.vec.into_iter())
    }
}

impl<T, I: ID> Tree<T, I> {
    /// Returns an iterator over values in insert order.
    pub fn values(&self) -> Values<'_, T, I> {
        Values(self.vec.iter())
    }

    /// Returns a mutable iterator over values in insert order.
    pub fn values_mut(&mut self) -> ValuesMut<'_, T, I> {
        ValuesMut(self.vec.iter_mut())
    }

    /// Returns an iterator over nodes in insert order.
    pub fn nodes(&self) -> Nodes<'_, T, I> {
        Nodes {
            tree: self,
            iter: 0..self.vec.len(),
        }
    }
}

macro_rules! axis_iterators {
    ($(#[$m:meta] $i:ident($f:path);)*) => {
        $(
            #[$m]
            #[derive(Debug)]
            pub struct $i<'a, T: 'a, I: ID = NodeId>(Option<NodeRef<'a, T, I>>);
            impl<'a, T: 'a, I: ID> Clone for $i<'a, T, I> {
                fn clone(&self) -> Self {
                    $i(self.0)
                }
            }
            impl<'a, T: 'a, I: ID> FusedIterator for $i<'a, T, I> {}
            impl<'a, T: 'a, I: ID> Iterator for $i<'a, T, I> {
                type Item = NodeRef<'a, T, I>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then($f);
                    node
                }
            }
        )*
    };
}

axis_iterators! {
    /// Iterator over ancestors.
    Ancestors(NodeRef::parent);

    /// Iterator over previous siblings.
    PrevSiblings(NodeRef::prev_sibling);

    /// Iterator over next siblings.
    NextSiblings(NodeRef::next_sibling);

    /// Iterator over first children.
    FirstChildren(NodeRef::first_child);

    /// Iterator over last children.
    LastChildren(NodeRef::last_child);
}

/// Iterator over children.
#[derive(Debug)]
pub struct Children<'a, T: 'a, I: ID = NodeId> {
    front: Option<NodeRef<'a, T, I>>,
    back: Option<NodeRef<'a, T, I>>,
}
impl<'a, T: 'a, I: ID> Clone for Children<'a, T, I> {
    fn clone(&self) -> Self {
        Self {
            front: self.front,
            back: self.back,
        }
    }
}
impl<'a, T: 'a, I: ID> FusedIterator for Children<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for Children<'a, T, I> {
    type Item = NodeRef<'a, T, I>;
    fn next(&mut self) -> Option<Self::Item> {
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
impl<'a, T: 'a, I: ID> DoubleEndedIterator for Children<'a, T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
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

/// Open or close edge of a node.
#[derive(Debug)]
pub enum Edge<'a, T: 'a, I: ID> {
    /// Open.
    Open(NodeRef<'a, T, I>),
    /// Close.
    Close(NodeRef<'a, T, I>),
}
impl<'a, T: 'a, I: ID> Copy for Edge<'a, T, I> {}
impl<'a, T: 'a, I: ID> Clone for Edge<'a, T, I> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T: 'a, I: ID> Eq for Edge<'a, T, I> {}
impl<'a, T: 'a, I: ID> PartialEq for Edge<'a, T, I> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Edge::Open(a), Edge::Open(b)) | (Edge::Close(a), Edge::Close(b)) => a == b,
            _ => false,
        }
    }
}

/// Iterator which traverses a subtree.
#[derive(Debug)]
pub struct Traverse<'a, T: 'a, I: ID = NodeId> {
    root: Option<NodeRef<'a, T, I>>,
    edge: Option<Edge<'a, T, I>>,
}
impl<'a, T: 'a, I: ID> Clone for Traverse<'a, T, I> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            edge: self.edge,
        }
    }
}
impl<'a, T: 'a, I: ID> FusedIterator for Traverse<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for Traverse<'a, T, I> {
    type Item = Edge<'a, T, I>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            None => {
                if let Some(root) = self.root {
                    self.edge = Some(Edge::Open(root));
                }
            }
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node));
                }
            }
            Some(Edge::Close(node)) => {
                if node == self.root.unwrap() {
                    self.root = None;
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
        }
        self.edge
    }
}

/// Iterator over a node and its descendants.
#[derive(Debug)]
pub struct Descendants<'a, T: 'a, I: ID = NodeId>(Traverse<'a, T, I>);
impl<'a, T: 'a, I: ID> Clone for Descendants<'a, T, I> {
    fn clone(&self) -> Self {
        Descendants(self.0.clone())
    }
}
impl<'a, T: 'a, I: ID> FusedIterator for Descendants<'a, T, I> {}
impl<'a, T: 'a, I: ID> Iterator for Descendants<'a, T, I> {
    type Item = NodeRef<'a, T, I>;
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }
        None
    }
}

impl<'a, T: 'a, I: ID> NodeRef<'a, T, I> {
    /// Returns an iterator over ancestors.
    pub fn ancestors(&self) -> Ancestors<'a, T, I> {
        Ancestors(self.parent())
    }

    /// Returns an iterator over previous siblings.
    pub fn prev_siblings(&self) -> PrevSiblings<'a, T, I> {
        PrevSiblings(self.prev_sibling())
    }

    /// Returns an iterator over next siblings.
    pub fn next_siblings(&self) -> NextSiblings<'a, T, I> {
        NextSiblings(self.next_sibling())
    }

    /// Returns an iterator over first children.
    pub fn first_children(&self) -> FirstChildren<'a, T, I> {
        FirstChildren(self.first_child())
    }

    /// Returns an iterator over last children.
    pub fn last_children(&self) -> LastChildren<'a, T, I> {
        LastChildren(self.last_child())
    }

    /// Returns an iterator over children.
    pub fn children(&self) -> Children<'a, T, I> {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    pub fn traverse(&self) -> Traverse<'a, T, I> {
        Traverse {
            root: Some(*self),
            edge: None,
        }
    }

    /// Returns an iterator over this node and its descendants.
    pub fn descendants(&self) -> Descendants<'a, T, I> {
        Descendants(self.traverse())
    }
}
