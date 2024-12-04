use std::path::Path;

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day02/testdata.dat"))?;
    // println!("Example data: {}", data);
    let mut safe_count = 0;
    for line in data.lines() {
        let report = Report::from(line);
        if report.is_safe() {
            safe_count += 1;
        }
    }
    println!("Safe reports: {}", safe_count);
    // assert_eq!(safe_count, 306);

    // --------- Part 2 -------------
    let mut safe_count = 0;
    for (i, line) in data.lines().enumerate() {
        let report = Report::from(line);
        if report.is_safe_dampened() {
            safe_count += 1;
        }
    }
    println!("Safe reports dampened: {}", safe_count);
    Ok(())
}

#[derive(Debug, Default)]
pub struct Report {
    levels: Vec<i64>,
    direction: Direction,
}

impl From<&str> for Report {
    fn from(levels_str: &str) -> Self {
        let levels: Vec<i64> = levels_str
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let direction = Direction::from(&Pair(levels[0], levels[1]));

        Self { levels, direction }
    }
}

impl Report {
    pub fn new(levels: Vec<i64>) -> Self {
        let direction = Direction::from(&Pair(levels[0], levels[1]));
        Self { levels, direction }
    }

    pub fn is_safe_dampened(&self) -> bool {
        let is_safe = self.is_safe();
        if !is_safe {
            for damp_index in 0..self.levels.len() {
                let dampened_report = Report::new(self.make_dampened_levels(damp_index));
                if dampened_report.is_safe() {
                    println!("dampened safe: {:?}", dampened_report.levels);
                    return true;
                }
            }
        }
        is_safe
    }

    fn make_dampened_levels(&self, damp_index: usize) -> Vec<i64> {
        let mut new_list = Vec::new();
        for (i, level) in self.levels.iter().enumerate() {
            if i != damp_index {
                new_list.push(*level);
            }
        }
        new_list
    }

    pub fn is_safe(&self) -> bool {
        for i in 0..self.levels.len() - 1 {
            let pair = Pair(self.levels[i], self.levels[i + 1]);
            let direction = Direction::from(&pair);
            if direction != self.direction || pair.diff() > 3 || pair.diff() < 1 {
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
