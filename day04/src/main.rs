use std::{fmt::Display, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day04/example.dat"))?;
    let mut cgrid = CharGrid::from(data.as_str());
    let sum = cgrid.find_xmas();
    println!("Example: sum = {}", sum);
    let result_grid = cgrid.get_result_grid();
    println!("Result: \n{}", result_grid);
    Ok(())
}

#[derive(Debug, Default)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }
}

#[derive(Debug, Default)]
struct CharGrid {
    grid: Vec<Vec<char>>,
    dimensions: Coord,
    xmas_coords: Vec<Coord>,
}

impl CharGrid {
    fn find_xmas(&mut self) -> i64 {
        // let mut cursor = Coord::default(); // (row, col)
        let mut sum = 0;
        for row in 0..self.dimensions.row {
            for col in 0..self.dimensions.col {
                if self.check_right(Coord::new(row, col)) {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn check_right(&mut self, cursor: Coord) -> bool {
        if cursor.col < self.dimensions.col - 4 {
            if &self.grid[cursor.row][cursor.col..cursor.col + 4] == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords.push(Coord::new(cursor.row, cursor.col + i));
                }
                return true;
            }
        }
        false
    }
    
    fn get_result_grid(&self) -> ResultGrid {
        let mut grid: Vec<Vec<char>> = (0..self.dimensions.row).map(|_| (0..self.dimensions.col).map(|_| '.').collect()).collect();
        for xmas_coord in self.xmas_coords.iter() {
            grid[xmas_coord.row][xmas_coord.col] = self.grid[xmas_coord.row][xmas_coord.col];
        }

        ResultGrid::new(grid)
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
        cgrid.dimensions = Coord { row: rows, col: cgrid.grid[0].len()};
        cgrid
    }
}


struct ResultGrid {
    grid: Vec<Vec<char>>,
}

impl ResultGrid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            grid
        }
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