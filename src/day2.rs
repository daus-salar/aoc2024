use std::{io, num::ParseIntError};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let input = from(input);
    evaluate_report(&input).unwrap()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let input = from(input);
    evaluate_reports_with_dumping(&input).unwrap()
}

fn evaluate_reports_with_dumping(input: &[Vec<i32>]) -> Result<usize, Error> {
    let safe_reports = input
        .iter()
        .filter(|r| {
            is_safe_with_dumping(r)
        })
        .count();
    Ok(safe_reports)
}

fn evaluate_report(input: &[Vec<i32>]) -> Result<usize, Error> {
    let safe_reports = input.iter().filter(|r| is_safe(r).is_ok()).count();
    Ok(safe_reports)
}

fn from(input: &str) -> Vec<Vec<i32>> {
    let mut input_parsed = Vec::<Vec<i32>>::new();
    for l in input.lines() {
        let r: Vec<i32> = l
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .flat_map(Result::ok)
            .collect();
        input_parsed.push(r);
    }
    input_parsed
}

#[derive(Debug, PartialEq)]
enum ErrorAt {
    Pos(usize),
}

fn is_safe_with_dumping(report: &[i32]) -> bool {
    match is_safe(report) {
        Ok(()) => true,
        Err(ErrorAt::Pos(1..=2)) => {
            is_safe(&dump_pos(report, 0)).is_ok() || is_safe(&dump_pos(report, 1)).is_ok()
        }
        Err(ErrorAt::Pos(pos)) => {
            let report_altered = dump_pos(report, pos);
            is_safe(&report_altered).is_ok()
        }
    }
}

fn is_safe_dumped_brute(report: &[i32]) -> bool {
    match is_safe(report) {
        Ok(()) => true,
        Err(ErrorAt::Pos(..)) => {
            for i in 0..report.len() {
                if is_safe(&dump_pos(report, i)).is_ok() {
                    return true;
                }
            }
            false
        }
    }
}

fn dump_pos(report: &[i32], pos: usize) -> Vec<i32> {
    let mut report_altered = report.to_owned();

    let _ = report_altered.splice(pos..pos + 1, vec![]);
    report_altered
}

fn is_safe(report: &[i32]) -> Result<(), ErrorAt> {
    let mut before: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    for (idx, level) in report.iter().enumerate() {
        (increasing, before) = match (
            increasing,
            before.map(|b| b < *level),
            before.map(|b| (b - *level).abs()),
        ) {
            (.., Some(local_increase)) if !(1..=3).contains(&local_increase) => {
                // increase amount critical
                return Err(ErrorAt::Pos(idx));
            }
            (None, None, ..) => (None, Some(*level)),
            (None, Some(local_increasing), ..) => (Some(local_increasing), Some(*level)),
            (Some(true), Some(true), ..) => (Some(true), Some(*level)),
            (Some(false), Some(false), ..) => (Some(false), Some(*level)),
            _ => {
                return Err(ErrorAt::Pos(idx));
            }
        }
    }
    Ok(())
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

    use std::fs::File;

    use io::Read;

    use super::*;

    #[test]
    fn is_safe_simple_false() {
        let a = vec![3, 4, 2, 1, 3, 3];
        assert_eq!(Err(ErrorAt::Pos(2)), is_safe(&a));
    }

    #[test]
    fn is_safe_true_simple() {
        let a = vec![1, 2, 3, 5, 7, 8];
        assert_eq!(Ok(()), is_safe(&a));
    }
    #[test]
    fn is_safe_critical_increase() {
        let a = vec![1, 2, 7, 8, 9];
        assert_eq!(Err(ErrorAt::Pos(2)), is_safe(&a));
    }

    #[test]
    fn is_safe_dumped_false() {
        assert!(!is_safe_with_dumping(&[9, 7, 6, 2, 1]));
        assert!(!is_safe_with_dumping(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn is_safe_dumped_true() {
        assert!(is_safe_with_dumping(&[7, 6, 4, 2, 1]));
        assert!(is_safe_with_dumping(&[1, 3, 2, 4, 5]));
        assert!(is_safe_with_dumping(&[8, 6, 4, 4, 1]));
        assert!(is_safe_with_dumping(&[1, 3, 6, 7, 9]));
        assert!(is_safe_with_dumping(&[6, 7, 5, 4, 3]));
    }

    #[test]
    fn is_safe_dumped_debug_true() {
        assert!(
            is_safe_with_dumping(&[84, 82, 83, 84, 85, 88, 90])
        );
        assert!(
            is_safe_with_dumping(&[70, 79, 82, 85, 86, 88, 89, 90])
        );
        assert!(
            is_safe_with_dumping(&[79, 90, 82, 85, 86, 88, 89, 90])
        );
    }
    #[test]
    fn evaluate_report_simple() {
        let input = parse_input("test_data/day2.txt").unwrap();
        let r = evaluate_report(&input).unwrap();
        assert_eq!(2, r);
    }

    #[test]
    fn evaluate_report_dumped_simple() {
        let input = parse_input("test_data/day2.txt").unwrap();
        let r = evaluate_reports_with_dumping(&input).unwrap();
        assert_eq!(4, r);
    }

    fn parse_input(path: &str) -> Result<Vec<Vec<i32>>, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;

        let input = input.as_str();
        Ok(from(input))
    }
}
