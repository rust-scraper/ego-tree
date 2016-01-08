extern crate ego_tree;

use ego_tree::Tree;

#[test]
fn clone() {
    let tree = Tree::new('a');
    let one = tree.root().id();
    let two = one.clone();
    assert_eq!(one, two);
    assert_eq!(tree.get(one), tree.get(two));
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
fn other_tree() {
    let one = Tree::new('a');
    let two = Tree::new('a');
    one.get(two.root().id());
}

#[test]
#[should_panic]
fn cloned_tree() {
    let one = Tree::new('a');
    let two = one.clone();
    one.get(two.root().id());
}
