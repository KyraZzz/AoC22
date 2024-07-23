use std::collections::HashSet;

use itertools::Itertools;

fn decode_line(line: &str) -> Option<usize> {
    line.as_bytes()
        .windows(4)
        .position(|window| window.iter().unique().count() == 4)
        .map(|pos| pos + 4)
}

fn main() {
    let res = include_str!("input.txt")
        .lines()
        .map(|line| decode_line(line))
        .collect::<Vec<_>>();

    println!("{:?}", res);
}
