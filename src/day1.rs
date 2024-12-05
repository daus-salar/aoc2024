use std::{
    collections::{BinaryHeap, HashMap},
    io::{self},
};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut a = BinaryHeap::<i32>::new();
    let mut b = BinaryHeap::<i32>::new();
    read_from(input, &mut a, &mut b).unwrap();

    let a: Vec<i32> = a.into_sorted_vec();
    let b: Vec<i32> = b.into_sorted_vec();
    distance_sorted(&a, &b)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut a = BinaryHeap::<i32>::new();
    let mut b = BinaryHeap::<i32>::new();
    read_from(input, &mut a, &mut b).unwrap();

    let a: Vec<i32> = a.into_sorted_vec();
    let b: Vec<i32> = b.into_sorted_vec();
    similarity_score(&a, &b)
}

fn read_from(input: &str, a: &mut BinaryHeap<i32>, b: &mut BinaryHeap<i32>) -> Result<(), Error> {
    for line in input.lines() {
        if let Ok(cells) = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
        {
            if cells.len() != 2 {
                return Err(Error::InputTwoColumsExpected(cells.len()));
            } else {
                a.push(cells[0]);
                b.push(cells[1]);
            }
        }
    }
    Ok(())
}

pub fn similarity_score(a: &[i32], b: &[i32]) -> i32 {
    let frequency = frequency(a);

    b.iter().fold(0, |score, entry| {
        score + *(frequency.get(entry).get_or_insert(&0)) * entry
    })
}

pub fn frequency(a: &[i32]) -> HashMap<i32, i32> {
    let frequency = a.iter().fold(HashMap::<i32, i32>::new(), |mut map, entry| {
        if let Some(v) = map.get_mut(entry) {
            *v += 1;
        } else {
            map.insert(*entry, 1);
        }
        map
    });
    frequency
}

#[derive(Debug)]
pub enum Error {
    InputTwoColumsExpected(usize),
    InputFileIOError(io::Error),
}

pub fn distance_sorted(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|p| (p.0 - p.1).abs()).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use io::Read;

    use super::*;

    #[test]
    fn read_into_simple() {
        let mut a = BinaryHeap::<i32>::new();
        let mut b = BinaryHeap::<i32>::new();
        read_into("test_data/day1.txt", &mut a, &mut b).unwrap();
        assert_eq!(a.into_sorted_vec(), vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(b.into_sorted_vec(), vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn frequency_simple() {
        let a = vec![3, 4, 2, 1, 3, 3];

        assert_eq!(
            HashMap::from([(3, 3), (4, 1), (2, 1), (1, 1)]),
            frequency(&a)
        );
    }

    #[test]
    fn similarity_score_simple() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(31, similarity_score(&a, &b));
    }

    fn read_into(
        path: &str,
        a: &mut BinaryHeap<i32>,
        b: &mut BinaryHeap<i32>,
    ) -> Result<(), Error> {
        let input = File::open(path)
            .and_then(|mut f| {
                let mut buf = String::new();
                f.read_to_string(&mut buf).map(|_| buf)
            })
            .map_err(Error::InputFileIOError)?;

        let input = input.as_str();
        read_from(input, a, b)
    }
}
