use std::{collections::HashMap, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day05/example.dat"))?;
    Ok(())
}

#[derive(Debug, Default)]
struct PageUpdate {
    pages: Vec<i64>,
}

impl PageUpdate {
    fn from_csv(csv_line: &str) -> Self {
        PageUpdate {
            pages: csv_line.split(",").map(|p| p.parse().unwrap()).collect(),
        } 
    }
}


#[derive(Debug, Default)]
struct OrderRules {
    rule_map: HashMap<i64, Vec<i64>>,
}

impl OrderRules {
    fn new() -> Self {
        OrderRules::default()
    }

    fn from_input(data: &str) -> Self {
        let mut rules = OrderRules::new();
        for line in data.lines() {
            if line.is_empty() {
                break;
            }
            rules.add(line);
        }
        rules
    }

    fn add(&mut self, rule: &str) {
        let mut parts = rule.split("|");
        let page_no: i64 = parts.next().unwrap().parse().unwrap();
        let following_page_no: i64 = parts.next().unwrap().parse().unwrap();
        if self.rule_map.contains_key(&page_no) {
            let rule = self.rule_map.get_mut(&page_no).unwrap();
            rule.push(following_page_no);
        } else {
            self.rule_map.insert(page_no, vec![following_page_no]);
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_frominput() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let r = OrderRules::from_input(&data);
        println!("{:?}", r);
        assert_eq!(r.rule_map[&97], vec![13, 61, 47, 29, 53, 75]);
        assert_eq!(r.rule_map.len(), 6);
    }

    #[test]
    fn test_pageupdate_from_csv_line() {
        let pu = PageUpdate::from_csv("75,47,61,53,29");
        assert_eq!(pu.pages.len(), 5);
        assert_eq!(pu.pages, vec![75,47,61,53,29]);
    }
}