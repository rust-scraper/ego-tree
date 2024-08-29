#![cfg(feature = "serde")]

use ego_tree::{tree, Tree};

#[test]
fn test_serde_round_trip() {
    let tree = tree!("a" => {"b", "c" => {"d", "e"}, "f"});
    let repr = serde_json::to_string(&tree).unwrap();
    println!("{repr}");
    let re_tree: Tree<&str> = serde_json::from_str(&repr).unwrap();
    println!("{re_tree}");
    assert_eq!(tree, re_tree);
}
