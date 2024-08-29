#![cfg(feature = "serde")]

use ego_tree::{tree, Tree};
use serde_test::{assert_tokens, Token};

#[test]
fn test_serde_round_trip() {
    let tree = tree!("a" => {"b", "c" => {"d", "e"}, "f"});
    let repr = serde_json::to_string(&tree).unwrap();
    println!("{repr}");
    let re_tree: Tree<&str> = serde_json::from_str(&repr).unwrap();
    println!("{re_tree}");
    assert_eq!(tree, re_tree);
}

#[test]
fn test_internal_serde_repr() {
    let tree = tree!("a");

    assert_tokens(
        &tree,
        &[
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::Str("value"),
            Token::Str("a"),
            Token::Str("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}
