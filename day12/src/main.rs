use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    is_start: bool,
    is_end: bool,
    elevation: char,
}

impl Node {
    fn new(input: char) -> Node {
        let mut is_start = false;
        let mut is_end = false;
        let mut elevation = input;
        if elevation == 'S' {
            is_start = true;
            elevation = 'a';
        } else if elevation == 'a' {
            is_start = true;
        } else if elevation == 'E' {
            is_end = true;
            elevation = 'z';
        }
        Node {
            is_start: is_start,
            is_end: is_end,
            elevation: elevation,
        }
    }

    fn is_start_node(&self) -> bool {
        self.is_start
    }

    fn is_end_node(&self) -> bool {
        self.is_end
    }

    fn can_move_to(&self, other: &Node) -> bool {
        let diff: i32 = (other.elevation as u8) as i32 - (self.elevation as u8) as i32;
        diff <= 1
    }
}

fn part1() {
    let mut grid: Vec<Vec<Node>> = vec![];
    let mut start_xy = (0, 0);
    let mut end_xy = (0, 0);
    let mut row = 0;
    for line in include_str!("input.txt").lines() {
        let mut row_nodes = vec![];
        let mut col = 0;
        for v in line.chars() {
            let n = Node::new(v);
            row_nodes.push(n.clone());
            if n.is_start_node() {
                start_xy = (row, col);
            } else if n.is_end_node() {
                end_xy = (row, col);
            }
            col += 1;
        }
        grid.push(row_nodes);
        row += 1;
    }

    let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start_xy);
    visited[start_xy.0][start_xy.1] = 0;

    while queue.len() > 0 {
        let current_xy = queue.pop_front().unwrap();
        let current_node = grid[current_xy.0][current_xy.1].clone();
        if current_node.is_end_node() {
            break;
        }
        let dx: [i32; 4] = [0, 0, -1, 1];
        let dy: [i32; 4] = [-1, 1, 0, 0];
        for i in 0..4 {
            let nx = current_xy.0 as i32 + dx[i];
            let ny = current_xy.1 as i32 + dy[i];
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                continue;
            }
            if visited[nx as usize][ny as usize] != -1 {
                continue;
            }
            if current_node.can_move_to(&grid[nx as usize][ny as usize]) {
                visited[nx as usize][ny as usize] = visited[current_xy.0][current_xy.1] + 1;
                queue.push_back((nx as usize, ny as usize));
            }
            println!("{:?}", queue);
        }
    }
    println!("{:?}", start_xy);
    println!("{:?}", visited[start_xy.0][start_xy.1]);
    println!("{:?}", end_xy);
    println!("{:?}", visited[end_xy.0][end_xy.1]);
}

fn main() {
    let mut grid: Vec<Vec<Node>> = vec![];
    let mut start_xys: Vec<(usize, usize)> = vec![];
    let mut end_xy = (0, 0);
    let mut row = 0;
    for line in include_str!("input.txt").lines() {
        let mut row_nodes = vec![];
        let mut col = 0;
        for v in line.chars() {
            let n = Node::new(v);
            row_nodes.push(n.clone());
            if n.is_start_node() {
                start_xys.push((row, col));
            } else if n.is_end_node() {
                end_xy = (row, col);
            }
            col += 1;
        }
        grid.push(row_nodes);
        row += 1;
    }

    let mut steps: Vec<i32> = vec![];
    for start_xy in start_xys.iter() {
        let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(*start_xy);
        visited[start_xy.0][start_xy.1] = 0;

        while queue.len() > 0 {
            let current_xy = queue.pop_front().unwrap();
            let current_node = grid[current_xy.0][current_xy.1].clone();
            if current_node.is_end_node() {
                break;
            }
            let dx: [i32; 4] = [0, 0, -1, 1];
            let dy: [i32; 4] = [-1, 1, 0, 0];
            for i in 0..4 {
                let nx = current_xy.0 as i32 + dx[i];
                let ny = current_xy.1 as i32 + dy[i];
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                    continue;
                }
                if visited[nx as usize][ny as usize] != -1 {
                    continue;
                }
                if current_node.can_move_to(&grid[nx as usize][ny as usize]) {
                    visited[nx as usize][ny as usize] = visited[current_xy.0][current_xy.1] + 1;
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
        steps.push(visited[end_xy.0][end_xy.1]);
    }
    println!(
        "{:?}",
        steps.into_iter().filter(|&x| x != -1).min().unwrap()
    );
}
