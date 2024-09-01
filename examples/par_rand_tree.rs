use std::ops::Deref;

use ego_tree::Tree;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};
use rayon::prelude::*;

fn full_merge_tree<T: Clone>(depth: usize, siblings: usize, value: T) -> Tree<T> {
    let mut tree = Tree::new(value.clone());

    if depth == 0 {
        return tree;
    }

    for _ in 0..siblings {
        let subtree = full_merge_tree(depth - 1, siblings, value.clone());
        tree.root_mut().append_subtree(subtree);
    }

    tree
}

#[allow(dead_code)]
fn rand_merge_tree<T: Clone>(
    depth: usize,
    siblings: usize,
    prune_prob: Probability,
    value: T,
) -> Tree<T> {
    let mut tree = Tree::new(value.clone());

    if depth == 0 {
        return tree;
    }

    (0..siblings)
        .filter(|_| {
            let dice: Probability = thread_rng().gen();
            dice >= prune_prob
        })
        .for_each(|_| {
            let subtree = full_merge_tree(depth - 1, siblings, value.clone());
            tree.root_mut().append_subtree(subtree);
        });

    tree
}

fn par_rand_merge_tree<T: Clone + Send + Sync>(
    depth: usize,
    siblings: usize,
    prune_prob: Probability,
    value: T,
) -> Tree<T> {
    if depth == 0 {
        return Tree::new(value.clone());
    }

    let tree = (0..siblings)
        .into_par_iter()
        .filter(|_| {
            let dice: Probability = thread_rng().gen();
            dice >= prune_prob
        })
        .map(|_| full_merge_tree(depth - 1, siblings, value.clone()))
        .reduce(
            || Tree::new(value.clone()),
            |mut tr, sub| {
                tr.root_mut().append_subtree(sub);
                tr
            },
        );

    tree
}

fn main() {
    let tree = par_rand_merge_tree(50, 4, 0.5.try_into().unwrap(), 1);
    println!("{tree}");
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
struct Probability(f64);

impl TryFrom<f64> for Probability {
    type Error = f64;
    fn try_from(value: f64) -> Result<Probability, f64> {
        if (0.0f64..1.0f64).contains(&value) {
            Ok(Probability(value))
        } else {
            Err(value)
        }
    }
}

impl Deref for Probability {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Distribution<Probability> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Probability {
        rng.gen_range(0.0f64..1.0f64).try_into().unwrap()
    }
}
