use std::{
    collections::HashSet,
    fmt::Debug,
};

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut map = LabMap::new(input);
    map.simulate();
    map.path_length
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    let prototype = LabMap::new(input);
    let free_guards_path = prototype.clone().simulate();
    let free_guards_path: HashSet<(usize, usize)> = free_guards_path
        .into_iter()
        .map(|gs| gs.pos)
        .filter_map(|pos| {
            if prototype.inside_pos(pos.0, pos.1) {
                Some((pos.0.unsigned_abs(), pos.1.unsigned_abs()))
            } else {
                None
            }
        })
        .fold(HashSet::new(), |mut s, p| {
            s.insert(p);
            s
        });
    for (i, j) in free_guards_path {
        let mut sim = prototype.clone();

        if match prototype.content(i, j) {
            '#' | '>' | 'v' | '<' | '^' => true,
            _ => false,
        } {
            continue;
        }
        sim.set_content(i, j, 'O');
        sim.simulate();

        if sim.inside() {
            count += 1;
        }
    }

    count
}

#[derive(Clone)]
pub struct LabMap {
    data: Vec<Vec<char>>,
    path_length: usize,
    pos: (isize, isize),
    new_content: Option<char>, //  obstacles: Vec<(usize, usize)>,
}

impl LabMap {
    
    pub fn new(lab_map_raw: &str) -> Self {
        let mut pos = None;
        let mut obstacles: Vec<(usize, usize)> = Vec::new();
        let data: Vec<Vec<char>> = lab_map_raw
            .lines()
            .enumerate()
            .map(|(row, l)| {
                let row: Vec<char> = l
                    .chars()
                    .enumerate()
                    .map(|(col, c)| {
                        if c == '^' || c == '>' || c == 'v' || c == '<' {
                            pos = Some((row, col))
                        } else if c == '#' {
                            obstacles.push((row, col));
                        }
                        c
                    })
                    .collect();
                row
            })
            .collect();
        let pos: (isize, isize) = pos.map(|(row, col)| (row as isize, col as isize)).unwrap();
        LabMap {
            data,
            path_length: 1,
            pos,
            new_content: Some('|'), //       obstacles,
        }
    }

    pub fn is_obstacle(&self, row: usize, col: usize) -> bool {
        self.data[row][col] == '#'
    }

