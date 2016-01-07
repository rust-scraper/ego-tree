#[macro_use]
extern crate ego_tree;

#[test]
fn values() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'a', &'b', &'c', &'d'],
        tree.values().collect::<Vec<_>>()
    );
}

#[test]
fn values_mut() {
    use std::ascii::AsciiExt;

    let mut tree = tree!('a' => { 'b', 'c', 'd' });

    for c in tree.values_mut() {
        *c = c.to_ascii_uppercase();
    }

    assert_eq!(
        vec![&'A', &'B', &'C', &'D'],
        tree.values().collect::<Vec<_>>()
    );
}

#[test]
fn into_values() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec!['a', 'b', 'c', 'd'],
        tree.into_values().collect::<Vec<_>>()
    );
}

#[test]
fn ancestors() {
    let tree = tree!('a' => { 'b' => { 'c' => { 'd' } } });
    let d = tree.root()
        .last_child().unwrap()
        .last_child().unwrap()
        .last_child().unwrap();
    assert_eq!(
        vec![&'c', &'b', &'a'],
        d.ancestors().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn prev_siblings() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'c', &'b'],
        tree.root()
            .last_child()
            .unwrap()
            .prev_siblings()
            .map(|n| n.value())
            .collect::<Vec<_>>()
    );
}

#[test]
fn next_siblings() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'c', &'d'],
        tree.root()
            .first_child()
            .unwrap()
            .next_siblings()
            .map(|n| n.value())
            .collect::<Vec<_>>()
    );
}

#[test]
fn children() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'b', &'c', &'d'],
        tree.root().children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn children_rev() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'d', &'c', &'b'],
        tree.root().children().rev().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn first_children() {
    let tree = tree!('a' => { 'b' => { 'd', 'e' }, 'c' });
    assert_eq!(
        vec![&'b', &'d'],
        tree.root().first_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn last_children() {
    let tree = tree!('a' => { 'b', 'c' => { 'd', 'e' } });
    assert_eq!(
        vec![&'c', &'e'],
        tree.root().last_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}
