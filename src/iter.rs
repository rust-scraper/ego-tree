//! Tree iterators.

use super::NodeRef;

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
