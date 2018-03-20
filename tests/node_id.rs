extern crate ego_tree;

use ego_tree::Tree;

#[test]
fn clone() {
    let tree = Tree::new('a');
    let one = tree.root().id();
    let two = one.clone();
    assert_eq!(one, two);
    unsafe { assert_eq!(tree.get(one), tree.get(two)); }
}

#[test]
fn eq() {
    let tree = Tree::new('a');
    let one = tree.root().id();
    let two = tree.root().id();
    assert_eq!(one, two);
}

#[test]
#[should_panic]
fn neq() {
    let mut tree = Tree::new('a');
    let id = tree.root_mut().append('b').id();
    assert_eq!(tree.root().id(), id);
}
