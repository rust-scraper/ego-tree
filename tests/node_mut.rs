#[macro_use]
extern crate ego_tree;

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
