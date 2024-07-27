use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::preceded,
    IResult,
};
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NOOP,
    ADDX(i32),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);
        alt((value(Self::NOOP, noop), map(addx, Self::ADDX)))(input)
    }
}

fn check_cnt(cnt: i32, x: i32) -> i32 {
    if cnt == 20 || (cnt - 20) % 40 == 0 {
        println!("cnt: {:?}, x: {:?}, cnt * x: {:?}", cnt, x, cnt * x);
        return cnt * x;
    }
    return 0;
}

fn part1() {
    let mut x = 1;
    let mut cnt = 0;
    let mut res = 0;
    for line in include_str!("input.txt").lines() {
        let v: Vec<String> = line
            .split(" ")
            .map(|x| x.parse::<String>().unwrap())
            .collect();

        let instr = match v[0].as_str() {
            "noop" => Instruction::NOOP,
            "addx" => Instruction::ADDX(v[1].parse::<i32>().unwrap()),
            _ => panic!("Invalid"),
        };

        match instr {
            Instruction::NOOP => {
                cnt += 1;
                res += check_cnt(cnt, x);
            }
            Instruction::ADDX(v) => {
                cnt += 1;
                res += check_cnt(cnt, x);
                cnt += 1;
                res += check_cnt(cnt, x);
                x += v;
            }
        };
    }
    println!("{:?}", res);
}

fn process_cycle(cycle: i32, varr: &mut Vec<String>, x: i32) {
    let position = (cycle - 1) % 40 + 1;
    let lb = max(x, 1);
    let ub = min(x + 2, 40);
    let symbol = if position >= lb && position <= ub {
        '#'
    } else {
        '.'
    };
    varr.push(symbol.to_string());
    if cycle != 0 && cycle % 40 == 0 {
        println!("{:?}", varr.join(""));
        varr.clear();
    }
}

fn main() {
    let mut x = 1;
    let mut cycle = 1;
    let mut varr: Vec<String> = vec![];
    let instrs = include_str!("input.txt")
        .lines()
        .map(|line| {
            Instruction::parse(line).unwrap().1
            // let v: Vec<String> = line
            //     .split(" ")
            //     .map(|x| x.parse::<String>().unwrap())
            //     .collect();

            // let instr = match v[0].as_str() {
            //     "noop" => Instruction::NOOP,
            //     "addx" => Instruction::ADDX(v[1].parse::<i32>().unwrap()),
            //     _ => panic!("Invalid"),
            // };
            // instr
        })
        .collect::<Vec<_>>();

    for instr in instrs {
        process_cycle(cycle, &mut varr, x);
        match instr {
            Instruction::NOOP => {
                cycle += 1;
            }
            Instruction::ADDX(v) => {
                cycle += 1;
                process_cycle(cycle, &mut varr, x);
                cycle += 1;
                x += v;
            }
        }
    }
}
