use common::{read_test_data, Error};
use std::{collections::HashMap, hash::Hash, path::Path, str::FromStr, string::FromUtf8Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day08/example.dat")).unwrap();
    println!("Example Data: \n{}", data);

    println!("Hello, world!");
    Ok(())
}

const EMPTY_SPOT: char = '.';
const ANTINODE: char = '#';

#[derive(Debug, Default, Clone)]
struct CityMap {
    antennas: Vec<Antenna>,
    ants_by_frequency: HashMap<char, Vec<Antenna>>,
    dimensions: MapDimensions,
}

impl CityMap {
    fn new() -> Self {
        CityMap::default()
    }

    fn add_antenna(&mut self, antenna: Antenna) {
        self.antennas.push(antenna.clone());
        if self.ants_by_frequency.contains_key(&antenna.frequency) {
            self.ants_by_frequency.get_mut(&antenna.frequency).unwrap().push(antenna.clone());
        } else {
            self.ants_by_frequency.insert(antenna.frequency, vec![antenna]);
        }
    }

    fn init_cols_dimension(&mut self, cols: usize) {
        if cols != self.dimensions.cols {
            if self.dimensions.cols == 0 {
                self.dimensions.cols = cols;
            } else {
                panic!("All rows must have the same number of columns in the city map");
            }
        }
    }
}

impl FromStr for CityMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut city_map = CityMap::new();
        for (row, line) in s.lines().enumerate() {
            city_map.init_cols_dimension(line.len());
            let spots = line.chars();
            for (col, spot) in spots.enumerate() {
                if spot != EMPTY_SPOT && spot != ANTINODE {
                    let antenna = Antenna::new(spot, row as i64, col as i64);
                    city_map.add_antenna(antenna);
                }
            }
        }

        Ok(city_map)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Antenna {
    frequency: char,
    location: Location,
}

impl Antenna {
    fn new(frequency: char, row: i64, col: i64) -> Self {
        Antenna {
            frequency,
            location: Location::new(row, col),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct GeoLine {
    m: f64,
    b: f64,
}

impl GeoLine {
    fn from_two_locations(loc1: &Location, loc2: &Location) -> Self {
        let m: f64 = f64::from((loc1.row - loc2.row) as f32 / (loc1.col - loc2.col) as f32);
        let b = loc1.row as f64 - m * loc1.col as f64;
        GeoLine { m, b }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Location {
    row: i64,
    col: i64,
}

impl Location {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct MapDimensions {
    rows: usize,
    cols: usize,
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_city_map() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let city_map = CityMap::from_str(&data).unwrap();
        assert_eq!(city_map.antennas.len(), 7);
        assert_eq!(city_map.ants_by_frequency.len(), 2);
        assert_eq!(city_map.ants_by_frequency.get(&'A').unwrap().len(), 3);
        assert_eq!(city_map.ants_by_frequency.get(&'0').unwrap().len(), 4);
    }
}
