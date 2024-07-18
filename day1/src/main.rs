/*
The question is really simple, but there are a few things that we could learn from implementing the solution in rust:
- Read inputs from a file
- Parse inputs and convert strings into integers
- Process inputs in batches using some advanced tools
*/

/* dumb version 1
Benchmark 1:
  Time (mean ± σ):      31.1 ms ±  90.7 ms    [User: 1.2 ms, System: 0.9 ms]
  Range (min … max):     1.1 ms … 289.3 ms    10 runs
*/
fn v1() {
    // read from test.txt
    let path = "src/test.txt";
    let lines = std::fs::read_to_string(path).unwrap();
    let l = lines.split("\n").collect::<Vec<_>>();
    let mut res: Vec<u32> = vec![];
    let mut sum: u32 = 0;
    for line in l {
        if line == "" {
            res.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    res.push(sum);
    println!("{:?}", res.iter().max().unwrap());
}

/*
Benchmark 1:
  Time (mean ± σ):      22.3 ms ±  69.2 ms    [User: 1.2 ms, System: 0.8 ms]
  Range (min … max):     0.7 ms … 242.0 ms
*/
fn v3() {
    let lines = include_str!("test.txt")
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let max_v = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u32>())
        .max()
        .unwrap();
    print!("{:?}", max_v);
}

/*
part 1 final version
*/
use ::itertools;
use itertools::Itertools;
fn part1() {
    // lets use itertools
    let vs = include_str!("test.txt")
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
        .unwrap();

    println!("{:?}", vs);
}

fn part2() {
    // lets use itertools
    let answer = include_str!("test.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .sum::<u64>();

    println!("{:?}", answer);
}

fn main() {
    part2();
}
