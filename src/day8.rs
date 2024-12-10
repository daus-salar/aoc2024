use std::{
    collections::{HashMap, HashSet},
    io,
};

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (antennas, size) = load_antennas(input);
    let (_, count) = count_antinodes(antennas, size);
    count
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (antennas, size) = load_antennas(input);
    let (_, count) = count_harmonic_antinodes(antennas, size);
    count
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

fn load_antennas(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize) {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut size = 0;
    for (row, line) in input.lines().enumerate() {
        size = line.len();
        for (col, content) in line.chars().enumerate() {
            if content.is_ascii_alphanumeric() {
                antennas.entry(content).or_default().push((row, col))
            }
        }
    }
    (antennas, size)
}

fn count_antinodes(
    antennas: HashMap<char, Vec<(usize, usize)>>,
    size: usize,
) -> (HashSet<Pos>, usize) {
    let mut antinodes = HashSet::<Pos>::new();
    for k in antennas.keys() {
        let antennas = antennas.get(k).unwrap();
        for (idx, a) in antennas.iter().enumerate() {
            // let Some(other) = antennas.get(idx..) else {
            //  continue;
            //};
            for b in antennas {
                let a = (a.0 as isize, a.1 as isize);
                let b = (b.0 as isize, b.1 as isize);
                let diff = (a.0 - b.0, a.1 - b.1);
                if let (0, 0) = diff {
                    continue;
                }
                antinodes.insert(Pos::new(a.0 + diff.0, a.1 + diff.1));
                antinodes.insert(Pos::new(b.0 - diff.0, b.1 - diff.1));
            }
        }
    }

    let antinodes_count = antinodes.iter().filter_map(|p| p.inside_map(size)).count();
    (antinodes, antinodes_count)
}

fn count_harmonic_antinodes(
    antennas: HashMap<char, Vec<(usize, usize)>>,
    size: usize,
) -> (HashSet<Pos>, usize) {
    let mut antinodes = HashSet::<Pos>::new();
    for k in antennas.keys() {
        let antennas = antennas.get(k).unwrap();
        for (idx, a) in antennas.iter().enumerate() {
            // let Some(other) = antennas.get(idx..) else {
            //  continue;
            //};
            for b in antennas {
                let a = (a.0 as isize, a.1 as isize);
                let b = (b.0 as isize, b.1 as isize);
                let diff = (a.0 - b.0, a.1 - b.1);
                if let (0, 0) = diff {
                    continue;
                }
                let mut value = Pos::from(a);
                loop {
                    value = Pos::new(value.x + diff.0, value.y + diff.1);
                    if value.inside_map(size).is_some() {
                        antinodes.insert(value.clone());
                    } else {
                        break;
                    }
                }
                let mut value = Pos::from(a);
                loop {
                    value = Pos::new(value.x - diff.0, value.y - diff.1);
                    if value.inside_map(size).is_some() {
                        antinodes.insert(value.clone());
                    } else {
                        break;
                    }
                }
                antinodes.insert(Pos::new(b.0 - diff.0, b.1 - diff.1));
            }
        }
    }

    let antinodes_count = antinodes.iter().filter_map(|p| p.inside_map(size)).count();
    (antinodes, antinodes_count)
}
#[derive(Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn inside_map(&self, size: usize) -> Option<(usize, usize)> {
        if 0 <= self.x && self.x < (size as isize) && 0 <= self.y && self.y < (size as isize) {
            Some((self.x.unsigned_abs(), self.y.unsigned_abs()))
        } else {
            None
        }
    }
}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        Pos {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}
impl From<Pos> for (usize, usize) {
    fn from(value: Pos) -> Self {
        (value.x.unsigned_abs(), value.x.unsigned_abs())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn part1_example_01() -> Result<(), Error> {
        let input = load("test_data/day8.txt")?;
        let input = input.as_str();

        let (antennas, ..) = load_antennas(input);

        assert_eq!(
            antennas.get(&'0'),
            Some(&vec![(1, 8), (2, 5), (3, 7), (4, 4)])
        );
        assert_eq!(antennas.get(&'A'), Some(&vec![(5, 6), (8, 8), (9, 9)]));
        Ok(())
    }

    #[test]
    fn part1_example_02() -> Result<(), Error> {
        let antennas = HashMap::from([
            //         ('0', vec![(1, 8), (2, 5)]),
            ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
            ('A', vec![(5, 6), (8, 8), (9, 9)]),
        ]);
        let size = 12;

        let (a, antinodes_count) = count_antinodes(antennas, size);
        assert_eq!(14, antinodes_count);
        Ok(())
    }


    #[test]
    fn part2_example_02() -> Result<(), Error> {
        let antennas = HashMap::from([
            //         ('0', vec![(1, 8), (2, 5)]),
            ('T', vec![(0, 0), (1, 3), (2, 1)]),
            
        ]);
        let size = 10;

        let (a, antinodes_count) = count_harmonic_antinodes(antennas, size);
        assert_eq!(9, antinodes_count);
        Ok(())
    }
    #[test]
    fn part1_example() {
        let input = load("test_data/day8.txt").unwrap();
        assert_eq!(part1(input.as_str()), 14);
    }

    #[test]
    fn part2_example() {
        let input = load("test_data/day8.txt").unwrap();
        assert_eq!(part2(input.as_str()), 34);
    }

    fn load(path: &str) -> Result<String, Error> {
        let mut input = String::new();
        let mut input_file = File::open(path)?;
        input_file.read_to_string(&mut input)?;
        Ok(input)
    }
}
