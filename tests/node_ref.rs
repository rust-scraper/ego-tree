#[macro_use]
extern crate ego_tree;

#[test]
fn value() {
    let tree = tree!('a');
    assert_eq!(&'a', tree.root().value());
}

#[test]
fn parent() {
    let tree = tree!('a' => { 'b' });
    let b = tree.root().first_child().unwrap();
    assert_eq!(tree.root(), b.parent().unwrap());
}

#[test]
fn prev_sibling() {
    let tree = tree!('a' => { 'b', 'c' });
    let c = tree.root().last_child().unwrap();
    assert_eq!(tree.root().first_child(), c.prev_sibling());
}

#[test]
fn next_sibling() {
    let tree = tree!('a' => { 'b', 'c' });
    let b = tree.root().first_child().unwrap();
    assert_eq!(tree.root().last_child(), b.next_sibling());
}

#[test]
fn first_child() {
    let tree = tree!('a' => { 'b', 'c' });
    assert_eq!(&'b', tree.root().first_child().unwrap().value());
}

#[test]
fn last_child() {
    let tree = tree!('a' => { 'b', 'c' });
    assert_eq!(&'c', tree.root().last_child().unwrap().value());
}

#[test]
fn index_of_child() {
    let tree = tree!('a' => { 'b', 'c' });
    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = root.last_child().unwrap();
    
    assert_eq!(0, root.index_of_child(&b).unwrap());
    assert_eq!(1, root.index_of_child(&c).unwrap());
    assert!(root.index_of_child(&root).is_none());
}

#[test]
fn has_siblings() {
    let tree = tree!('a' => { 'b', 'c' });
    assert_eq!(false, tree.root().has_siblings());
    assert_eq!(true, tree.root().first_child().unwrap().has_siblings());
}

#[test]
fn has_children() {
    let tree = tree!('a' => { 'b', 'c' });
    assert_eq!(true, tree.root().has_children());
    assert_eq!(false, tree.root().first_child().unwrap().has_children());
}

#[test]
fn clone() {
    let tree = tree!('a');
    let one = tree.root();
    let two = one.clone();
    assert_eq!(one, two);
}

#[test]
fn eq() {
    let tree = tree!('a');
    assert_eq!(tree.root(), tree.root());
}

#[test]
#[should_panic]
fn neq() {
    let tree = tree!('a' => { 'b', 'c' });
    assert_eq!(tree.root(), tree.root().first_child().unwrap());
}

#[test]
#[should_panic]
fn neq_tree() {
    let one = tree!('a');
    let two = one.clone();
    assert_eq!(one.root(), two.root());
}
