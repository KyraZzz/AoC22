// #[derive(Debug)]
// struct PathedIoError {
//     // construct an error struct so we can propagate the specific error through function calls.
//     path: String,
//     inner: std::io::Error,
// }

// the following can be replaced by the debug macro,
// but we can provide a more human-readable error message.
// This is actually already written in a crate fs-err.
// impl std::fmt::Debug for PathedIoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "for file {:?}: {}", self.path, self.inner)
//     }
// }

// fn read_input() -> Result<String, PathedIoError> {
//     let path = "input.txt";
//     match std::fs::read_to_string(path) {
//         Ok(s) => Ok(s),
//         Err(e) => Err(PathedIoError {
//             path: path.into(),
//             inner: e,
//         }),
//     }
// }

// with fs_err, can do even fancier with color-eyre
// fn read_input() -> Result<String, std::io::Error> {
//     fs_err::read_to_string("src/input.txt")
// }

// use color_eyre::eyre::Context;

// using color-eyre
// fn read_input() -> color_eyre::Result<String> {
//     let path = "src/input.txt";
//     let input = std::fs::read_to_string(path).wrap_err("reading input file")?;
//     Ok(input)
// }

fn main() -> color_eyre::Result<()> {
    // let input = std::fs::read_to_string("src/input.txt").unwrap();
    color_eyre::install()?;
    // let input = read_input()?;
    // println!("{input}");
    // read a file at compile time rather than runtime;
    // let input = include_str!("input.txt");
    // let mut lines = input.lines();
    // loop {
    //     let maybe_line = lines.next();
    //     match maybe_line {
    //         Some(line) => {
    //             println!("{}", line);
    //         }
    //         None => break,
    //     }
    // }
    // for line in include_str!("input.txt").lines() {
    //     println!("Got line: {}", line);
    // }

    // let lines = include_str!("input.txt")
    //     .lines()
    //     .map(|line| line.parse::<u64>().ok())
    //     .collect::<Vec<_>>();
    // let groups = lines
    //     .split(|line| line.is_none())
    //     .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
    //     .collect::<Vec<_>>();
    // println!("{:?}", groups);
    // let max_in_groups = groups.iter().max();
    // println!("{:?}", max_in_groups);

    Ok(())
}
