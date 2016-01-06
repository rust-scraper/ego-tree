extern crate ego_tree;

use ego_tree::Tree;

#[test]
fn new() {
    let tree = Tree::new('a');
    let root = tree.root();
    assert_eq!(&'a', root.value());
    assert_eq!(None, root.parent());
    assert_eq!(None, root.prev_sibling());
    assert_eq!(None, root.next_sibling());
    assert_eq!(None, root.first_child());
    assert_eq!(None, root.last_child());
}

#[test]
fn get() {
    let tree = Tree::new('a');
    let id = tree.root().id();
    let root = tree.get(id);
    assert_eq!(&'a', root.value());
}

#[test]
fn orphan() {
    let mut tree = Tree::new('a');
    tree.orphan('b');
    let root = tree.root();
    assert_eq!(&'a', root.value());
    assert_eq!(None, root.parent());
    assert_eq!(None, root.prev_sibling());
    assert_eq!(None, root.next_sibling());
    assert_eq!(None, root.first_child());
    assert_eq!(None, root.last_child());
}

#[test]
fn append_1() {
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');

    let root = tree.root();
    assert_eq!(&'a', root.value());
    assert_eq!(root.first_child(), root.last_child());

    let child = root.first_child().unwrap();
    assert_eq!(&'b', child.value());
    assert_eq!(Some(root), child.parent());
    assert_eq!(None, child.prev_sibling());
    assert_eq!(None, child.next_sibling());
    assert_eq!(None, child.first_child());
    assert_eq!(None, child.last_child());
}

#[test]
fn append_2() {
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');

    let root = tree.root();
    assert_eq!(&'a', root.value());

    let first_child = root.first_child().unwrap();
    assert_eq!(&'b', first_child.value());
    assert_eq!(Some(root), first_child.parent());
    assert_eq!(None, first_child.prev_sibling());

    let last_child = root.last_child().unwrap();
    assert_eq!(&'c', last_child.value());
    assert_eq!(Some(root), last_child.parent());
    assert_eq!(None, last_child.next_sibling());

    assert_eq!(Some(last_child), first_child.next_sibling());
    assert_eq!(Some(first_child), last_child.prev_sibling());
}

#[test]
fn append_3() {
    let mut tree = Tree::new('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');
    tree.root_mut().append('d');

    let b = tree.root().first_child().unwrap();
    let c = b.next_sibling().unwrap();
    let d = tree.root().last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(&'c', c.value());
    assert_eq!(&'d', d.value());

    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(Some(d), c.next_sibling());
    assert_eq!(Some(c), d.prev_sibling());
}
