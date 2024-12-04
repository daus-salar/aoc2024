use std::{
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn main() -> Result<(), Error> {
    let input = read_input("input")?;

    let comp = Computation::parse(input)?;
    
    println!("Compute {}", comp.calculate());
    println!("Compute {}", comp.calculate_with_switches());

    Ok(())
}

fn read_input(path: &str) -> Result<String, Error> {
    let mut input = String::new();
    let mut input_file = File::open(path)?;
    input_file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse(input: &str) -> Result<Vec<Operation>, Error> {
    let ops_pattern = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();

    let mut result: Vec<Operation> = vec![];
    for c in ops_pattern.captures_iter(input) {
        match &c[0] {
            "do()" => result.push(Operation::Do()),
            "don't()" => result.push(Operation::DoNot()),
            mul => {
                let mul_pattern = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
                for (_, [op1, op2]) in mul_pattern.captures(&mul).map(|c| c.extract()) {
                    let op1: i32 = op1.parse().unwrap();
                    let op2: i32 = op2.parse().unwrap();
                    result.push(Operation::Multiply(op1, op2));
                }
            }
        }
    }

    Ok(result)
}
#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(i32, i32),
    Do(),
    DoNot(),
}

struct Computation {
    ops: Vec<Operation>,
}

type CalculateFn = fn(&Computation, ignore_do : bool) -> i32;

impl Computation {
    fn calculate(&self) -> i32 {
        self.ops
            .iter()
            .map(|op| match op {
                Operation::Multiply(a, b) => a * b,
                Operation::Do() => 0,
                Operation::DoNot() => 0,
            })
            .sum()
    }

    fn calculate_with_switches(&self) -> i32 {
        let mut enable = true;
        let mut result = 0;
        for op in self.ops.iter() {
            match op {
                Operation::Multiply(op1, op2) => if enable { result +=*op1 * *op2}
                Operation::Do() => enable = true,
                Operation::DoNot() => enable = false,
            }
        }
        result
    }
    fn from(ops: Vec<Operation>) -> Self {
        Computation { ops }
    }

    fn parse(input: String) -> Result<Self, Error> {
        let ops = parse(input.as_str());
        let comp = ops.map(|ops| Computation::from(ops));
        comp
    }
}
#[derive(Debug)]
pub enum Error {
    InputError(io::Error),
    ParsingInput(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::InputError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParsingInput(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_input_test() {
        let input = read_input("input_test").unwrap();
        assert_eq!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            input
        );
    }

    #[test]
    fn part_one_parse() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed_input: Result<Vec<Operation>, Error> = parse(input);
        assert_eq!(
            parsed_input.unwrap(),
            vec![
                Operation::Multiply(2, 4),
                Operation::Multiply(5, 5),
                Operation::Multiply(11, 8),
                Operation::Multiply(8, 5)
            ]
        );
    }
    #[test]
    fn part_two_parse() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed_input: Result<Vec<Operation>, Error> = parse(input);
        assert_eq!(
            parsed_input.unwrap(),
            vec![
                Operation::Multiply(2, 4),
                Operation::DoNot(),
                Operation::Multiply(5, 5),
                Operation::Multiply(11, 8),
                Operation::Do(),
                Operation::Multiply(8, 5)
            ]
        );
    }
    #[test]
    fn operations_calculate() {
        let cmps = Computation::from(vec![
            Operation::Multiply(2, 4),
            Operation::Multiply(5, 5),
            Operation::Multiply(11, 8),
            Operation::Multiply(8, 5),
        ]);
        assert_eq!(cmps.calculate(), 161);
    }

    #[test]
    fn part_one_input_test_complete() {
        let input = read_input("input_test").unwrap();
        let comp = Computation::parse(input);
        let result = comp.map(|comp| comp.calculate());
        assert_eq!(161, result.unwrap());
    }

    #[test]
    fn part_two_input_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let comp = Computation::parse(input);
        let result = comp.map(|comp| comp.calculate_with_switches());
        assert_eq!(48, result.unwrap());
    }
}
