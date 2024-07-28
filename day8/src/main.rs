fn num_increasing_order(arr: &Vec<u8>) {
    let mut maxv = arr[0];
    let mut 
    let mut cnt = 
}

fn main() {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in include_str!("test.txt").lines() {
        grid.push(
            line.chars()
                .into_iter()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect(),
        );
    }
    let rows = grid.len();
    let cols = grid[0].len();

    // consider each row twice, one from top and one from bottom
    // consider each column twice, one from left and one from right
    // for each vector, count num items are in increasing order
    for row in grid[1..rows].iter() {
        let rev_row = row.into_iter().rev();
        
    }
    println!("{:?}", visible);
}
