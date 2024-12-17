use std::{fmt::Display, path::Path, str::FromStr};

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView};
use common::{read_test_data, Error};
use strum_macros::{Display, EnumString, VariantArray};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day07/testdata.dat")).unwrap();
    // println!("Example Data: \n{}", data);

    let mut sum_solveable_equations: i64 = 0;
    for line in data.lines() {
        let eq = Equation::from_str(line).unwrap();
        if eq.is_solvable() {
            sum_solveable_equations += eq.expected_result;
        }
    }

    println!("Sum of solveable equations: {}", sum_solveable_equations);
    assert_eq!(sum_solveable_equations, 3351424677624);


    Ok(())
}

#[derive(Debug, Default)]
struct Equation {
    expected_result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn is_solvable(&self) -> bool {
        let combination_count = (2 as u64).pow(self.operator_count());
        for i in 0..combination_count {
            let operators = i.view_bits::<Lsb0>();
            let eq_result = self.solve(operators);
            if eq_result == self.expected_result {
                return true;
            }
            // println!("Result of equation: {}", eq_result);
        }
        false
    }

    fn operator_count(&self) -> u32 {
        (self.operands.len() - 1) as u32
    }

    fn solve(&self, operators: &BitSlice<u64>) -> i64 {
        let mut result = self.operands[0];
        for j in 0..self.operator_count() as usize {
            // print!("{} ", if operators[j] { Operator::Add } else { Operator::Mul });
            if operators[j] {
                result += self.operands[j + 1];
            } else {
                result *= self.operands[j + 1];
            }
        }
        result
    }
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut equation = Self::default();
        let mut parts = s.split(":");
        if parts.clone().count() != 2 {
            return Err(Error::SyntaxError("Generale form of equations is result: op1 op2 op3 ...".to_string()));
        }

        equation.expected_result = parts.next().unwrap().parse()?;
        let operands_str = parts.next().unwrap().trim();
        equation.operands = operands_str.split(" ").map(|n| n.parse().unwrap()).collect();

        Ok(equation)
    }
}

#[derive(Debug, VariantArray, EnumString, Display)]
enum Operator {
    #[strum(to_string = "+")]
    Add,
    #[strum(to_string = "*")]
    Mul,
}

impl Operator {
    fn count() -> u64 {
        2
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solvable() {
        let eq1 = Equation::from_str("190: 10 19").unwrap();
        assert!(eq1.is_solvable());
        let eq1 = Equation::from_str("3267: 81 40 27").unwrap();
        assert!(eq1.is_solvable());
        let eq1 = Equation::from_str("83: 17 5").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("156: 15 6").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("7290: 6 8 6 15").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("161011: 16 10 13").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("192: 17 8 14").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("21037: 9 7 18 13").unwrap();
        assert!(!eq1.is_solvable());
        let eq1 = Equation::from_str("292: 11 6 16 20").unwrap();
        assert!(eq1.is_solvable());
    }

    #[test]
    fn test_solveablefrominput() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();

        let mut solveable_equations: Vec<usize> = vec![];
        for (i, line) in data.lines().enumerate() {
            if Equation::from_str(line).unwrap().is_solvable() {
                solveable_equations.push(i);
            }
        }

        assert_eq!(solveable_equations, vec![0, 1, 8]);
    }

    #[test]
    fn test_frominput() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();
        let lines: Vec<&str> = data.lines().collect();
        let equation = Equation::from_str(lines[0]).unwrap();
        assert_eq!(equation.expected_result, 190);
        assert_eq!(equation.operands, vec![10, 19]);

        let equation = Equation::from_str(lines[8]).unwrap();
        assert_eq!(equation.expected_result, 292);
        assert_eq!(equation.operands, vec![11, 6, 16, 20]);
    }
}
