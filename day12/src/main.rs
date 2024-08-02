use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Start,
    End,
    Square(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(v) => v,
        }
    }
}

struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

struct CellRecord {
    prev: Option<GridCoord>,
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    visited: HashMap<GridCoord, CellRecord>,
    current: HashMap<GridCoord, CellRecord>,
    num_steps: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut data = vec![];

        for c in input.chars() {
            let cell = match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Square(c as u8 - 'a' as u8),
                '\n' => continue,
                _ => panic!("Not valid"),
            };
            data.push(cell);
        }

        Self {
            width,
            height,
            data,
            current: Default::default(),
            visited: Default::default(),
            num_steps: 0,
        }
    }

    fn in_bounds(&self, coord: &GridCoord) -> bool {
        return coord.x < self.height && coord.y < self.width;
    }

    fn cell(&self, coord: &GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        let idx = coord.x * self.width + coord.y;
        Some(&self.data[idx])
    }

    fn cell_mut(&mut self, coord: &GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        let idx = coord.x * self.width + coord.y;
        Some(&mut self.data[idx])
    }

    fn reachable_neighbours<'a>(
        &'a self,
        coord: &'a GridCoord,
    ) -> impl Iterator<Item = GridCoord> + 'a {
        let current_elev = self.cell(coord).unwrap().elevation();
        let delta: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        delta.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|coord| self.in_bounds(coord))
            .filter(|coord| self.cell(coord).unwrap().elevation() <= current_elev + 1)
        })
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}x{} grid", self.height, self.width)?;
        for i in 0..self.height {
            for j in 0..self.width {
                let cell = self.cell(&(i, j).into()).unwrap();
                match cell {
                    Cell::Start => write!(fmt, "S")?,
                    Cell::End => write!(fmt, "E")?,
                    Cell::Square(c) => write!(fmt, "{}", (b'a' + c) as char)?,
                };
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("test.txt");
    let grid = Grid::parse(input);
    println!("{:?}", grid.data);
    print!("{:?}", grid);
}
