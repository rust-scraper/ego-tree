#![cfg(feature = "serde")]

use ego_tree::{tree, Tree};

#[test]
fn test_serialize() {
    let tree = tree!("r" => {"a", "b" => { "d", "e" }, "c"});

    let serialized = serde_json::to_string(&tree).unwrap();
    println!("{serialized}");
}
