use std::{
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn main() -> Result<(), Error> {
    let input = parse_input("input")?;
    
    todo!();

    Ok(())
}


fn parse_input(path: &str) -> Result<Vec<Vec<i32>>, Error> {
    let mut input = String::new();
    let mut input_file = File::open(path)?;
    input_file.read_to_string(&mut input)?;

    let mut input_parsed = Vec::<Vec<i32>>::new();
    for l in input.lines() {
        let r: Result<Vec<i32>, _> = l
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();
        match r {
            Ok(r) => input_parsed.push(r),
            Err(pie) => return Err(Error::ParsingInput(pie)),
        }
    }
    Ok(input_parsed)
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
        let input = parse_input("input_test").unwrap();
       todo!();
        assert_eq!(2, 2);
    }

    #[test]
    fn part_two_input_test() {
        let input = parse_input("input_test").unwrap();
        todo!();
        assert_eq!(2, 2);
    }
}
