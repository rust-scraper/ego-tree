#![cfg(feature = "serde")]

use std::assert_eq;

use ego_tree::{tree, Tree};

#[test]
fn test_serialize() {
    let tree = tree!("r" => {"a", "b" => { "d", "e" }, "c"});

    let serialized = serde_json::to_string(&tree).unwrap();
    println!("{serialized}");
    let deserialized: Tree<&str> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(tree, deserialized);
}
