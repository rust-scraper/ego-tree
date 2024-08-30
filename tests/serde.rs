#![cfg(feature = "serde")]

use std::io::Read;

use ego_tree::{tree, Tree};
use serde_test::{assert_tokens, Token};

#[test]
fn test_internal_serde_repr_trivial() {
    let tree = tree!("a");

    assert_tokens(
        &tree,
        &[
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("a"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_internal_serde_repr() {
    let tree = tree!("a" => {"b", "c" => {"d", "e"}, "f"});

    assert_tokens(
        &tree,
        &[
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("a"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(3) },
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("b"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("c"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(2) },
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("d"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("e"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::Struct {
                name: "Node",
                len: 2,
            },
            Token::BorrowedStr("value"),
            Token::BorrowedStr("f"),
            Token::BorrowedStr("children"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_hypercomlex_json_tree() {
    let infile = std::fs::File::open("data/tree.json").expect("Faile to open tree.json");

    let tree: Tree<String> =
        serde_json::from_reader(infile).expect("Failed to deserialize the tree in tree.json");

    println!("{tree}");
}
