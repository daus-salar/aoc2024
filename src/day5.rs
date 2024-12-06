use std::sync::atomic::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}
#[derive(PartialEq, Eq)]
struct OrderRule {
    page1: i32,
    page2: i32,
}

impl PartialOrd for OrderRule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.page1, self.page2) {
            //(page1 @ page1 = other.page1, other.page2) => {
            (p1, p2) if p1 == other.page1 && p2 == other.page2 => Some(std::cmp::Ordering::Equal),
            (.., p2) if p2 == other.page1 => Some(std::cmp::Ordering::Greater),
            (p1, ..) if p1 == other.page2 => Some(std::cmp::Ordering::Less),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BinaryHeap,
        fs::File,
        io::{self, Read},
    };

    use regex_lite::Regex;
    use reqwest::Error;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn parse_from_file() -> Result<(), io::Error> {
        let path: &str = "test_data/day5.txt";
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;

        let input = input.as_str();

        let mut rules_part = true;

        let mut rules = Vec::<OrderRule>::new();

        let order_rule = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();
        for line in input.lines() {
            if rules_part {
                if line.trim().is_empty() {
                    rules_part = false;
                    continue;
                }

                if let Some((_, [page1, page2])) = order_rule.captures(line).map(|c| c.extract()) {
                    let rule = OrderRule {
                        page1: page1.parse().unwrap(),
                        page2: page2.parse().unwrap(),
                    };
                    rules.push(rule);
                }
            } else {
            }
        }
        Ok(())
    }
}
