use std::{default, path::Path, str::FromStr};

use common::{read_test_data, Error};
use regex::Regex;

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day03/example2.dat"))?;
    // println!("Example data: {}", data);
    let muls = parse(&data);
    let mut sum = 0;
    for mul in muls {
        // println!("{} = {}", mul.token, mul.eval());
        sum += mul.eval();
    }
    println!("Sum of products: {}", sum);

    // part 2

    let mut state = ParseState::default();
    let mut enabled_string: String = String::new();
    let mut buf: Vec<char> = Vec::new();
    let chars: Vec<char> = data.chars().collect();
    let mut cursor = 0;
    while cursor < chars.len() {
        if cursor<chars.len() - 4 && chars[cursor..cursor+4] == ['d', 'o', '(', ')'] {
            println!("do() at {}", cursor);
            state = ParseState::Enabled;
            cursor += 4;
        }
        if cursor<chars.len() - 7 && chars[cursor..cursor+7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            println!("don't()  at {}", cursor);
            state = ParseState::Disabled;
            cursor += 7;
        }
        if state == ParseState::Enabled {
            buf.push(chars[cursor]);
        }
        cursor += 1;
    }
    enabled_string = buf.into_iter().collect();
    println!("enabled_string: {}", enabled_string);
    Ok(())
}

#[derive(Debug, Default, PartialEq)]
enum ParseState {
    #[default]
    Enabled,
    Disabled,
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
