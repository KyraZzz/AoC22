use std::{cmp::Ordering, fmt};

use serde::Deserialize;
#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl Fn(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(n),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        // return the first ordering that isn't `Equal`
                        .find(|&ord| ord != Ordering::Equal)
                        // or compare the lengths
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{n}"),
            Node::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

fn part1() {
    let mut sum = 0;
    for (i, groups) in include_str!("input.txt").split("\n\n").enumerate() {
        let i = i + 1;

        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();
        println!("\n== Pair {i} ==");
        println!("l = {l:?}");
        println!("r = {r:?}");
        println!("l < r = {}", l < r);
        if l < r {
            sum += i;
        }
    }
    dbg!(sum);
}

fn main() {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];

    let mut packets = include_str!("input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    dbg!(decoder_key);
}
