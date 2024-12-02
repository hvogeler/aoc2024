use std::{collections::HashMap, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let example_data = read_test_data(Path::new("./day01/example.dat"))?;
    println!("Example data: \n{}", example_data);
    println!("Sum of differences (example) = {}", sumabs(&example_data));

    let test_data = read_test_data(Path::new("./day01/testdata.dat"))?;
    println!("Sum of differences = {}", sumabs(&test_data));

    // -------------- Part 2 --------------
    let example_data = read_test_data(Path::new("./day01/example.dat"))?;
    println!("Sim Score (example) = {}", sim_score(&example_data));
    let test_data = read_test_data(Path::new("./day01/testdata.dat"))?;
    println!("Sim Score = {}", sim_score(&test_data));


    Ok(())
}

fn sumabs(data: &str) -> i64 {
    let mut sum: i64 = 0;
    let lists = make2lists(data);
    for (i, v) in lists.0.iter().enumerate() {
        sum += (v - lists.1[i]).abs();
    }
    sum
}

fn sim_score(data: &str) -> i64 {
    let lists = make2lists(data);
    let map = makeMapWithCounts(lists.1);
    let mut sum = 0;
    for v in lists.0 {
        sum += map.get(&v).unwrap_or(&0) * v;
    }
    sum
}

fn makeMapWithCounts(list: Vec<i64>) -> HashMap<i64, i64> {
    let mut map = HashMap::new();
    for v in list {
        if map.contains_key(&v) {
            let new_count = map[&v] + 1;
            map.insert(v, new_count);
        } else {
            map.insert(v, 1);
        }
    }
    map
}

fn make2lists(data: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in data.lines() {
        let parts: Vec<i64> = line.split("   ").map(|v| v.parse::<i64>().unwrap()).collect();
        list1.push(parts[0]);
        list2.push(parts[1]);
    }

    list1.sort();
    list2.sort();
    (list1, list2)
}