    pub fn content(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    pub fn set_content(&mut self, row: usize, col: usize, new_value: char) {
        self.data[row][col] = new_value;
    }

    pub fn inside(&self) -> bool {
        let row = self.pos.0;
        let col = self.pos.1;
        self.inside_pos(row, col)
    }

    fn inside_pos(&self, row: isize, col: isize) -> bool {
        0 <= row
            && row < (self.data.len() as isize)
            && 0 <= col
            && col < (self.data[0].len() as isize)
    }

    pub fn infront_of_guard_pos(&self) -> Option<(isize, isize)> {
        if !self.inside() {
            return None;
        }
        match self.content(self.row(), self.col()) {
            '>' => Some((self.pos.0, self.pos.1 + 1)),
            '^' => Some((self.pos.0 - 1, self.pos.1)),
            '<' => Some((self.pos.0, self.pos.1 - 1)),
            'v' => Some((self.pos.0 + 1, self.pos.1)),
            d => {
                panic!("unexpected direction {:} at {:?}.", d, self.pos);
            }
        }
    }

    fn guard_sees_infront(&self) -> Option<char> {
        if let Some((row, col)) = self.infront_of_guard_pos() {
            if self.inside_pos(row, col) {
                let row = row.unsigned_abs();
                let col = col.unsigned_abs();
                return Some(self.content(row, col));
            }
        }
        None
    }

    fn simulate(&mut self) -> HashSet<GuardState> {
        let mut path: HashSet<GuardState> = HashSet::new();
        loop {
            let current_pos = GuardState {
                pos: self.pos,
                dir: self.current_dir(),
            };
            if !path.insert(current_pos) {
                break;
            }
            // println!["{:?}", self];
            match self.guard_sees_infront() {
                Some('#') | Some('O') => {
                    self.turn();
                }
                _ => {
                    self.move_forward();
                }
            }
            if !self.inside() {
                break;
            }
        }
        path
    }

    fn current_dir(&mut self) -> char {
        self.content(self.row(), self.col())
    }

    fn col(&self) -> usize {
        self.pos.1.unsigned_abs()
    }

    fn row(&self) -> usize {
        self.pos.0.unsigned_abs()
    }

    fn replace(&mut self, row: isize, col: isize) {
        let guard = self.content(self.row(), self.col());
        if self.inside_pos(row, col) {
            let row = row.unsigned_abs();
            let col = col.unsigned_abs();
            if !self.visited(row, col) {
                if self.is_obstacle(row, col) {
                    panic!("obstacle")
                }
                self.path_length += 1;
            }
            self.set_content(row, col, guard);
        }

        self.set_content(self.row(), self.col(), self.new_content.unwrap());
        self.pos = (row, col);
    }

    fn visited(&mut self, row: usize, col: usize) -> bool {
        self.content(row, col) == '|'
            || self.content(row, col) == '-'
            || self.content(row, col) == '+'
    }

    fn move_forward(&mut self) {
        if let Some((row, col)) = self.infront_of_guard_pos() {
            self.replace(row, col);
            self.new_content = self.new_content();
        }
    }

    fn turn(&mut self) {
        let new_dir = match self.content(self.row(), self.col()) {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            d => {
                panic!("unexpected direction {:} at {:?}.", d, self.pos);
            }
        };
        self.new_content = Some('+');
        self.set_content(self.row(), self.col(), new_dir);
    }

    fn new_content(&self) -> Option<char> {
        if !self.inside() {
            return None;
        }
        match self.content(self.row(), self.col()) {
            '>' | '<' => {
                if (self.inside_pos(self.pos.0 - 1, self.pos.1)
                    && self.content(self.row() - 1, self.col()) == '|')
                    || (self.inside_pos(self.pos.0 + 1, self.pos.1)
                        && self.content(self.row() + 1, self.col()) == '|')
                {
                    Some('+')
                } else {
                    Some('-')
                }
            }
            'v' | '^' => {
                if (self.inside_pos(self.pos.0, self.pos.1 - 1)
                    && self.content(self.row(), self.col() - 1) == '-')
                    || (self.inside_pos(self.pos.0, self.pos.1 + 1)
                        && self.content(self.row(), self.col() + 1) == '-')
                {
                    Some('+')
                } else {
                    Some('|')
                }
            }
            d => {
                panic!("unexpected direction {:} at {:?}.", d, self.pos);
            }
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct GuardState {
    pos: (isize, isize),
    dir: char,
}

impl Debug for LabMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            for char in line {
                f.write_fmt(format_args!("{:}", char))?;
            }
            f.write_fmt(format_args!("\n"))?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    
    use crate::common;
    use common::load;


    use super::*;

    #[test]
    fn part1_example() {
        //  assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn parse_test() {
        let lab_map_raw = load("test_data/day6.txt").unwrap();

        let  map = LabMap::new(lab_map_raw.as_str());

        let expected = (6, 4);
        assert_eq!(expected, map.pos);
        assert_eq!(Some((5, 4)), map.infront_of_guard_pos());

        assert_eq!('^', map.content(6, 4));
        assert_eq!('#', map.content(6, 1));
        assert_eq!('.', map.content(9, 9));
        assert_eq!('#', map.content(8, 0));
    }

    #[test]
    fn simulate_test() {
        let lab_map_raw = load("test_data/day6.txt").unwrap();

        let mut map = LabMap::new(lab_map_raw.as_str());

        map.simulate();

        assert_eq!(41, map.path_length);
    }

    #[test]
    fn part2_test() {
        let lab_map_raw = load("test_data/day6.txt").unwrap();

        assert_eq!(6, part2(lab_map_raw.as_str()));
    }

    #[test]
    fn part2_input_test() {
        let lab_map_raw = load("input/2024/day6.txt").unwrap();

        assert_eq!(1915, part2(lab_map_raw.as_str()));
    }


}
