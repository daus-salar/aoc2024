extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;
extern crate crypto;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub mod day10;

aoc_lib! { year = 2024 }

pub mod common {

    use std::{
        fs::File,
        io::{self, Read},
    };

    #[derive(Debug, PartialEq)]
    pub enum Error {
        IO(std::io::ErrorKind),
    }

    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Error::IO(value.kind())
        }
    }

    pub fn load(path: &str) -> Result<String, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;
        Ok(input)
    }
}
