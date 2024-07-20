/*
A X: rock (1)
B Y: paper (2)
C Z: scissor (3)

2 - 1 == 1
3 - 2 == 1
1 - 3 == -2
*/
use std::{collections::HashMap, str::FromStr};

use color_eyre::eyre::Ok;
fn v1() {
    let vm = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let scores = include_str!("input.txt")
        .lines()
        .map(|line| {
            let parsed = line.split(' ').map(|v| vm.get(v)).collect::<Vec<_>>();
            let mut score: i32 = 0;
            score += parsed[1].unwrap();
            let diff = parsed[1].unwrap() - parsed[0].unwrap();
            if diff == 1 || diff == -2 {
                score += 6;
            } else if diff == 0 {
                score += 3;
            }
            score
        })
        .sum::<i32>();
    println!("{:?}", scores);
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, theirs: Move) -> bool {
        matches!(
            (self, theirs),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err(color_eyre::eyre::eyre!(
                "expected <theirs>SP<ours>EOF, got {s:?}"
            ));
        };
        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Round {
    fn score(self) -> usize {
        self.ours.inherent_points() + self.ours.outcome(self.theirs).inherent_points()
    }
}
fn part1() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let rounds: Vec<_> = include_str!("test.txt")
        .lines()
        .map(|line| line.parse::<Round>())
        .collect::<Result<_, _>>()?;
    let score: usize = rounds.iter().map(|round| round.score()).sum();
    print!("{:?}", score);

    Ok(())
}

fn main -> color_eyre::Result<()> {
    part1()
}