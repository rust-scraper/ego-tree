//! Tree iterators.

use super::Ref;

/// Iterator over node ancestors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ancestors<'a, T: 'a> {
    node: Option<Ref<'a, T>>,
}

impl<'a, T: 'a> Iterator for Ancestors<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Ref<'a, T>> {
        let node = self.node.take();
        self.node = node.as_ref().and_then(Ref::parent);
        node
    }
}

impl<'a, T: 'a> Ref<'a, T> {
    /// Returns an iterator over this node and its ancestors.
    pub fn ancestors(&self) -> Ancestors<T> {
        Ancestors { node: Some(self.clone()) }
    }
}
