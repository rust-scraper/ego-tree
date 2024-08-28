#![cfg(feature = "serde")]
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::{Node, NodeId, Tree};

impl<T: Serialize> Serialize for Tree<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.vec.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Tree<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::deserialize(deserializer)?;
        Ok(Tree { vec })
    }
}

impl<T: Serialize> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            &self.parent,
            &self.prev_sibling,
            &self.next_sibling,
            &self.children,
            &self.value,
        )
            .serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Node<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (parent, prev_sibling, next_sibling, children, value) =
            <(
                Option<NodeId>,
                Option<NodeId>,
                Option<NodeId>,
                Option<(NodeId, NodeId)>,
                T,
            )>::deserialize(deserializer)?;
        Ok(Node {
            parent,
            prev_sibling,
            next_sibling,
            children,
            value,
        })
    }
}

impl Serialize for NodeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = <NonZeroUsize>::deserialize(deserializer)?;
        Ok(NodeId(index))
    }
}
