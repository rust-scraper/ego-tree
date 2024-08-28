#![cfg(feature = "serde")]
#[macro_use]
extern crate ego_tree;
use std::assert_eq;

use ego_tree::Tree;

#[test]
fn test_serialize() {
    let tree = tree!("r" => {"a", "b" => { "d", "e" }, "c"});

    let serialized = serde_json::to_string(&tree).unwrap();
    let deserialized: Tree<&str> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(tree, deserialized);
}
