extern crate ego_tree;

use ego_tree::{Tree, Ref};

#[test]
fn ancestors() {
    let mut tree = Tree::new('a');
    let b = tree.root_mut().append('b');
    let c = tree.get_mut(b).append('c');
    let d = tree.get_mut(c).append('d');

    let d = tree.get(d);
    let mut iter = d.ancestors();
    assert_eq!(Some(&'d'), iter.next().as_ref().map(Ref::value));
    assert_eq!(Some(&'c'), iter.next().as_ref().map(Ref::value));
    assert_eq!(Some(&'b'), iter.next().as_ref().map(Ref::value));
    assert_eq!(Some(&'a'), iter.next().as_ref().map(Ref::value));
    assert_eq!(None, iter.next());
}
