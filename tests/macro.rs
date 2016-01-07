#[macro_use]
extern crate ego_tree;

use ego_tree::Tree;

#[test]
fn empty() {
    let macro_tree: Tree<i32> = tree!();
    let manual_tree: Tree<i32> = Tree::default();
    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn root() {
    let macro_tree = tree!('a');
    let manual_tree = Tree::new('a');
    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn empty_children() {
    let macro_tree = tree!('a' => { });
    let manual_tree = Tree::new('a');
    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn single_child() {
    let macro_tree = tree!('a' => { 'b' });

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn single_child_comma() {
    let macro_tree = tree! {
        'a' => {
            'b',
        }
    };

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn leaves() {
    let macro_tree = tree!('a' => { 'b', 'c', 'd' });

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');
    manual_tree.root_mut().append('c');
    manual_tree.root_mut().append('d');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn leaves_comma() {
    let macro_tree = tree! {
        'a' => {
            'b',
            'c',
            'd',
        }
    };

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');
    manual_tree.root_mut().append('c');
    manual_tree.root_mut().append('d');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_empty_children() {
    let macro_tree = tree!('a' => { 'b' => { } });

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_single_child() {
    let macro_tree = tree!('a' => { 'b' => { 'c' } });

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');
    manual_tree.root_mut().last_child().unwrap().append('c');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_single_child_comma() {
    let macro_tree = tree! {
        'a' => {
            'b' => {
                'c',
            },
        }
    };

    let mut manual_tree = Tree::new('a');
    manual_tree.root_mut().append('b');
    manual_tree.root_mut().last_child().unwrap().append('c');

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_leaves() {
    let macro_tree = tree!('a' => { 'b' => { 'c', 'd', 'e' } });

    let mut manual_tree = Tree::new('a');
    {
        let mut node_mut = manual_tree.root_mut();
        node_mut.append('b');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('c');
        node_mut.append('d');
        node_mut.append('e');
    }

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_leaves_comma() {
    let macro_tree = tree! {
        'a' => {
            'b' => {
                'c',
                'd',
                'e',
            },
        }
    };

    let mut manual_tree = Tree::new('a');
    {
        let mut node_mut = manual_tree.root_mut();
        node_mut.append('b');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('c');
        node_mut.append('d');
        node_mut.append('e');
    }

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn nested_nested() {
    let macro_tree = tree!('a' => { 'b' => { 'c' => { 'd' } } });

    let mut manual_tree = Tree::new('a');
    {
        let mut node_mut = manual_tree.root_mut();
        node_mut.append('b');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('c');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('d');
    }

    assert_eq!(manual_tree, macro_tree);
}

#[test]
fn mixed() {
    let macro_tree = tree! {
        'a' => {
            'b',
            'c' => { },
            'd' => { 'e' },
            'f' => { 'g' => { 'h' } },
            'i',
        }
    };

    let mut manual_tree = Tree::new('a');
    {
        let mut node_mut = manual_tree.root_mut();
        node_mut.append('b');
        node_mut.append('c');
        node_mut.append('d');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('e');
        node_mut = node_mut.parent().unwrap();
        node_mut.append('f');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('g');
        node_mut = node_mut.last_child().unwrap();
        node_mut.append('h');
        node_mut = node_mut.parent().unwrap();
        node_mut = node_mut.parent().unwrap();
        node_mut.append('i');
    }

    assert_eq!(manual_tree, macro_tree);
}
