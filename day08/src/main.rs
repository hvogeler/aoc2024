use common::{read_test_data, Error};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    iter::{FlatMap, Map},
    path::Path,
    str::FromStr,
    string::FromUtf8Error,
};

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
    lines_by_antennas: HashMap<Antenna, Vec<GeoLine>>,
    antinodes_by_line: HashMap<GeoLine, Vec<Location>>,
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

    fn create_lines(&mut self) {
        self.lines_by_antennas.clear();
        for freq in self.ants_by_frequency.keys() {
            let antennas = self.ants_by_frequency.get(freq).unwrap();
            for i in 0..antennas.len() - 1 {
                let mut lines: Vec<GeoLine> = Vec::new();
                for j in (i + 1)..antennas.len() {
                    let geo_line = GeoLine::new(antennas[i].clone(), antennas[j].clone());
                    lines.push(geo_line);
                }
                self.lines_by_antennas.insert(antennas[i].clone(), lines);
            }
        }
    }

    fn create_antinodes(&mut self) {
        // let antenna1 = &self.antennas[0];
        // let line1 = &self.lines_by_antennas.get(&antenna1).unwrap()[0];
        for line in self.lines_by_antennas.values().flat_map(|it| it) {
            let d = line.distance();
            println!("{:?}, distance: {}", line, d);
            let mut antinodes: Vec<Location> = Vec::new();
            antinodes.push(Location::new(line.a.location.row - (d.rows as i64), line.a.location.col - (d.cols as i64)));
            antinodes.push(Location::new(line.a.location.row + (d.rows as i64), line.a.location.col + (d.cols as i64)));
            antinodes.push(Location::new(line.b.location.row - (d.rows as i64), line.b.location.col - (d.cols as i64)));
            antinodes.push(Location::new(line.b.location.row + (d.rows as i64), line.b.location.col + (d.cols as i64)));
            let antinodes: Vec<Location> = antinodes
                .into_iter()
                .filter(|node| *node != line.a.location && *node != line.b.location)
                .collect();
            println!("Antinode canidates: {:?}\n\n", antinodes);
            self.antinodes_by_line.insert(line.clone(), antinodes.clone());
        }
    }

    // fn antinodes_to_str(&self) -> String {
    //     for row in 0..self.dimensions.rows {
    //         let mut 
    //         for col in 0..self.dimensions.cols {

    //         }
    //     }
    //     "".to_string()
    // }

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct GeoLine {
    a: Antenna,
    b: Antenna,
}

impl GeoLine {
    fn new(a: Antenna, b: Antenna) -> Self {
        Self { a, b }
    }

    fn distance(&self) -> Distance {
        Distance {
            rows: self.a.location.row - self.b.location.row,
            cols: self.a.location.col - self.b.location.col,
        }
        // (((self.a.location.row - self.b.location.row).pow(2) + (self.a.location.col - self.b.location.col).pow(2)) as f64).sqrt()
    }

    fn get_line_eq(&self) -> LineEq {
        let m: f64 = f64::from((self.a.location.row - self.b.location.row) as f32 / (self.a.location.col - self.b.location.col) as f32);
        let b = self.a.location.row as f64 - m * self.a.location.col as f64;
        LineEq { m, b }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct LineEq {
    m: f64,
    b: f64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Location {
    row: i64,
    col: i64,
}

impl Location {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Distance {
    rows: i64,
    cols: i64,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rows: {}, cols: {}", self.rows, self.cols)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
struct MapDimensions {
    rows: usize,
    cols: usize,
}

impl Display for MapDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rows: {}, cols: {}", self.rows, self.cols)
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_antinodes() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut city_map = CityMap::from_str(&data).unwrap();
        city_map.create_lines();
        city_map.create_antinodes();
        
    }

    #[test]
    fn test_lines() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut city_map = CityMap::from_str(&data).unwrap();
        city_map.create_lines();
        // println!("{:#?}", city_map.lines_by_antennas);
        let line_a = city_map.lines_by_antennas.get(&Antenna::new('0', 3, 7)).unwrap();
        assert_eq!(line_a.len(), 1);
        assert_eq!(line_a[0].b, Antenna::new('0', 4, 4));
        let line_a = city_map.lines_by_antennas.get(&Antenna::new('0', 1, 8)).unwrap();
        assert_eq!(line_a.len(), 3);
        assert_eq!(line_a[1].b, Antenna::new('0', 3, 7));
        assert_eq!(line_a[1].get_line_eq(), LineEq { m: -2.0, b: 17.0 });
        let line_a = city_map.lines_by_antennas.get(&Antenna::new('0', 2, 5)).unwrap();
        assert_eq!(line_a.len(), 2);
        assert_eq!(line_a[0].b, Antenna::new('0', 3, 7));
        assert_eq!(line_a[0].get_line_eq(), LineEq { m: 0.5, b: -0.5 });
        // assert_eq!(city_map.lines_by_antennas.len(), 10);
    }

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
