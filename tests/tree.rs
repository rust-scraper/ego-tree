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
fn root() {
    let tree = Tree::new('a');
    assert_eq!(&'a', tree.root().value());
}

#[test]
fn root_mut() {
    let mut tree = Tree::new('a');
    assert_eq!(&'a', tree.root_mut().value());
}

#[test]
fn orphan() {
    let mut tree = Tree::new('a');
    let mut orphan = tree.orphan('b');
    assert_eq!(&'b', orphan.value());
    assert!(orphan.parent().is_none());
}

#[test]
fn get() {
    let tree = Tree::new('a');
    let id = tree.root().id();
    assert_eq!(Some(tree.root()), tree.get(id));
}

#[test]
fn get_mut() {
    let mut tree = Tree::new('a');
    let id = tree.root().id();
    assert_eq!(Some('a'), tree.get_mut(id).map(|mut n| *n.value()));
}

#[test]
fn clone() {
    let one = Tree::new('a');
    let two = one.clone();
    assert_eq!(one, two);
}

#[test]
fn eq() {
    let one = Tree::new('a');
    let two = Tree::new('a');
    assert_eq!(one, two);
}

#[test]
#[should_panic]
fn neq() {
    let one = Tree::new('a');
    let two = Tree::new('b');
    assert_eq!(one, two);
}
