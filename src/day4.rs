use std::io::{self};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let puzzle = parse_input(input).unwrap();
    puzzle.count_xmas()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let puzzle = parse_input(input).unwrap();
    puzzle.count_crossed_mas()
}

pub struct Puzzle {
    data: Vec<Vec<char>>,
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
impl Puzzle {
    pub fn columns(&self) -> usize {
        if self.rows() > 0 {
            self.data[0].len()
        } else {
            0
        }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, r_idx: usize, c_idx: usize) -> Option<char> {
        if r_idx < self.rows() && c_idx < self.columns() {
            Some(self.data[r_idx][c_idx])
        } else {
            None
        }
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;
        for r_idx in 0..self.rows() {
            for c_idx in 0..self.columns() {
                if self.get(r_idx, c_idx) == Some('X') {
                    count += self.count_xmas_at(r_idx, c_idx);
                }
            }
        }
        count
    }

    fn count_crossed_mas(&self) -> usize {
        let mut count = 0;
        for r_idx in 0..self.rows() {
            for c_idx in 0..self.columns() {
                if self.is_crossed_mas_at(r_idx, c_idx) {
                    count += 1;
                }
            }
        }
        count
    }
    /*


    */
    fn count_xmas_at(&self, r_idx: usize, c_idx: usize) -> usize {
        if self.get(r_idx, c_idx) != Some('X') {
            return 0;
        }

        let mut count = 0;

        let is_xmas_at =
            |r_idx: usize, c_idx: usize, dir: (isize, isize)| self.is_xmas_at(r_idx, c_idx, dir);
        for dir in [
            (1, 0),
            (0, 1),
            (1, 1),
            (0, -1),
            (-1, 0),
            (1, -1),
            (-1, -1),
            (-1, 1),
            /*
             */
        ] {
            if is_xmas_at(r_idx, c_idx, dir) {
                count += 1;
            }
        }

        count
    }

    fn is_xmas_at(&self, r_idx: usize, c_idx: usize, dir: (isize, isize)) -> bool {
        if self.get(r_idx, c_idx) != Some('X') {
            return false;
        }
        let mut r_idx = r_idx;
        let mut c_idx = c_idx;

        for c in &XMAS[1..] {
            if let Some(new_r) = r_idx.checked_add_signed(dir.0) {
                r_idx = new_r
            } else {
                return false;
            }
            if let Some(new_c) = c_idx.checked_add_signed(dir.1) {
                c_idx = new_c
            } else {
                return false;
            }
            if self.get(r_idx, c_idx) != Some(*c) {
                return false;
            }
        }
        true
    }

    fn is_crossed_mas_at(&self, r_idx: usize, c_idx: usize) -> bool {
        let mut lr = false;
        let mut rl = false;
        if self.get(r_idx, c_idx) != Some('A') {
            return false;
        }
        if let Some(r_idx) = r_idx.checked_sub(1) {
            if let Some(c_idx) = c_idx.checked_sub(1) {
                if (self.get(r_idx, c_idx) == Some('M')
                    && self.get(r_idx + 2, c_idx + 2) == Some('S'))
                    || (self.get(r_idx, c_idx) == Some('S')
                        && self.get(r_idx + 2, c_idx + 2) == Some('M'))
                {
                    lr = true;
                }
            }
        }
        if let Some(r_idx) = r_idx.checked_sub(1) {
            if let Some(c_idx) = c_idx.checked_sub(1) {
                if (self.get(r_idx, c_idx + 2) == Some('M')
                    && self.get(r_idx + 2, c_idx) == Some('S'))
                    || (self.get(r_idx, c_idx + 2) == Some('S')
                        && self.get(r_idx + 2, c_idx) == Some('M'))
                {
                    rl = true;
                }
            }
        }
        lr && rl
    }
}

fn parse_input(var_name: &str) -> Result<Puzzle, Error> {
    let input_parsed: Vec<Vec<char>> = var_name
        .lines()
        .map(|s| s.to_string().chars().collect::<Vec<char>>())
        .collect();
    Ok(Puzzle { data: input_parsed })
}

#[derive(Debug)]
pub enum Error {
    InputError(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::InputError(value)
    }
}


#[cfg(test)]
mod tests {

    use std::fs::File;

    use io::Read;

    use super::*;

    #[test]
    fn part_one_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(10, input.columns());
        assert_eq!(10, input.rows());
        assert_eq!(Some('X'), input.get(2, 4));
    }

    #[test]
    fn is_xmas_at_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(false, input.is_xmas_at(2, 4, (0, 1)));
        assert_eq!(true, input.is_xmas_at(4, 0, (0, 1)));
        assert_eq!(true, input.is_xmas_at(4, 6, (0, -1)));
    }

    #[test]
    fn count_xmas_at_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(0, input.count_xmas_at(2, 4));
        assert_eq!(1, input.count_xmas_at(4, 0));
        assert_eq!(2, input.count_xmas_at(4, 6));
    }

    #[test]
    fn count_xmas_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(18, input.count_xmas());
    }
    #[test]
    fn part_one_count_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        input.count_xmas();
    }

    #[test]
    fn count_is_crossed_mas_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(true, input.is_crossed_mas_at(1, 2));
    }

    #[test]
    fn count_mas_input_test() {
        let input = parse_from_file("test_data/day4.txt").unwrap();

        assert_eq!(9, input.count_crossed_mas());
    }
    #[test]
    fn count_mas_input() {
        let input = parse_from_file("input/2024/day4.txt").unwrap();

        assert_eq!(1864, input.count_crossed_mas());
    }
    fn parse_from_file(path: &str) -> Result<Puzzle, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;

        let var_name = input.as_str();
        parse_input(var_name)
    }
}
