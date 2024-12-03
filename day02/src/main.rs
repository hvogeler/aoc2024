use std::{default, path::Path};

use common::{read_test_data, Error};

fn main()  -> Result<(), Error> {
    let data = read_test_data(Path::new("./day02/testdata.dat"))?;
    println!("Example data: {}", data);
    let mut safe_count = 0;
    for line in data.lines() {
        if Report::from(line).is_safe() {
            safe_count += 1;
        }
    }
    println!("Safe reports: {}", safe_count);
    Ok(())
}

#[derive(Debug, Default)]
pub struct Report {
    levels: Vec<i64>,
    direction: Direction,
}

impl From<&str> for Report {
    fn from(levels_str: &str) -> Self {
        let levels: Vec<i64> = levels_str.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        let direction = Direction::from(&Pair(levels[0], levels[1]));

        Self {
            levels,
            direction,
        }
    }
}


impl Report {
    pub fn is_safe(&self) -> bool {
        for i in 0..self.levels.len() - 1 {
            let pair = Pair(self.levels[i], self.levels[i+1]);
            let direction = Direction::from(&pair);
            if direction != self.direction || pair.diff() > 3 {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct Pair(i64, i64);

impl Pair {
    pub fn diff(&self) -> i64 {
        (self.0 - self.1).abs()
    }
}

#[derive(Debug, Default, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    #[default]
    NotStrictlyMonotonic,
}

impl From<&Pair> for Direction {
    fn from(pair: &Pair) -> Self {
        if pair.0 < pair.1 {
            return Direction::Increasing;
        }
        if pair.0 > pair.1 {
            return Direction::Decreasing;
        }
        Direction::NotStrictlyMonotonic
    }
}