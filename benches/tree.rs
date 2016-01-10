#![feature(test)]

extern crate test;
extern crate ego_tree;

use test::Bencher;
use ego_tree::Tree;

#[bench]
fn orphan(b: &mut Bencher) {
    let mut tree = Tree::new(());
    b.iter(|| { tree.orphan(()); });
}

#[bench]
fn append(b: &mut Bencher) {
    let mut tree = Tree::new(());
    let mut root = tree.root_mut();
    b.iter(|| { root.append(()); });
}

#[bench]
fn prepend(b: &mut Bencher) {
    let mut tree = Tree::new(());
    let mut root = tree.root_mut();
    b.iter(|| { root.prepend(()); });
}

#[bench]
fn insert_before(b: &mut Bencher) {
    let mut tree = Tree::new(());
    let mut root = tree.root_mut();
    let mut node = root.append(());
    b.iter(|| { node.insert_before(()); });
}

#[bench]
fn insert_after(b: &mut Bencher) {
    let mut tree = Tree::new(());
    let mut root = tree.root_mut();
    let mut node = root.append(());
    b.iter(|| { node.insert_after(()); });
}

#[bench]
fn append_detach(b: &mut Bencher) {
    let mut tree = Tree::new(());
    let mut root = tree.root_mut();
    b.iter(|| { root.append(()).detach(); });
}
