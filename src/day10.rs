use std::{io, slice::SliceIndex};

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

#[derive(PartialEq, Eq, Debug)]
pub struct TopoMap {
    heights: Vec<Vec<u16>>,
    trail_heads: Vec<(usize, usize)>,
}

impl TopoMap {
    pub fn new() {}
    pub fn parse(input: String) -> Self {
        let mut map = Vec::<Vec<u16>>::new();
        for line in input.lines() {
            let mut row = Vec::<u16>::new();
            for char in line.chars() {
                row.push((char as u16) - 48);
            }
            map.push(row);
        }
        let trail_heads = search_trails_heads(&map);
        TopoMap {
            heights: map,
            trail_heads,
        }
    }

    pub fn get<'a>(&'a self, index: (usize, usize)) -> Option<&'a u16> {
        self.heights
            .get(index.0)
            .map_or(None, |vec| vec.get(index.1))
    }
}
fn search_trails_heads(input: &[Vec<u16>]) -> Vec<(usize, usize)> {
    let mut trail_heads: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, height) in row.iter().enumerate() {
            if *height == 0 {
                trail_heads.push((row_idx, col_idx));
            }
        }
    }
    trail_heads
}

#[cfg(test)]
mod tests {
    use std::{u16, vec};

    use crate::{
        common::{self, load},
        day10::{search_trails_heads, TopoMap},
    };

    #[test]
    fn part1_example() {}

    #[test]
    fn part2_example() {}
    #[test]
    fn parse_simple() -> Result<(), common::Error> {
        let input = load("test_data/day10.txt")?;
        let map = TopoMap::parse(input);

        assert_eq!(
            map.heights,
            vec![[0, 1, 2, 3], [1, 2, 3, 4], [8, 7, 6, 5], [9, 8, 7, 6]]
        );

        assert_eq!(
            map,
            TopoMap {
                heights: vec![
                    vec![0, 1, 2, 3],
                    vec![1, 2, 3, 4],
                    vec![8, 7, 6, 5],
                    vec![9, 8, 7, 6]
                ],
                trail_heads: vec![(0, 0)]
            }
        );

        Ok(())
    }

    #[test]
    fn search_trails_heads_simple() -> Result<(), common::Error> {
        let input = vec![
            vec![0 as u16, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ];

        let trail_heads = search_trails_heads(&input);

        assert_eq!(vec![(0, 0)], trail_heads);
        Ok(())
    }

    #[test]
    fn find_trail_simple() {
        let map = TopoMap {
            heights: vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ],
            trail_heads: vec![(0, 0)],
        };

        for head in &map.trail_heads {
            let trail_count = count_trails(&map, head, 0);
            assert_eq!(trail_count, 3)
        }
    }

    #[test]
    fn count_trails_simple() {
        let map = TopoMap {
            heights: vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ],
            trail_heads: vec![(0, 0)],
        };

        let trail_count = count_trails(&map, &(0, 0), 0);
        assert_eq!(trail_count, 3)
    }

    fn count_trails(map: &TopoMap, head: &(usize, usize), expected: u16) -> i32 {
        let next_expected = expected + 1;
        let trail_count = match map.get(*head) {
            Some(&found) if found != expected => 0,
            Some(9) => 1,
            Some(0..=8) => {
                let mut count = match *head {
                    (0, 0) => 0,
                    (0, col) => count_trails(map, &(0, col - 1), next_expected),
                    (row, 0) => count_trails(map, &(row - 1, 0), next_expected),
                    (row, col) => {
                        count_trails(map, &(row - 1, col), next_expected)
                            + count_trails(map, &(row, col - 1), next_expected)
                    }
                };
                count += count_trails(map, &(head.0 + 1, head.1), next_expected)
                    + count_trails(map, &(head.0, head.1 + 1), next_expected);
                count
            }
            None => 0,
            _ => panic!("data type is not legal"),
        };
        trail_count
    }
}
