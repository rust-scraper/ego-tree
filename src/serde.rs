use serde::ser::{Serialize, SerializeStruct};

use crate::{NodeRef, Tree};

#[derive(Debug)]
struct SerNode<'a, T> {
    value: &'a T,
    children: Vec<SerNode<'a, T>>,
}

impl<'a, T> From<NodeRef<'a, T>> for SerNode<'a, T> {
    fn from(node: NodeRef<'a, T>) -> Self {
        let value: &T = node.value();
        let children: Vec<SerNode<'a, T>> = node.children().map(SerNode::<'a, T>::from).collect();
        SerNode { value, children }
    }
}

impl<'a, T: Serialize> Serialize for SerNode<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.value, &self.children).serialize(serializer)
    }
}

impl<T: Serialize> Serialize for Tree<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerNode::from(self.root()).serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_ser_node_from() {
        let tree = tree!("a" => {"b", "c" => {"d", "e"}, "f"});
        let repr = serde_json::to_string(&SerNode::from(tree.root())).unwrap();
        println!("{repr}");
    }
}
