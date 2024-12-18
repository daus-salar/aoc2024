use std::io;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
#[aoc(day10, part1)]
fn part1(input: &str) -> String {
    print!("{:}", input);
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    print!("{:}", input);
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value.kind())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{self, load};

    

    #[test]
    fn part1_example() {
        
    }

    #[test]
    fn part2_example() {
        
    }
    #[test]
    fn parse_simple() -> Result<(), common::Error> {
        let input = load("test_data/day10.txt")?;
        let mut map = Vec::<Vec<u8>>::new();
        for line in input.lines() {
            let mut row = Vec::<u8>::new();
            for char in line.chars() {
                row.push((char as u8) - 48);
            }
            map.push(row);
        }

        assert_eq!(
            map,
            vec![[0, 1, 2, 3], [1, 2, 3, 4], [8, 7, 6, 5], [9, 8, 7, 6]]
        );
        Ok(())
    }
}
