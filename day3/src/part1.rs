#[derive(Clone, Copy, PartialEq, Eq)]
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

fn part1() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let mut total_score = 0;
    for line in include_str!("input.txt").lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let dup = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        .copied()
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("There should be exactly one duplicated item")
            .priority();
        total_score += dup;
    }
    println!("{}", total_score);

    Ok(())
}
