use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = parse_input(path)?;
    let safe_reports = evaluate_report(&input)?;
    let safe_reports_with_dumping = evaluate_reports_with_dumping(&input)?;

    println!("Reports are safe {} ", safe_reports);
    println!(
        "Reports are safe {} when one is dumped ",
        safe_reports_with_dumping
    );

    Ok(())
}

fn evaluate_reports_with_dumping(input: &Vec<Vec<i32>>) -> Result<usize, Error> {
    let safe_reports = input
        .into_iter()
        .filter(|r| {
            if !is_safe_with_dumping(&r) {
                if is_safe_dumped_brute(&r) {
                    println!("{:?}", r);
                    true
                } else {
                    false
                }
            } else {
                true
            }
        })
        .count();
    Ok(safe_reports)
}

fn evaluate_report(input: &Vec<Vec<i32>>) -> Result<usize, Error> {
    let safe_reports = input.into_iter().filter(|r| is_safe(&r).is_ok()).count();
    Ok(safe_reports)
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

#[derive(Debug, PartialEq)]
enum ErrorAt {
    Pos(usize),
}

fn is_safe_with_dumping(report: &Vec<i32>) -> bool {
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

fn is_safe_dumped_brute(report: &Vec<i32>) -> bool {
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

fn dump_pos2<'a>(report: &'a Vec<i32>, pos: usize) -> Box<dyn Iterator<Item = &i32> + 'a> {
    let lower = if pos == 0 { 0 } else { pos };
    let upper = if pos == 0 { 1 } else { pos };

    let it = report
        .into_iter()
        .take(lower)
        .chain(report.into_iter().skip(upper));
    Box::new(it)
}

fn dump_pos(report: &Vec<i32>, pos: usize) -> Vec<i32> {
    let mut report_altered = report.clone();

    let _ = report_altered.splice(pos..pos + 1, vec![]);
    report_altered
}

fn is_safe(report: &Vec<i32>) -> Result<(), ErrorAt> {
    let mut before: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    for (idx, level) in report.into_iter().enumerate() {
        (increasing, before) = match (
            increasing,
            before.map(|b| b < *level),
            before.map(|b| (b - *level).abs()),
        ) {
            (.., Some(local_increase)) if local_increase > 3 || local_increase < 1 => {
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

fn is_safe2<'a, T: Iterator<Item = &'a i32>>(report: T) -> Result<(), ErrorAt> {
    let mut before: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    for (idx, level) in report.into_iter().enumerate() {
        (increasing, before) = match (
            increasing,
            before.map(|b| b < *level),
            before.map(|b| (b - *level).abs()),
        ) {
            (.., Some(local_increase)) if local_increase > 3 || local_increase < 1 => {
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
        assert_eq!(false, is_safe_with_dumping(&vec![9, 7, 6, 2, 1]));
        assert_eq!(false, is_safe_with_dumping(&vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn is_safe_dumped_true() {
        assert_eq!(true, is_safe_with_dumping(&vec![7, 6, 4, 2, 1]));
        assert_eq!(true, is_safe_with_dumping(&vec![1, 3, 2, 4, 5]));
        assert_eq!(true, is_safe_with_dumping(&vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe_with_dumping(&vec![1, 3, 6, 7, 9]));
        assert_eq!(true, is_safe_with_dumping(&vec![6, 7, 5, 4, 3]));
    }

    #[test]
    fn is_safe_dumped_debug_true() {
        assert_eq!(
            true,
            is_safe_with_dumping(&vec![84, 82, 83, 84, 85, 88, 90])
        );
        assert_eq!(
            true,
            is_safe_with_dumping(&vec![70, 79, 82, 85, 86, 88, 89, 90])
        );
        assert_eq!(
            true,
            is_safe_with_dumping(&vec![79, 90, 82, 85, 86, 88, 89, 90])
        );
    }
    #[test]
    fn evaluate_report_simple() {
        let input = parse_input("input_test").unwrap();
        let r = evaluate_report(&input).unwrap();
        assert_eq!(2, r);
    }

    #[test]
    fn evaluate_report_dumped_simple() {
        let input = parse_input("input_test").unwrap();
        let r = evaluate_reports_with_dumping(&input).unwrap();
        assert_eq!(4, r);
    }
}
