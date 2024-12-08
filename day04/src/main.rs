use std::{fmt::Display, io::Cursor, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day04/testdata.dat"))?;
    let mut cgrid = CharGrid::from(data.as_str());
    let sum = cgrid.find_xmas();
    println!("Part 1: Count = {}", sum);
    let result_grid = cgrid.get_result_grid();
    println!("Result: \n{}", result_grid);
    assert_eq!(sum, 2534);

    // Part 2
    let sum = cgrid.find_xmas2();
    let result_grid = cgrid.get_result_grid();
    println!("Result: \n{}\n", result_grid);
    println!("Part 2: Count = {}", sum);
    assert_eq!(sum, 1866);

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
    fn find_xmas2(&mut self) -> i64 {
        self.xmas_coords.clear();
        let mut count = 0;
        for row in 1..(self.dimensions.row - 1) {
            for col in 1..(self.dimensions.col - 1) {
                if self.grid[row][col] == 'A' {
                    if self.is_mas_cross(Coord::new(row, col)) {
                        count += 1;
                        self.xmas_coords.push(Coord::new(row, col));
                        self.xmas_coords.push(Coord::new(row - 1, col - 1));
                        self.xmas_coords.push(Coord::new(row - 1, col + 1));
                        self.xmas_coords.push(Coord::new(row + 1, col - 1));
                        self.xmas_coords.push(Coord::new(row + 1, col + 1));
                    }
                }
            }
        }
        count
    }

    fn is_mas_cross(&mut self, at: Coord) -> bool {
        let mut chars = [' '; 4];
        chars[0] = self.grid[at.row - 1][at.col - 1];
        chars[1] = self.grid[at.row - 1][at.col + 1];
        chars[2] = self.grid[at.row + 1][at.col - 1];
        chars[3] = self.grid[at.row + 1][at.col + 1];

        let count_s = chars.iter().filter(|c| **c == 'S').count();
        let count_m = chars.iter().filter(|c| **c == 'M').count();

        count_m == 2 && count_s == 2 && chars != ['S', 'M', 'M' ,'S']  && chars != ['M', 'S', 'S' ,'M']
    }

    fn find_xmas(&mut self) -> i64 {
        let mut count = 0;
        for row in 0..self.dimensions.row {
            for col in 0..self.dimensions.col {
                if self.grid[row][col] == 'X' {
                    if self.check_right(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_left(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_up(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_down(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_up_right(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_down_right(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_up_left(Coord::new(row, col)) {
                        count += 1;
                    }
                    if self.check_down_left(Coord::new(row, col)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn check_right(&mut self, cursor: Coord) -> bool {
        if cursor.col < self.dimensions.col - 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row][cursor.col + i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row, cursor.col + i));
                }
                return true;
            }
        }
        false
    }

    fn check_left(&mut self, cursor: Coord) -> bool {
        if cursor.col >= 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row][cursor.col - i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row, cursor.col - i));
                }
                return true;
            }
        }
        false
    }

    fn check_up(&mut self, cursor: Coord) -> bool {
        if cursor.row >= 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row - i][cursor.col];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row - i, cursor.col));
                }
                return true;
            }
        }
        false
    }

    fn check_down(&mut self, cursor: Coord) -> bool {
        if cursor.row < self.dimensions.row - 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row + i][cursor.col];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row + i, cursor.col));
                }
                return true;
            }
        }
        false
    }

    fn check_up_right(&mut self, cursor: Coord) -> bool {
        if cursor.col < self.dimensions.col - 3 && cursor.row >= 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row - i][cursor.col + i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row - i, cursor.col + i));
                }
                return true;
            }
        }
        false
    }

    fn check_down_right(&mut self, cursor: Coord) -> bool {
        if cursor.col < self.dimensions.col - 3 && cursor.row < self.dimensions.row - 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row + i][cursor.col + i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row + i, cursor.col + i));
                }
                return true;
            }
        }
        false
    }

    fn check_up_left(&mut self, cursor: Coord) -> bool {
        if cursor.col >= 3 && cursor.row >= 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row - i][cursor.col - i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row - i, cursor.col - i));
                }
                return true;
            }
        }
        false
    }

    fn check_down_left(&mut self, cursor: Coord) -> bool {
        if cursor.col >= 3 && cursor.row < self.dimensions.row - 3 {
            let v = &mut [' '; 4];
            for i in 0..4 {
                v[i] = self.grid[cursor.row + i][cursor.col - i];
            }
            if *v == ['X', 'M', 'A', 'S'] {
                for i in 0..4 {
                    self.xmas_coords
                        .push(Coord::new(cursor.row + i, cursor.col - i));
                }
                return true;
            }
        }
        false
    }

    fn get_result_grid(&self) -> ResultGrid {
        let mut grid: Vec<Vec<char>> = (0..self.dimensions.row)
            .map(|_| (0..self.dimensions.col).map(|_| '.').collect())
            .collect();
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
        cgrid.dimensions = Coord {
            row: rows,
            col: cgrid.grid[0].len(),
        };
        cgrid
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
