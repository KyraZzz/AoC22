use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Move {
    dir: String,
    step: i32,
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let start_H: (i32, i32) = (0, 0);
    let start_T: (i32, i32) = (0, 0);
    let mut current_H = start_H;
    let mut current_T = start_T;

    let num_rows = 5;
    let num_cols = 6;

    let moves = include_str!("test.txt")
        .lines()
        .map(|line| {
            let v = line
                .split(" ")
                .map(|x| x.parse::<String>().unwrap())
                .collect::<Vec<String>>();
            Move {
                dir: v[0].clone(),
                step: v[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Move>>();

    let dx = [0, 0, -1, 1];
    let dy = [-1, 1, 0, 0];
    for m in moves {
        let diff_x = current_H.0 - current_T.0;
        let diff_y = current_H.1 - current_T.1;
        let step_change = match m.dir.as_str() {
            "U" => (m.step * dx[0], m.step * dy[0]),
            "D" => (m.step * dx[1], m.step * dy[1]),
            "L" => (m.step * dx[2], m.step * dy[2]),
            "R" => (m.step * dx[3], m.step * dy[3]),
            _ => panic!("Invalid"),
        };

        current_H = (current_H.0 + step_change.0, current_H.1 + step_change.1);
        current_T = (current_T.0 + step_change.0, current_T.1 + step_change.1);
        let diff_x = current_H.0 - current_T.0;
        let diff_y = current_H.1 - current_T.1;
        if diff_x.abs() > 2 || diff_y.abs() > 2 {
            panic!("Invalid");
        }
    }

    Ok(())
}
