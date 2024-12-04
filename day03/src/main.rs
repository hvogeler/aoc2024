use std::{path::Path, str::FromStr};

use common::{read_test_data, Error};
use regex::Regex;

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day03/testdata.dat"))?;
    println!("Example data: {}", data);
    let muls = parse(&data);
    let mut sum = 0;
    for mul in muls {
        println!("{} = {}", mul.token, mul.eval());
        sum += mul.eval();
    }
    println!("Sum of products: {}", sum);
    Ok(())
}

#[derive(Debug)]
struct Mul {
    token: String,
}

impl Mul {
    pub fn eval(&self) -> i64 {
        let strip_oper = self.token.clone().split_off(4);
        let strip_closing_parens = strip_oper.strip_suffix(")").unwrap();
        let operands: Vec<i64> = strip_closing_parens
            .split(",")
            .map(|op| op.parse::<i64>().unwrap())
            .collect();
        operands[0] * operands[1]
    }
}

pub fn parse(s: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let muls: Vec<&str> = re.find_iter(s).map(|m| m.as_str()).collect();

    muls.into_iter()
        .map(|m| Mul {
            token: String::from_str(m).unwrap(),
        })
        .collect()
}
