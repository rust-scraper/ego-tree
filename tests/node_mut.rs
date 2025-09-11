use ego_tree::{tree, NodeRef};

#[test]
fn value() {
    let mut tree = tree!('a');
    assert_eq!(&'a', tree.root_mut().value());
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
    assert!(tree.root_mut().first_child().unwrap().has_siblings());
    assert!(!tree.root_mut().has_siblings());
}

#[test]
fn has_children() {
    let mut tree = tree!('a' => { 'b', 'c' });
    assert!(tree.root_mut().has_children());
    assert!(!tree.root_mut().first_child().unwrap().has_children());
}

#[test]
fn for_each_next_sibling() {
    let mut tree = tree!(1 => { 2, 3, 4, 5, 6 });
    let mut root = tree.root_mut();
    let mut c = root.first_child().unwrap();

    c.for_each_next_sibling(|n| {
        *n.value() += 1;
    });

    let res = tree!(1 => { 2, 4, 5, 6, 7 });

    assert_eq!(tree, res);
}

#[test]
fn for_each_prev_sibling() {
    let mut tree = tree!(1 => { 2, 3, 4, 5, 6 });
    let mut root = tree.root_mut();
    let mut c = root.last_child().unwrap();

    c.for_each_prev_sibling(|n| {
        *n.value() += 1;
    });

    let res = tree!(1 => { 3, 4, 5, 6, 6 });

    assert_eq!(tree, res);
}

#[test]
fn for_each_sibling() {
    let rt = 2;
    let mut tree = tree!(rt => { 2, 3, 4, 5, 6 });
    let mut root = tree.root_mut();
    let mut c = root.last_child().unwrap();

    c.for_each_sibling(|n| {
        let v = n.parent().map(|mut p| *p.value()).unwrap();
        *n.value() += v;
    });

    let res = tree!(rt => { 4, 5, 6, 7, 8 });

    assert_eq!(tree, res);
}

#[test]
fn for_each_child() {
    let mut tree = tree!(1 => { 2, 3, 4, 5, 6 });
    let mut root = tree.root_mut();
    root.for_each_child(|n| *n.value() += 1);

    assert_eq!(*root.value(), 1);

    let res = tree!(1 => { 3, 4, 5, 6, 7 });

    assert_eq!(tree, res);
}

#[test]
fn for_each_descendant() {
    let mut tree = tree!(1 => { 2 => {3, 4, 5, 6}, 3, 4 => {0, 1}, 5, 6 => {1, 2} });
    let mut root = tree.root_mut();
    root.for_each_descendant(|n| *n.value() += 1);

    let tree2 = tree!(2 => { 3 => {4, 5, 6, 7}, 4, 5 => {1, 2}, 6, 7 => {2, 3} });

    assert_eq!(tree, tree2);
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
fn insert_before_first() {
    let mut tree = tree!('a' => { 'c' });
    tree.root_mut().first_child().unwrap().insert_before('b');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = root.last_child().unwrap();

    assert_eq!(&'b', b.value());
    assert_eq!(Some(root), b.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(None, c.next_sibling());
}

#[test]
fn insert_before() {
    let mut tree = tree!('a' => { 'b', 'd' });
    tree.root_mut().last_child().unwrap().insert_before('c');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = b.next_sibling().unwrap();
    let d = root.last_child().unwrap();

    assert_eq!(&'c', c.value());
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
fn insert_after_first() {
    let mut tree = tree!('a' => { 'b' });
    tree.root_mut().first_child().unwrap().insert_after('c');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = root.last_child().unwrap();

    assert_eq!(&'c', c.value());
    assert_eq!(Some(root), c.parent());
    assert_eq!(None, b.prev_sibling());
    assert_eq!(Some(c), b.next_sibling());
    assert_eq!(Some(b), c.prev_sibling());
    assert_eq!(None, c.next_sibling());
}

#[test]
fn insert_after() {
    let mut tree = tree!('a' => { 'b', 'd' });
    tree.root_mut().first_child().unwrap().insert_after('c');

    let root = tree.root();
    let b = root.first_child().unwrap();
    let c = b.next_sibling().unwrap();
    let d = root.last_child().unwrap();

    assert_eq!(&'c', c.value());
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
fn insert_at_index() {
    let mut tree = tree!('a' => { 'b', 'c', 'e' });

    {
        let mut root = tree.root_mut();
        let mut child = root.first_child().unwrap();

        for _ in 0..2 {
            child = child.into_next_sibling().unwrap();
        }

        child.insert_before('d');
    }

    let descendants = tree
        .root()
        .descendants()
        .map(|n| n.value())
        .collect::<Vec<_>>();

    assert_eq!(&[&'a', &'b', &'c', &'d', &'e',], &descendants[..]);
}

#[test]
fn detach() {
    let mut tree = tree!('a' => { 'b', 'd' });
    let mut root = tree.root_mut();
    let mut b = root.first_child().unwrap();
    let mut c = b.insert_after('c');
    c.detach();

    assert!(c.parent().is_none());
    assert!(c.prev_sibling().is_none());
    assert!(c.next_sibling().is_none());
}

#[test]
#[should_panic(expected = "Cannot append node as a child to itself")]
fn append_id_itself() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'c', 'd' },
            'e' => { 'f', 'g' },
        }
    };
    let mut a = tree.root_mut();
    let mut e = a.last_child().unwrap();
    e.append_id(e.id());
}

