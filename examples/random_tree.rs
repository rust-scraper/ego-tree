use std::{fmt, ops::Deref};

use ego_tree::{NodeMut, Tree};
use rand::{
    distributions::{Alphanumeric, Standard},
    prelude::Distribution,
    thread_rng, Rng,
};

#[allow(dead_code)]
fn complete_tree<T: Clone>(node: &mut NodeMut<T>, depth: usize, siblings: usize, value: T) {
    if depth == 0 {
        return;
    }

    for _ in 0..siblings {
        let mut child = node.append(value.clone());
        complete_tree(&mut child, depth - 1, siblings, value.clone());
    }
}

#[allow(dead_code)]
fn random_content_tree<T>(node: &mut NodeMut<T>, depth: usize, siblings: usize)
where
    Standard: Distribution<T>,
{
    if depth == 0 {
        return;
    }

    let mut rng = thread_rng();

    for _ in 0..siblings {
        let mut child = node.append(rng.gen::<T>());
        random_content_tree(&mut child, depth - 1, siblings);
    }
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

fn random_tree<T: Clone>(
    node: &mut NodeMut<T>,
    depth: usize,
    siblings: usize,
    pruning_prob: Probability,
    value: T,
) {
    if depth == 0 {
        return;
    }

    let mut rng = thread_rng();

    for _ in 0..siblings {
        let dice = rng.gen::<Probability>();
        if dice < pruning_prob {
            continue;
        }

        let mut child = node.append(value.clone());
        random_tree(&mut child, depth - 1, siblings, pruning_prob, value.clone());
    }
}

struct RandStr(String);

impl RandStr {
    fn new(string: String) -> RandStr {
        RandStr(string)
    }
}

impl Deref for RandStr {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RandStr {
    fn from(value: String) -> Self {
        RandStr::new(value)
    }
}

impl Distribution<RandStr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandStr {
        const LENGTH: usize = 10;
        rng.sample_iter(&Alphanumeric)
            .take(LENGTH)
            .map(char::from)
            .collect::<String>()
            .into()
    }
}

impl fmt::Display for RandStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

fn main() {
    let mut tree: Tree<String> = Tree::new("root".to_string());
    random_tree(
        &mut tree.root_mut(),
        50,
        10,
        0.87.try_into().unwrap(),
        "node".to_string(),
    );

    println!("{tree}");
}
