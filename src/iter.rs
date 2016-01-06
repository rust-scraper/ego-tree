//! Tree iterators.

use std::{fmt, slice, vec};

use super::{Tree, Node, NodeRef, NodeMut};

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
}

/// Iterator over node ancestors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ancestors<'a, T: 'a> {
    node: Option<NodeRef<'a, T>>,
}

impl<'a, T: 'a> Iterator for Ancestors<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        let node = self.node.take();
        self.node = node.as_ref().and_then(NodeRef::parent);
        node
    }
}

impl<'a, T: 'a> NodeRef<'a, T> {
    /// Returns an iterator over this node and its ancestors.
    pub fn ancestors(&self) -> Ancestors<T> {
        Ancestors { node: Some(self.clone()) }
    }
}
