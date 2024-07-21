use itertools::Itertools;
use std::ops::RangeInclusive;
trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn part1() {
    let redundant = include_str!("input.txt")
        .lines()
        .map(|line| {
            {
                line.split(',')
                    .map(|range| {
                        range
                            .split('-')
                            .map(|n| n.parse().expect("Range start and end should be u32"))
                            .collect_tuple::<(u32, u32)>()
                            .map(|(start, end)| start..=end)
                            .expect("Each range should have a start and end")
                    })
                    .collect_tuple::<(_, _)>()
                    .expect("Each line must have a pair of ranges")
            }
        })
        .filter(|(first, second)| first.contains_range(second) || second.contains_range(first))
        .count();
    println!("{:?}", redundant);
}

fn part2() {
    let redundant = include_str!("input.txt")
        .lines()
        .map(|line| {
            {
                line.split(',')
                    .map(|range| {
                        range
                            .split('-')
                            .map(|n| n.parse().expect("Range start and end should be u32"))
                            .collect_tuple::<(u32, u32)>()
                            .map(|(start, end)| start..=end)
                            .expect("Each range should have a start and end")
                    })
                    .collect_tuple::<(_, _)>()
                    .expect("Each line must have a pair of ranges")
            }
        })
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count();
    println!("{:?}", redundant);
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let _ = part2();
    Ok(())
}
