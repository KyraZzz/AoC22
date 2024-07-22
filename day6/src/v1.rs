use std::collections::{HashSet, VecDeque};

fn decode_line(line: &str) -> Option<usize> {
    let chars = line.chars().collect::<Vec<_>>();
    let mut signals = VecDeque::new();
    let mut signal_set: HashSet<char> = HashSet::new();
    let mut char_idx = 0;
    while char_idx < chars.len() {
        if signal_set.len() == 14 {
            return Some(char_idx);
        }

        let char = chars[char_idx];
        if signal_set.contains(&char) {
            while !signals.is_empty() {
                let pop_char = signals.pop_front().unwrap();
                signal_set.remove(&pop_char);
                if pop_char != char {
                    continue;
                } else {
                    break;
                }
            }
        }
        signals.push_back(char);
        signal_set.insert(char);

        char_idx += 1;
    }

    None
}

fn main() {
    let res = include_str!("input.txt")
        .lines()
        .map(|line| decode_line(line))
        .collect::<Vec<_>>();

    println!("{:?}", res);
}
