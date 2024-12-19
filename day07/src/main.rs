use common::{read_test_data, Error};
use convert_base::Convert;
use std::{
    io::{self, Write},
    path::Path,
    str::FromStr,
    time::Instant,
};
use strum_macros::{Display, EnumString, VariantArray};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day07/testdata.dat")).unwrap();
    // println!("Example Data: \n{}", data);

    let mut sum_solveable_equations: i64 = 0;
    let mut non_solveable_eq: Vec<Equation> = Vec::new(); // keep for part 2
    for line in data.lines() {
        let eq = Equation::from_str(line).unwrap();
        if eq.is_solvable(&[Operator::Add, Operator::Mul]) {
            sum_solveable_equations += eq.expected_result;
        } else {
            non_solveable_eq.push(eq);
        }
    }

    println!("Sum of solveable equations: {}", sum_solveable_equations);
    assert_eq!(sum_solveable_equations, 3_351_424_677_624);

    // Part 2
    // add equation produced from concatenations to the list of non-solveable equiations
    let now = Instant::now();
    let mut current_percent = 0;
    for (i, eq) in non_solveable_eq.iter().enumerate() {
        if eq.is_solvable(&[Operator::Add, Operator::Mul, Operator::Concat]) {
            sum_solveable_equations += eq.expected_result;
        }
        let percent = i * 100 / non_solveable_eq.len();
        if percent % 10 == 0 && percent != current_percent {
            print!("{}%..", percent);
            io::stdout().flush().unwrap();
            current_percent = percent;
        }
    }
    let duration = now.elapsed();
    println!("\nSum of solveable equations Part 2: {}", sum_solveable_equations);
    println!("Duration: {} seconds", duration.as_secs());
    assert_eq!(sum_solveable_equations, 204_976_636_995_111);

    Ok(())
}

#[derive(Debug, Default)]
struct Equation {
    expected_result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        let mut base = Convert::new(10, operators.len() as u64);

        // Calculate all operator combinations. Depending on the number of operators and the number of operands.
        // Example: (+, +) (+, -) (-, +) (-, -) for the operatiors + and - used in an equation with 3 operands (using 2 operators).
        // Example: (+, +, +) .. (-, -, -) would be 8 (2^3) for 2 operators and 4 operands
        // Example: (+, +) (+, -) (+, ||) (-, +) (-, -) (-, ||) (||, +) (||, -) ( ||, ||) 
        //          which would be 9 (3^2) for 3 operators used in an equation with 3 operands.
        // Check solveablity for each of the operator combinations.
        let combination_count = (operators.len() as u64).pow(self.operator_count());
        for i in 0..combination_count {
            let mut b3: Vec<u64> = base.convert(&vec![i]);
            for _ in b3.len()..self.operator_count() as usize {
                b3.push(0);
            }
            let operators: Vec<Operator> = b3.iter().map(|n| operators[*n as usize].clone()).collect();
            let eq_result = self.solve(&operators);
            if eq_result == self.expected_result {
                return true;
            }
        }
        false
    }

    fn operator_count(&self) -> u32 {
        (self.operands.len() - 1) as u32
    }

    fn solve(&self, operators: &Vec<Operator>) -> i64 {
        let mut result = self.operands[0];
        for j in 0..self.operator_count() as usize {
            match operators[j] {
                Operator::Add => result += self.operands[j + 1],
                Operator::Mul => result *= self.operands[j + 1],
                Operator::Concat => result = Self::concat_2_ints(&result, &self.operands[j + 1]),
            }
        }
        result
    }

    fn concat_2_ints(a: &i64, b: &i64) -> i64 {
        let op1 = *a;
        let op2 = b;
        let mut s: String = op1.to_string();
        s += &op2.to_string();
        s.parse().unwrap()
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

#[derive(Debug, VariantArray, EnumString, Display, PartialEq, Clone)]
enum Operator {
    #[strum(to_string = "+")]
    Add,
    #[strum(to_string = "*")]
    Mul,
    #[strum(to_string = "||")]
    Concat,
}

#[cfg(test)]
mod tests {

    use convert_base::Convert;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solve() {
        let eq1 = Equation::from_str("190: 10 19").unwrap();
        assert_eq!(eq1.solve(&vec![Operator::Add]), 29);
        assert_eq!(eq1.solve(&vec![Operator::Mul]), 190);
        assert_eq!(eq1.solve(&vec![Operator::Concat]), 1019);
        let eq1 = Equation::from_str("3267: 81 40 27").unwrap();
        assert_eq!(eq1.solve(&vec![Operator::Add, Operator::Add]), 148);
        assert_eq!(eq1.solve(&vec![Operator::Add, Operator::Mul]), (81 + 40) * 27);
        assert_eq!(eq1.solve(&vec![Operator::Mul, Operator::Mul]), (81 * 40) * 27);
        assert_eq!(eq1.solve(&vec![Operator::Concat, Operator::Add]), 8167);
        assert_eq!(eq1.solve(&vec![Operator::Concat, Operator::Concat]), 814027);
        assert_eq!(eq1.solve(&vec![Operator::Add, Operator::Concat]), 12127);

        let eq1 = Equation::from_str("7290: 6 8 6 15").unwrap();
        assert_eq!(eq1.solve(&vec![Operator::Add, Operator::Concat, Operator::Mul]), 146 * 15);
    }

    #[test]
    fn test_base_convert() {
        let mut base = Convert::new(10, 2);
        let b3: Vec<u64> = base.convert(&vec![3u64]);
        println!("{:?}", b3);
    }

    #[test]
    fn test_solvable() {
        let eq1 = Equation::from_str("3267: 81 40 27").unwrap();
        assert!(eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("190: 10 19").unwrap();
        assert!(eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("83: 17 5").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("156: 15 6").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("7290: 6 8 6 15").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("161011: 16 10 13").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("192: 17 8 14").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("21037: 9 7 18 13").unwrap();
        assert!(!eq1.is_solvable(&[Operator::Add, Operator::Mul]));
        let eq1 = Equation::from_str("292: 11 6 16 20").unwrap();
        assert!(eq1.is_solvable(&[Operator::Add, Operator::Mul]));

        // Part 2
        let eq1 = Equation::from_str("7290: 6 8 6 15").unwrap();
        assert!(eq1.is_solvable(&[Operator::Add, Operator::Mul, Operator::Concat]));
    }

    #[test]
    fn test_solveablefrominput() {
        let data = read_test_data(Path::new("./example.dat")).unwrap();

        let mut solveable_equations: Vec<usize> = vec![];
        for (i, line) in data.lines().enumerate() {
            if Equation::from_str(line).unwrap().is_solvable(&[Operator::Add, Operator::Mul]) {
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
