extern crate ego_tree;

use ego_tree::Tree;

#[test]
fn values() {
    let mut tree = Tree::new('a');
    {
        let mut root = tree.root_mut();
        root.append('b');
        root.append('c');
        root.append('d');
    }

    assert_eq!(
        vec![&'a', &'b', &'c', &'d'],
        tree.values().collect::<Vec<_>>()
    );
}

#[test]
fn values_mut() {
    use std::ascii::AsciiExt;

    let mut tree = Tree::new('a');
    {
        let mut root = tree.root_mut();
        root.append('b');
        root.append('c');
        root.append('d');
    }

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
    let mut tree = Tree::new('a');
    {
        let mut root = tree.root_mut();
        root.append('b');
        root.append('c');
        root.append('d');
    }

    assert_eq!(
        vec!['a', 'b', 'c', 'd'],
        tree.into_values().collect::<Vec<_>>()
    );
}

#[test]
fn ancestors() {
    let mut tree = Tree::new('a');
    let b = tree.root_mut().append('b');
    let c = tree.get_mut(b).append('c');
    let d = tree.get_mut(c).append('d');

    let d = tree.get(d);
    assert_eq!(
        vec![&'c', &'b', &'a'],
        d.ancestors().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn prev_siblings() {
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');
    tree.root_mut().append('d');

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
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');
    tree.root_mut().append('d');

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
fn first_children() {
    let mut tree = Tree::new('a');
    let b = tree.root_mut().append('b');
    tree.root_mut().append('c');
    tree.get_mut(b).append('d');
    tree.get_mut(b).append('e');

    assert_eq!(
        vec![&'b', &'d'],
        tree.root().first_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn last_children() {
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');
    let c = tree.root_mut().append('c');
    tree.get_mut(c).append('d');
    tree.get_mut(c).append('e');

    assert_eq!(
        vec![&'c', &'e'],
        tree.root().last_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}
