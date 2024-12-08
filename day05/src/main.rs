use std::{cmp::Ordering, collections::{HashMap, HashSet}, path::Path};

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day05/testdata.dat"))?;
    let rules = OrderRules::from_input(&data);
    let page_updates = PageUpdate::page_updates_from(&data, &rules);

    let mut incorrect_updates: Vec<PageUpdate> = Vec::new();
    let mut sum_middle_numbers = 0;
    for update in page_updates {
        if update.is_correctly_ordered() {
            sum_middle_numbers += update.get_middle_page_no();
        } else {
            incorrect_updates.push(update);
        }
    }
    println!("Part 1: Sum of middle page numbers: {}", sum_middle_numbers);
    assert_eq!(sum_middle_numbers, 5108);

    // Part 2

    let mut sum_middle_numbers = 0;
    for page_update in incorrect_updates.iter_mut() {
        page_update.pages.sort();
        sum_middle_numbers += page_update.get_middle_page_no();
    }
    println!("Part 2: Sum of middle page numbers: {}", sum_middle_numbers);
    assert_eq!(sum_middle_numbers, 7380);

    Ok(())
}

#[derive(Debug)]
struct Page<'a> {
    page_no: i64,
    order_rules: &'a OrderRules,
}

impl<'a> PartialEq for Page<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.page_no == other.page_no
    }
}

impl<'a> Eq for Page<'a> {}

impl<'a> PartialOrd for Page<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(ref followers) = self.order_rules.rule_map.get(&self.page_no) {
            if followers.contains(&other.page_no) {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        }
        panic!("Order rules are incomplete")
    }
}

impl<'a> Ord for Page<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Page<'a> {
    fn new(page_no: i64, order_rules: &'a OrderRules) -> Self {
        Self {
            page_no,
            order_rules,
        }
    }
}

#[derive(Debug, Default)]
struct PageUpdate<'a> {
    pages: Vec<Page<'a>>,
}

impl<'a> PageUpdate<'a> {
    fn from_csv(csv_line: &str, order_rules: &'a OrderRules) -> Self {
        PageUpdate {
            pages: csv_line
                .split(",")
                .map(|p| Page::new(p.parse().unwrap(), order_rules))
                .collect(),
        }
    }

    fn is_correctly_ordered(&self) -> bool {
        for i in 0..self.pages.iter().len() {
            for j in (i + 1)..self.pages.len() {
                if self.pages[i] > self.pages[j] {
                    return false;
                }
            }
        }
        true
    }

    fn page_updates_from(data: &str, order_rules: &'a OrderRules) -> Vec<Self> {
        let mut in_page_updates = false;
        let mut updates: Vec<Self> = Vec::new();
        for line in data.lines() {
            if line.is_empty() {
                in_page_updates = true;
                continue;
            }
            if in_page_updates {
                updates.push(PageUpdate::from_csv(line, order_rules));
            }
        }
        updates
    }

    fn get_middle_page_no(&self) -> i64 {
        if self.pages.len() % 2 == 0 {
            panic!("Uneven number of pages expected");
        }
        self.pages[self.pages.len() / 2].page_no
    }
}

#[derive(Debug, Default)]
struct OrderRules {
    rule_map: HashMap<i64, HashSet<i64>>,
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
            rule.insert(following_page_no);
        } else {
            let mut set_greated_page_nos = HashSet::new();
            set_greated_page_nos.insert(following_page_no);
            self.rule_map.insert(page_no, set_greated_page_nos);
        }
        if !self.rule_map.contains_key(&following_page_no) {
            self.rule_map.insert(following_page_no, HashSet::new());
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
        assert_eq!(r.rule_map[&97], HashSet::from_iter(vec![13, 61, 47, 29, 53, 75]));
        assert_eq!(r.rule_map.len(), 7);
    }

    #[test]
    fn test_pageupdate_from_csv_line() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let rules = OrderRules::from_input(&data);
        let pu = PageUpdate::from_csv("75,47,61,53,29", &rules);
        assert_eq!(pu.pages.len(), 5);
        assert_eq!(
            pu.pages
                .iter()
                .map(|p| p.page_no as i32)
                .collect::<Vec<i32>>(),
            vec![75, 47, 61, 53, 29]
        );
    }

    #[test]
    fn test_get_middle_pageno() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let rules = OrderRules::from_input(&data);
        let pu = PageUpdate::from_csv("75,47,61,53,29", &rules);
        assert_eq!(pu.get_middle_page_no(), 61);
        let pu = PageUpdate::from_csv("75,29,13", &rules);
        assert_eq!(pu.get_middle_page_no(), 29);
    }

    #[test]
    #[should_panic]
    fn test_even_pages() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let rules = OrderRules::from_input(&data);
        let pu = PageUpdate::from_csv("75,29,13,2", &rules);
        pu.get_middle_page_no();
    }

    #[test]
    fn test_pageupdate_from_data() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let rules = OrderRules::from_input(&data);
        let pus = PageUpdate::page_updates_from(&data, &rules);
        assert_eq!(pus.len(), 6);
        assert_eq!(
            pus[0]
                .pages
                .iter()
                .map(|p| p.page_no as i32)
                .collect::<Vec<i32>>(),
            vec![75, 47, 61, 53, 29]
        );
    }

    #[test]
    fn test_pageupdate_correctly_ordered() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let rules = OrderRules::from_input(&data);
        let pu = PageUpdate::from_csv("75,47,61,53,29", &rules);
        assert!(pu.is_correctly_ordered());
        let pu = PageUpdate::from_csv("75,97,47,61,53", &rules);
        assert!(!pu.is_correctly_ordered());
        let pu = PageUpdate::from_csv("61,13,29", &rules);
        assert!(!pu.is_correctly_ordered());
        let pu = PageUpdate::from_csv("97,13,75,29,47", &rules);
        assert!(!pu.is_correctly_ordered());
    }
}
