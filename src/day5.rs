use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self},
    num::ParseIntError,
};

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input).unwrap();
    let ord = CustomOrd::new(rules);

    updates
        .iter()
        .filter(|u| ord.check(u))
        .filter_map(|u| {
            let middle = u.len() / 2;
            u.get(middle)
        })
        .copied()
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input).unwrap();
    let ord = CustomOrd::new(rules);

    let mut sum = 0;
    for mut u in updates.into_iter().filter(|u| !ord.check(u)) {
        u.sort_by(&ord);
        let middle = u.len() / 2;
        sum += u.get(middle).unwrap();
    }
    sum
}

pub struct CustomOrd {
    rules: HashMap<i32, HashMap<i32, OrdRule>>,
}

impl CustomOrd {
    fn new(rules: Vec<OrdRule>) -> Self {
        let initial: HashMap<i32, HashMap<i32, OrdRule>> = HashMap::new();
        let rules = rules.into_iter().fold(initial, |mut m, rule| {
            m.entry(rule.page1)
                .or_default()
                .insert(rule.page2, rule);
            m
        });
        CustomOrd { rules }
    }

    fn cmp(&self, o1: &i32, o2: &i32) -> Ordering {
        if o1 == o2 {
            return Ordering::Equal;
        }
        if let Some(rules_o1) = self.rules.get(o1) {
            if rules_o1.contains_key(o2) {
                return Ordering::Less;
            }
        }
        if let Some(rules_o2) = self.rules.get(o2) {
            if rules_o2.contains_key(o1) {
                return Ordering::Greater;
            }
        }
        panic!("not comparable");
    }

    pub fn check(&self, update: &Update) -> bool {
        let mut last_page: Option<i32> = None;
        for page in &update.pages {
            if let Some(last_page) = last_page {
                if self.cmp(&last_page, page) != Ordering::Less {
                    return false;
                }
            }
            last_page = Some(*page);
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OrdRule {
    page1: i32,
    page2: i32,
}

impl PartialOrd for OrdRule {
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

impl OrdRule {
    pub fn parse(line: &str) -> Result<OrdRule, Error> {
        let mut page1 = 0;

        let mut candidate = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                candidate.push(c);
                continue;
            } else if c == '|' {
                page1 = candidate.parse::<i32>()?;
                candidate.clear();
                continue;
            }
        }
        if !candidate.is_empty() {
            Ok(OrdRule {
                page1,
                page2: candidate.parse::<i32>()?,
            })
        } else {
            Err(Error::InCompleteOrderRule)
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Update {
    pages: Vec<i32>,
}
impl Update {
    pub fn parse(line: &str) -> Result<Self, Error> {
        let mut pages: Vec<i32> = Vec::new();
        let mut candidate = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                candidate.push(c);
                continue;
            } else if c == ',' {
                pages.push(candidate.parse::<i32>()?);
                candidate.clear();
                continue;
            }
        }
        if !candidate.is_empty() {
            pages.push(candidate.parse::<i32>()?);
        }
        Ok(Update { pages })
    }

    fn len(&self) -> usize {
        self.pages.len()
    }

    fn get(&self, index: usize) -> Option<&i32> {
        self.pages.get(index)
    }

    fn sort_by(&mut self, ord: &CustomOrd) {
        self.pages.sort_by(|a, b| ord.cmp(a, b));
    }
}
#[derive(Debug, PartialEq)]
pub enum Error {
    IO(std::io::ErrorKind),
    ParseInt(core::num::IntErrorKind),
    InCompleteOrderRule,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value.kind())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value.kind().clone())
    }
}

pub fn parse_input(input: &str) -> Result<(Vec<OrdRule>, Vec<Update>), Error> {
    let mut rules_part = true;

    let mut rules = Vec::<OrdRule>::new();
    let mut updates: Vec<Update> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            rules_part = false;
            continue;
        } else if rules_part {
            rules.push(OrdRule::parse(line)?);
            continue;
        } else {
            updates.push(Update::parse(line)?);
            continue;
        }
    }
    Ok((rules, updates))
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn parse_update_simple() {
        assert_eq!(
            Update::parse("2,345,64,343,64,3,244"),
            Ok(Update {
                pages: vec![2, 345, 64, 343, 64, 3, 244]
            })
        );
    }

    #[test]
    fn parse_order_rule_simple() {
        assert_eq!(
            OrdRule::parse("2|345"),
            Ok(OrdRule {
                page1: 2,
                page2: 345
            })
        );
    }
    #[test]
    fn parse_test_input() {
        let (rules, updates) = parse("test_data/day5.txt").unwrap();
        assert!(rules.contains(&OrdRule {
            page1: 47,
            page2: 53
        }));
        let ord = CustomOrd::new(rules);
        let first = updates.first().unwrap();
        assert!(ord.check(first));
        assert!(!ord.check(updates.get(3).unwrap()));
    }

    #[test]
    fn part1_test_input() {
        let input = load("test_data/day5.txt").unwrap();

        assert_eq!(143, part1(input.as_str()));
    }

    fn parse(path: &str) -> Result<(Vec<OrdRule>, Vec<Update>), Error> {
        let input = load(path)?;

        parse_input(input.as_str())
    }

    fn load(path: &str) -> Result<String, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;
        Ok(input)
    }
}
