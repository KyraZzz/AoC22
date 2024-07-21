#[derive(Clone, Copy, Debug)]
struct Range {
    start: u8,
    end: u8,
}

impl TryFrom<String> for Range {
    type Error = color_eyre::eyre::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let range = s
            .split('-')
            .into_iter()
            .map(|sec| sec.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        if range.len() == 2 {
            Ok(Range {
                start: range[0],
                end: range[1],
            })
        } else {
            Err(color_eyre::eyre::eyre!("Invalid range: {:?}", range))
        }
    }
}

impl Range {
    fn within_range(self, other: Range) -> bool {
        return other.start >= self.start && other.end <= self.end;
    }

    fn overlap(self, other: Range) -> bool {
        return !(other.end < self.start || other.start > self.end);
    }
}

fn part1() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let total_overlaps: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let ranges = line
                .split(",")
                .map(|s| s.to_string().try_into().unwrap())
                .collect::<Vec<Range>>();
            let (first_range, second_range) = (ranges[0], ranges[1]);
            if first_range.within_range(second_range) || second_range.within_range(first_range) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{:?}", total_overlaps);

    Ok(())
}

fn part2() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let total_overlaps: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let ranges = line
                .split(",")
                .map(|s| s.to_string().try_into().unwrap())
                .collect::<Vec<Range>>();
            let (first_range, second_range) = (ranges[0], ranges[1]);
            if first_range.overlap(second_range) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{:?}", total_overlaps);

    Ok(())
}

fn main() -> color_eyre::eyre::Result<()> {
    part2();
    Ok(())
}
