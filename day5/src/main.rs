use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};

#[derive(Debug)]
struct Crate(char);

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // `drop` takes a value and returns nothing
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

// struct Stack {
//     crates: Vec<&Crate>,
// }

// struct Move {
//     num: usize,
//     start: usize,
//     end: usize,
// }

fn main() -> color_eyre::eyre::Result<()> {
    Ok(())
}
