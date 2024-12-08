use std::{collections::HashSet, fmt::Display, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day06/testdata.dat")).unwrap();
    let mut grid = CharGrid::from(data.as_str());
    println!("grid: \n{}", grid);
    grid.get_objects();
    let mut walker = Walker::new(&grid);
    let mut position = walker.walk();
    while position == Position::OnGrid {
        position = walker.walk();
    }
    println!(
        "Number of positions visited: {}",
        walker.positions_visited.len()
    );
    let result_grid = walker.get_result_grid();
    println!("Result: \n{}", result_grid);
    Ok(())
}

const OBSTACLE: &'static char = &'#';
const GUARD: &'static char = &'^';

#[derive(Debug)]
struct Walker<'a> {
    char_grid: &'a CharGrid,
    guard_current_position: Coord,
    direction: Direction,
    positions_visited: HashSet<Coord>,
}

impl<'a> Walker<'a> {
    fn new(char_grid: &'a CharGrid) -> Self {
        Walker {
            char_grid,
            guard_current_position: char_grid.guard.clone(),
            direction: Direction::Up,
            positions_visited: HashSet::new(),
        }
    }

    fn walk(&mut self) -> Position {
        let mut next_pos = self.guard_current_position.clone();
        match self.direction {
            Direction::Up => {
                if next_pos.row == 0 {
                    return Position::OffGrid;
                }
                next_pos.row -= 1;
            }
            Direction::Down => next_pos.row += 1,
            Direction::Left => {
                if next_pos.col == 0 {
                    return Position::OffGrid;
                }
                next_pos.col -= 1;
            }
            Direction::Right => next_pos.col += 1,
        };
        if next_pos.row >= self.char_grid.dimensions.row
            || next_pos.col >= self.char_grid.dimensions.col
        {
            self.guard_current_position = next_pos;
            return Position::OffGrid;
        }
        if self.char_grid.at(&next_pos) == *OBSTACLE {
            self.turn();
        } else {
            self.guard_current_position = next_pos.clone();
            self.positions_visited.insert(next_pos);
        }
        Position::OnGrid
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn get_result_grid(&self) -> ResultGrid {
        let mut grid: Vec<Vec<char>> = (0..self.char_grid.dimensions.row)
            .map(|_| (0..self.char_grid.dimensions.col).map(|_| '.').collect())
            .collect();
        for position_visited in self.positions_visited.iter() {
            grid[position_visited.row][position_visited.col] = 'X';
        }

        ResultGrid::new(grid)
    }
}

#[derive(Debug, Default)]
struct CharGrid {
    grid: Vec<Vec<char>>,
    dimensions: Coord,
    obstacles: HashSet<Coord>,
    guard: Coord,
}

impl CharGrid {
    fn get_objects(&mut self) {
        for (i, row) in self.grid.iter().enumerate() {
            for j in 0..row.len() {
                if self.grid[i][j] == *OBSTACLE {
                    self.obstacles.insert(Coord::new(i, j));
                }
                if self.grid[i][j] == *GUARD {
                    self.guard = Coord::new(i, j);
                }
            }
        }
    }

    fn is_obstacle(&self, coord: &Coord) -> bool {
        self.obstacles.contains(coord)
    }

    fn at(&self, coord: &Coord) -> char {
        self.grid[coord.row][coord.col]
    }
}

impl From<&str> for CharGrid {
    fn from(data: &str) -> Self {
        let mut rows = 0;
        let mut cgrid = Self {
            ..Default::default()
        };
        for line in data.lines() {
            let row: Vec<char> = line.chars().collect();
            cgrid.grid.push(row);
            rows += 1;
        }
        cgrid.dimensions = Coord {
            row: rows,
            col: cgrid.grid[0].len(),
        };
        cgrid
    }
}

impl Display for CharGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.grid.iter() {
            let row_str: String = row.into_iter().collect::<String>() + "\n";
            s = s + &row_str;
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
enum Position {
    OnGrid,
    OffGrid,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {}

#[derive(Debug, Default, Eq, Hash, PartialEq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }
}
struct ResultGrid {
    grid: Vec<Vec<char>>,
}

impl ResultGrid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }
}

impl Display for ResultGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.grid.iter() {
            let row_str: String = row.into_iter().collect::<String>() + "\n";
            s = s + &row_str;
        }
        write!(f, "{}", s)
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_frominput() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut grid = CharGrid::from(data.as_str());
        grid.get_objects();
        assert_eq!(grid.guard, Coord::new(6, 4));
        assert_eq!(grid.obstacles.len(), 8);
        assert!(grid.obstacles.contains(&Coord::new(8, 0)));
    }
}