#[test]
fn append_id_noop() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'c', 'd' },
            'e' => { 'f', 'g' },
        }
    };
    let mut a = tree.root_mut();
    let e_id = a.last_child().unwrap().id();
    a.append_id(e_id);
    assert_eq!(a.first_child().unwrap().next_sibling().unwrap().id(), e_id);
}

#[test]
#[should_panic(expected = "Cannot prepend node as a child to itself")]
fn prepend_id_itself() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'c', 'd' },
            'e' => { 'f', 'g' },
        }
    };
    let mut a = tree.root_mut();
    let mut e = a.last_child().unwrap();
    e.prepend_id(e.id());
}

#[test]
fn prepend_id_noop() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'c', 'd' },
            'e' => { 'f', 'g' },
        }
    };
    let mut a = tree.root_mut();
    let b_id = a.first_child().unwrap().id();
    a.prepend_id(b_id);
    assert_eq!(a.last_child().unwrap().prev_sibling().unwrap().id(), b_id);
}

#[test]
fn reparent_from_id_append() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'c', 'd' },
            'e' => { 'f', 'g' },
        }
    };
    let e_id = tree.root().last_child().unwrap().id();
    tree.root_mut()
        .first_child()
        .unwrap()
        .reparent_from_id_append(e_id);

    let b = tree.root().first_child().unwrap();
    let e = tree.root().last_child().unwrap();
    let d = b.first_child().unwrap().next_sibling().unwrap();
    let g = b.last_child().unwrap();
    let f = g.prev_sibling().unwrap();

    assert!(!e.has_children());
    assert_eq!(&'f', f.value());
    assert_eq!(&'g', g.value());
    assert_eq!(Some(f), d.next_sibling());
    assert_eq!(Some(d), f.prev_sibling());
}

#[test]
fn reparent_from_id_prepend() {
    let mut tree = tree! {
        'a' => {
            'b' => { 'f', 'g' },
            'e' => { 'c', 'd' },
        }
    };
    let e_id = tree.root().last_child().unwrap().id();
    tree.root_mut()
        .first_child()
        .unwrap()
        .reparent_from_id_prepend(e_id);

    let b = tree.root().first_child().unwrap();
    let e = tree.root().last_child().unwrap();
    let c = b.first_child().unwrap();
    let d = c.next_sibling().unwrap();
    let f = b.last_child().unwrap().prev_sibling().unwrap();

    assert!(!e.has_children());
    assert_eq!(&'c', c.value());
    assert_eq!(&'d', d.value());
    assert_eq!(Some(f), d.next_sibling());
    assert_eq!(Some(d), f.prev_sibling());
}

#[test]
fn reparent_from_id_append_multiple_siblings() {
    // Test reparenting when source has 3+ children to ensure all siblings get proper parent update
    let mut tree = tree! {
        'a' => {
            'b' => { 'x' },  // destination node
            'c' => { 'd', 'e', 'f' },  // source node with 3 children
        }
    };

    let c_id = tree.root().last_child().unwrap().id();
    let b_id = tree.root().first_child().unwrap().id();

    // Reparent children from 'c' to 'b'
    tree.root_mut()
        .first_child()
        .unwrap()
        .reparent_from_id_append(c_id);

    let b = tree.get(b_id).unwrap();
    let x = b.first_child().unwrap();  // original child of b
    let d = x.next_sibling().unwrap();  // first reparented child
    let e = d.next_sibling().unwrap();  // middle reparented child
    let f = e.next_sibling().unwrap();  // last reparented child

    // All children should now have 'b' as their parent
    assert_eq!(Some(b), x.parent());
    assert_eq!(Some(b), d.parent());
    assert_eq!(Some(b), e.parent());
    assert_eq!(Some(b), f.parent());
}

#[test]
fn reparent_from_id_prepend_multiple_siblings() {
    // Test reparenting when source has 3+ children to ensure all siblings get proper parent update
    let mut tree = tree! {
        'a' => {
            'b' => { 'x' },  // destination node
            'c' => { 'd', 'e', 'f' },  // source node with 3 children
        }
    };

    let c_id = tree.root().last_child().unwrap().id();
    let b_id = tree.root().first_child().unwrap().id();

    // Reparent children from 'c' to 'b'
    tree.root_mut()
        .first_child()
        .unwrap()
        .reparent_from_id_prepend(c_id);

    let b = tree.get(b_id).unwrap();
    let d = b.first_child().unwrap();  // first reparented child
    let e = d.next_sibling().unwrap(); // middle reparented child
    let f = e.next_sibling().unwrap(); // last reparented child
    let x = f.next_sibling().unwrap(); // original child of b

    // All children should now have 'b' as their parent
    assert_eq!(Some(b), d.parent());
    assert_eq!(Some(b), e.parent());
    assert_eq!(Some(b), f.parent());
    assert_eq!(Some(b), x.parent());
}

#[test]
fn into() {
    let mut tree = tree!('a');
    let node_ref: NodeRef<_> = tree.root_mut().into();
    assert_eq!(&'a', node_ref.value());
}
