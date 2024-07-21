use im::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
        }
    }
}

impl Item {
    fn priority(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

fn part2() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let mut total_score = 0;
    let lines = include_str!("input.txt").lines();
    let mut num_members = 0;
    let mut items: HashSet<Item> = HashSet::new();
    for line in lines {
        let current = line
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        num_members += 1;
        if num_members == 1 {
            items = current.into_iter().collect::<HashSet<_>>();
            continue;
        } else {
            items = items
                .iter()
                .filter(|&item| current.contains(item))
                .copied()
                .collect::<HashSet<_>>();
        }
        if num_members == 3 {
            println!("{:?}", items);
            assert!(items.len() == 1);
            num_members = 0;
            total_score += items.iter().collect::<Vec<_>>()[0].priority();
        }
    }
    println!("{}", total_score);

    Ok(())
}
use itertools::Itertools;
fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let mut total_score = 0;
    // let rucksacks = include_str!("input.txt")
    //     .lines()
    //     .map(|line| {
    //         line.bytes()
    //             .map(Item::try_from)
    //             .collect::<Result<HashSet<Item>, _>>()
    //     })
    //     .collect::<Vec<_>>();
    // let total_score = itertools::process_results(rucksacks, |rs| {
    //     rs.tuples()
    //         .map(|(a, b, c)| {
    //             a.iter()
    //                 .copied()
    //                 .find(|i| b.contains(i) && c.contains(i))
    //                 .map(|i| i.priority())
    //                 .unwrap_or_default()
    //         })
    //         .sum::<usize>()
    // })?;
    let total_score: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| b.try_into().unwrap())
                .collect::<HashSet<Item>>()
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            chunks
                .reduce(|a, b| a.intersection(b))
                .expect("We always have three chunks")
                .iter()
                .next()
                .expect("Always have one common badge")
                .priority()
        })
        .sum();
    println!("{:?}", total_score);

    Ok(())
}
