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
