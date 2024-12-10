use std::{io, num::ParseIntError};

use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let eqs = parse(input).unwrap();

    eqs.into_iter()
        .filter_map(|mut e| if e.calibrate() { Some(e) } else { None })
        .map(|e| e.result)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let eqs = parse(input).unwrap();

    eqs.into_iter()
        .filter_map(|mut e| if e.calibrate2() { Some(e) } else { None })
        .map(|e| e.result)
        .sum()
}

fn parse(input: &str) -> Result<Vec<Equation>, Error> {
    input
        .lines()
        .map(Equation::parse)
        .collect::<Result<Vec<Equation>, Error>>()
}

#[derive(Debug, PartialEq, Eq)]
enum Operators {
    Multiply,
    Add,
    Concate,
}

#[derive(Debug, PartialEq)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
    operators: Vec<Operators>,
}

impl Equation {
    fn parse(line: &str) -> Result<Self, Error> {
        let mut result: i64 = 0;

        let mut operands = vec![];

        let mut candidate = String::new();
        let mut result_part = true;
        for c in line.chars() {
            if result_part {
                if c.is_numeric() {
                    candidate.push(c);
                    continue;
                } else if c == ':' {
                    result = candidate.parse::<i64>()?;
                    candidate.clear();
                    result_part = false;
                    continue;
                }
            } else if c.is_numeric() {
                candidate.push(c);
                continue;
            } else if c.is_whitespace() && !candidate.is_empty() {
                let operand = candidate.parse::<i64>()?;
                operands.push(operand);
                candidate.clear();
                continue;
            }
        }
        if !candidate.is_empty() {
            let operand = candidate.parse::<i64>()?;
            operands.push(operand);
            candidate.clear();
        }

        Ok(Equation {
            result,
            operands,
            operators: vec![],
        })
    }

    fn verify(&self) -> bool {
        let mut rhs = self.operands[0];
        for op_pair in self.operands[1..].iter().zip(self.operators.iter()) {
            match op_pair {
                (operand, Operators::Add) => {
                    rhs += *operand;
                }
                (operand, Operators::Multiply) => {
                    rhs *= *operand;
                }
                (operand, Operators::Concate) => {
                    let mut intermediate = rhs.to_string();
                    intermediate.push_str(operand.to_string().as_str());
                    rhs = intermediate.parse().unwrap();
                }
            };
        }
        rhs == self.result
    }

    fn calibrate(self: &mut Equation) -> bool {
        for _ in 1..self.operands.len() {
            self.operators.push(Operators::Add);
        }
        loop {
            if self.verify() {
                return true;
            }
            let range = 0..self.operators.len();
            for i in range {
                match self.operators[i] {
                    Operators::Multiply => {
                        self.operators[i] = Operators::Add;
                        if i == self.operators.len() - 1 {
                            self.operators.clear();
                            return false;
                        }
                    }
                    Operators::Add => {
                        self.operators[i] = Operators::Multiply;

                        break;
                    }
                    Operators::Concate => {}
                }
            }
        }
    }

    fn calibrate2(self: &mut Equation) -> bool {
        for _ in 1..self.operands.len() {
            self.operators.push(Operators::Add);
        }
        loop {
            if self.verify() {
                return true;
            }
            let range = 0..self.operators.len();
            let mut all_flip = true;
            for i in range {
                match self.operators[i] {
                    Operators::Multiply => {
                        self.operators[i] = Operators::Add;
                    }
                    Operators::Add => {
                        self.operators[i] = Operators::Concate;
                        all_flip = false;
                        break;
                    }
                    Operators::Concate => {
                        self.operators[i] = Operators::Multiply;
                        all_flip = false;
                        break;
                    }
                }
            }

            if all_flip {
                self.operators.clear();
                return false;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IO(std::io::ErrorKind),
    ParseError(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value.kind())
    }
}
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseError(value)
    }
}
#[cfg(test)]
mod tests {

    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn part2_example() {}

    #[test]
    fn equation_parse_example() {
        assert_eq!(
            Ok(Equation {
                result: 190,
                operands: vec![10, 19],
                operators: vec![]
            }),
            Equation::parse("190: 10 19")
        );
    }

    #[test]
    fn part1_parse_complete() {
        let input = load("test_data/day7.txt").unwrap();
        let eqs = parse(input.as_str());
        assert!(eqs.is_ok());
        let eqs = eqs.unwrap();

        assert_eq!(
            Equation {
                result: 292,
                operands: vec![11, 6, 16, 20],
                operators: vec![]
            },
            eqs[8]
        );
    }

    #[test]
    fn part1_data() {
        let input = load("test_data/day7.txt").unwrap();
        let result = part1(input.as_str());

        assert_eq!(3749, result);
    }

    #[test]
    fn part2_data() {
        let input = load("test_data/day7.txt").unwrap();
        let result = part2(input.as_str());

        assert_eq!(11387, result);
    }
    #[test]
    fn part1_find_operators() {
        let mut eq = Equation {
            result: 190,
            operands: vec![10, 19],
            operators: vec![],
        };
        eq.calibrate();
        assert_eq!(
            Equation {
                result: 190,
                operands: vec![10, 19],
                operators: vec![Operators::Multiply],
            },
            eq
        )
    }

    #[test]
    fn equation_verify() {
        let eq = Equation {
            result: 156,
            operands: vec![15, 6],
            operators: vec![Operators::Concate],
        };
        assert!(eq.verify());
    }
    #[test]
    fn equation_verify2() {
        let eq = Equation {
            result: 7290,
            operands: vec![6, 8, 6, 15],
            operators: vec![Operators::Multiply, Operators::Concate, Operators::Multiply],
        };
        assert!(eq.verify());
    }

    #[test]
    fn part1_calibrate2_operators() {
        let mut eq = Equation {
            result: 156,
            operands: vec![15, 6],
            operators: vec![],
        };
        eq.calibrate2();
        assert_eq!(
            Equation {
                result: 156,
                operands: vec![15, 6],
                operators: vec![Operators::Concate],
            },
            eq
        )
    }

    #[test]
    fn part1_calibrate2_operators2() {
        let mut eq = Equation {
            result: 7290,
            operands: vec![6, 8, 6, 15],
            operators: vec![],
        };
        eq.calibrate2();
        assert!(eq.verify());
        assert_eq!(
            Equation {
                result: 7290,
                operands: vec![6, 8, 6, 15],
                operators: vec![Operators::Multiply, Operators::Concate, Operators::Multiply],
            },
            eq
        )
    }

    #[test]
    fn part1_find_operators2() {
        let mut eq = Equation {
            result: 3267,
            operands: vec![81, 40, 27],
            operators: vec![],
        };
        eq.calibrate();
        assert_eq!(
            Equation {
                result: 3267,
                operands: vec![81, 40, 27],
                operators: vec![Operators::Multiply, Operators::Add],
            },
            eq
        )
    }

    fn load(path: &str) -> Result<String, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;
        Ok(input)
    }
}
