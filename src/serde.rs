use serde::{ser::Serialize, Deserialize};

use crate::{NodeId, NodeRef, Tree};

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

#[derive(Debug)]
struct DeserNode<T> {
    value: T,
    children: Vec<DeserNode<T>>,
}

impl<T> DeserNode<T> {
    fn to_tree_node(self, tree: &mut Tree<T>, parent: NodeId) -> NodeId {
        let mut parent = tree.get_mut(parent).unwrap();
        let node = parent.append(self.value).id();

        for child in self.children {
            child.to_tree_node(tree, node);
        }

        node
    }
}

impl<T> From<DeserNode<T>> for Tree<T> {
    fn from(root: DeserNode<T>) -> Self {
        let mut tree: Tree<T> = Tree::new(root.value);
        let root_id = tree.root().id;

        for child in root.children {
            child.to_tree_node(&mut tree, root_id);
        }

        tree
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for DeserNode<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (value, children) = <(T, Vec<DeserNode<T>>)>::deserialize(deserializer)?;
        Ok(DeserNode { value, children })
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Tree<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deser = DeserNode::<T>::deserialize(deserializer)?;
        Ok(deser.into())
    }
}
