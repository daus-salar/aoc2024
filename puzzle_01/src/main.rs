use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> Result<(), Error> {
    let mut a = BinaryHeap::<i32>::new();
    let mut b = BinaryHeap::<i32>::new();
    let path = "input";
    read_into(path, &mut a, &mut b)?;

    let a: Vec<i32> = a.into_sorted_vec();
    let b: Vec<i32> = b.into_sorted_vec();
    println!("Distance of input is {}.", distance_sorted(&a, &b));
    println!("Similarity of b to a is {}.", similarity_score(&a, &b));
    Ok(())
}

fn read_into(path: &str, a: &mut BinaryHeap<i32>, b: &mut BinaryHeap<i32>) -> Result<(), Error> {
    let input = File::open(path)
        .and_then(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).map(|_| buf)
        })
        .map_err(Error::InputFileIOError)?;

    for line in input.lines() {
        let cells: Vec<i32> = line
            .split_whitespace()
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();
        if cells.len() != 2 {
            return Err(Error::InputTwoColumsExpected);
        } else {
            a.push(cells[0]);
            b.push(cells[1]);
        }
    }
    Ok(())
}

pub fn similarity_score(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let frequency = frequency(&a);

    b.iter().fold(0, |score, entry| {
        score + *(frequency.get(entry).get_or_insert(&0)) * entry
    })
}

pub fn frequency(a: &Vec<i32>) -> HashMap<i32, i32> {
    let frequency = a.iter().fold(HashMap::<i32, i32>::new(), |mut map, entry| {
        if let Some(v) = map.get_mut(entry) {
            *v = *v + 1;
        } else {
            map.insert(*entry, 1);
        }
        map
    });
    frequency
}

#[derive(Debug)]
pub enum Error {
    InputTwoColumsExpected,
    InputFileIOError(io::Error),
}

pub fn distance_unsorted(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut a = a.clone();
    let mut b = b.clone();
    a.sort();
    b.sort();

    distance_sorted(&a, &b)
}

pub fn distance_sorted(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    a.iter().zip(b.iter()).map(|p| (p.0 - p.1).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_simple() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        let result = distance_unsorted(&a, &b);
        assert_eq!(result, 11);
        assert_eq!(result, distance_unsorted(&b, &a));
    }

    #[test]
    fn read_into_simple() {
        let mut a = BinaryHeap::<i32>::new();
        let mut b = BinaryHeap::<i32>::new();
        read_into("input_test", &mut a, &mut b).unwrap();
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
}
