#[macro_use]
extern crate ego_tree;

use ego_tree::NodeRef;

#[test]
fn value() {
    let mut tree = tree!('a');
    assert_eq!(&'a', tree.root_mut().value());
}

#[test]
fn id() {
    let mut tree = tree!('a');
    let id = tree.root_mut().id();
    assert_eq!(id, tree.get_mut(id).id());
}

#[test]
fn parent() {
    let mut tree = tree!('a' => { 'b' });
    assert_eq!(
        &'a',
        tree.root_mut()
            .first_child()
            .unwrap()
            .parent()
            .unwrap()
            .value()
    );
}

#[test]
fn prev_sibling() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(
        &'b',
        tree.root_mut()
            .last_child()
            .unwrap()
            .prev_sibling()
            .unwrap()
            .value()
    );
}

#[test]
fn next_sibling() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(
        &'c',
        tree.root_mut()
            .first_child()
            .unwrap()
            .next_sibling()
            .unwrap()
            .value()
    );
}

#[test]
fn first_child() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(&'b', tree.root_mut().first_child().unwrap().value());
}

#[test]
fn last_child() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(&'c', tree.root_mut().last_child().unwrap().value());
}

#[test]
fn has_siblings() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(true, tree.root_mut().first_child().unwrap().has_siblings());
    assert_eq!(false, tree.root_mut().has_siblings());
}

#[test]
fn has_children() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert_eq!(true, tree.root_mut().has_children());
    assert_eq!(false, tree.root_mut().first_child().unwrap().has_children());
}

#[test]
fn append_1() {
    let mut tree = tree!('a');
    tree.root_mut().append('b');

    let root = tree.root();
    let child = root.first_child().unwrap();

    assert_eq!(&'b', child.value());
    assert_eq!(Some(child), root.last_child());
    assert_eq!(Some(root), child.parent());
    assert_eq!(None, child.next_sibling());
    assert_eq!(None, child.next_sibling());
}

#[test]
fn append_2() {
    let mut tree = tree!('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = root.last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(&'c', c.value());
    assert_eq!(Some(root), b.parent());
    assert_eq!(Some(root), c.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(None, c.next_sibling());
}

#[test]
fn append_3() {
    let mut tree = tree!('a');
    tree.root_mut().append('b');
    tree.root_mut().append('c');
    tree.root_mut().append('d');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = b.next_sibling().unwrap();
    let d = root.last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(&'c', c.value());
    assert_eq!(&'d', d.value());
    assert_eq!(Some(root), b.parent());
    assert_eq!(Some(root), c.parent());
    assert_eq!(Some(root), d.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(Some(d), c.next_sibling());
    assert_eq!(Some(c), d.prev_sibling());
    assert_eq!(None, d.next_sibling());
}

#[test]
fn prepend_1() {
    let mut tree = tree!('a');
    tree.root_mut().prepend('b');

    let root = tree.root();
    let child = root.first_child().unwrap();

    assert_eq!(&'b', child.value());
    assert_eq!(Some(child), root.last_child());
    assert_eq!(Some(root), child.parent());
    assert_eq!(None, child.next_sibling());
    assert_eq!(None, child.next_sibling());
}

#[test]
fn prepend_2() {
    let mut tree = tree!('a');
    tree.root_mut().prepend('c');
    tree.root_mut().prepend('b');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = root.last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(&'c', c.value());
    assert_eq!(Some(root), b.parent());
    assert_eq!(Some(root), c.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(None, c.next_sibling());
}

#[test]
fn prepend_3() {
    let mut tree = tree!('a');
    tree.root_mut().prepend('d');
    tree.root_mut().prepend('c');
    tree.root_mut().prepend('b');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = b.next_sibling().unwrap();
    let d = root.last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(&'c', c.value());
    assert_eq!(&'d', d.value());
    assert_eq!(Some(root), b.parent());
    assert_eq!(Some(root), c.parent());
    assert_eq!(Some(root), d.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(Some(d), c.next_sibling());
    assert_eq!(Some(c), d.prev_sibling());
    assert_eq!(None, d.next_sibling());
}

#[test]
fn into() {
    let mut tree = tree!('a');
    let node_ref: NodeRef<_> = tree.root_mut().into();
    assert_eq!(&'a', node_ref.value());
}
