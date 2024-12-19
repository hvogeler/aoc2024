use std::{fmt::Display, path::Path, result, str::FromStr};

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView};
use common::{read_test_data, Error};
use strum_macros::{Display, EnumString, VariantArray};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day07/example.dat")).unwrap();
    // println!("Example Data: \n{}", data);

    let mut sum_solveable_equations: i64 = 0;
    let mut non_solveable_eq: Vec<Equation> = Vec::new(); // keep for part 2
    for line in data.lines() {
        let eq = Equation::from_str(line).unwrap();
        if eq.is_solvable() {
            sum_solveable_equations += eq.expected_result;
        } else {
            non_solveable_eq.push(eq);
        }
    }

    println!("Sum of solveable equations: {}", sum_solveable_equations);
    // assert_eq!(sum_solveable_equations, 3351424677624);

    // Part 2
    // add equation produced from concatenations to the list of non-solveable equiations
    let mut equations_concatted: Vec<Equation> = Vec::new();
    for eq in non_solveable_eq {
        let combination_count = (2 as u64).pow(eq.operator_count());
        for i in 0..combination_count as usize {
            let concat_eq = Equation {
                expected_result: eq.expected_result,
                operands: eq.concat_operands(i.view_bits::<Lsb0>()),
            };
            equations_concatted.push(concat_eq);
        }
    }

    for eq in equations_concatted {
        println!("Check: {:?}", eq);
        if eq.is_solvable() {
            println!("  Solveable: {:?}", eq);
            sum_solveable_equations += eq.expected_result;
        }
    }

    println!("Sum of solveable equations Part 2: {}", sum_solveable_equations);

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

    fn concat_operands(&self, operator_positions: &BitSlice<usize>) -> Vec<i64> {
        let mut result_ops: Vec<i64> = vec![self.operands[0]];
        let mut src_operand_idx = 1;
        let mut tgt_operand_idx = 0;
        for operator_idx in 0..self.operator_count() {
            if operator_positions[operator_idx as usize] {
                let op1 = result_ops[tgt_operand_idx];
                let op2 = self.operands[src_operand_idx];
                let mut s: String = op1.to_string();
                s += &op2.to_string();
                result_ops[tgt_operand_idx] = s.parse().unwrap();
            } else {
                result_ops.push(self.operands[src_operand_idx]);
                tgt_operand_idx += 1;
            }
            src_operand_idx += 1;
        }
        result_ops
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
    use bitvec::{bits, bitvec, vec};
    use convert_base::Convert;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base_convert() {
        let mut base = Convert::new(10, 3);
        let b3: Vec<u64> = base.convert(&vec![26u64]);
        println!("{:?}", b3);
    }

    #[test]
    fn test_concat_operands() {
        // let eq1 = Equation::from_str("190: 10 19").unwrap();
        // let xx = eq1.concat_operands(bitvec![1].as_bitslice());
        // assert_eq!(xx, vec![1019]);
        let eq1 = Equation::from_str("3267: 81 40 27").unwrap();
        let xx = eq1.concat_operands(bits![1,1]);
        assert_eq!(xx, vec![814027]);
        let xx = eq1.concat_operands(bits![1,0]);
        assert_eq!(xx, vec![8140, 27]);
        let xx = eq1.concat_operands(bits![0,1]);
        assert_eq!(xx, vec![81, 4027]);
        let eq1 = Equation::from_str("7290: 6 8 6 15").unwrap();
        let xx = eq1.concat_operands(bits![1,1,1]);
        assert_eq!(xx, vec![68615]);
        let xx = eq1.concat_operands(bits![0,1,1]);
        assert_eq!(xx, vec![6, 8615]);
        let xx = eq1.concat_operands(bits![0,0,1]);
        assert_eq!(xx, vec![6, 8, 615]);
        let xx = eq1.concat_operands(bits![0,1,0]);
        assert_eq!(xx, vec![6, 86, 15]);
        let xx = eq1.concat_operands(bits![1,1,0]);
        assert_eq!(xx, vec![686, 15]);
        let xx = eq1.concat_operands(bits![1,0,0]);
        assert_eq!(xx, vec![68, 6, 15]);
        let xx = eq1.concat_operands(bits![1,0,1]);
        assert_eq!(xx, vec![68, 615]);
    }

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
