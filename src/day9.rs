use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    iter::repeat_n,
};

use aoc_runner_derive::{aoc, aoc_generator};

fn parse(input: &str) -> Vec<Block> {
    let mut disk_map = Vec::<Block>::new();
    for (idx, c) in input.char_indices() {
        if idx % 2 == 0 {
            let id = idx / 2;
            let size: u32 = c.to_digit(10).unwrap();
            disk_map.push(Block::Id(id as u16, size));
        } else {
            let size: u32 = c.to_digit(10).unwrap();
            if size > 0 {
                disk_map.push(Block::Free(size));
            }
        }
    }
    disk_map
}

fn defragment(mut disk_map: VecDeque<Block>) -> Vec<Block> {
    let mut defragmented: Vec<Block> = Vec::new();
    loop {
        let Some(from_left) = disk_map.pop_front() else {
            break;
        };
        match from_left {
            Block::Free(free_space) => {
                let pop_back = disk_map.pop_back();
                match pop_back {
                    Some(Block::Id(id, size)) if size < free_space => {
                        defragmented.push(Block::Id(id, size));
                        disk_map.push_front(Block::Free(free_space - size))
                    }
                    Some(Block::Id(id, size)) if size > free_space => {
                        defragmented.push(Block::Id(id, free_space));
                        disk_map.push_back(Block::Id(id, size - free_space))
                    }
                    Some(Block::Id(id, size)) if size == free_space => {
                        defragmented.push(Block::Id(id, free_space));
                    }
                    Some(Block::Free(..)) => {
                        /* blow away free space at the end but put back free space at the front */
                        disk_map.push_front(from_left);
                    }
                    Some(c) => {
                        panic!("{:?} not expected ", c)
                    }
                    None => {
                        break;
                    }
                }
            }
            c => defragmented.push(c),
        }
        /*
        print_map(&defragmented);
        print!("|");
        print_map2(&disk_map);
        println!("");
        */
    }
    defragmented
}

fn print_map(v: &Vec<Block>) {
    for b in v {
        match b {
            Block::Free(size) => {
                print!["{:}", "...........".get(0..(*size as usize)).unwrap()];
            }
            Block::Id(id, size) => {
                print![
                    "{:}",
                    repeat_n(format!("{:}", id), *size as usize).collect::<String>()
                ];
            }
        }
    }
}
fn print_map2(v: &VecDeque<Block>) {
    for b in v {
        match b {
            Block::Free(size) => {
                print!["{:}", "...........".get(0..(*size as usize)).unwrap()];
            }
            Block::Id(id, size) => {
                print![
                    "{:}",
                    repeat_n(format!("{:}", id), *size as usize).collect::<String>()
                ];
            }
        }
    }
}
fn hash(defragmented: Vec<Block>) -> u64 {
    let mut hash: u64 = 0;
    for (idx, id) in defragmented
        .iter()
        .map(|b| {
            if let Block::Id(id, size) = b {
                repeat_n(*id, *size as usize).collect::<Vec<u16>>()
            } else {
                Vec::<u16>::new()
            }
        })
        .flatten()
        .enumerate()
    {
        hash += (idx as u64) * (id as u64);
    }
    hash
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);
    let defragmented = defragment(input.into());
    hash(defragmented)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> String {
    todo!()
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Block {
    Free(u32),
    Id(u16, u32),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free(size) => f.write_fmt(format_args!("Free({:})", size)),
            Block::Id(id, size) => f.write_fmt(format_args!("Id({:},{:})", id, size)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, iter};

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(1928, part1("2333133121414131402"))
    }

    #[test]
    fn parse_test() {
        let input = "12345";

        assert_eq!(
            parse(input),
            vec![
                Block::Id(0, 1),
                Block::Free(2),
                Block::Id(1, 3),
                Block::Free(4),
                Block::Id(2, 5)
            ]
        )
    }

    #[test]
    fn parse_test2() {
        let input = "2333133121414131402";

        assert_eq!(
            parse(input),
            vec![
                Block::Id(0, 2),
                Block::Free(3),
                Block::Id(1, 3),
                Block::Free(3),
                Block::Id(2, 1),
                Block::Free(3),
                Block::Id(3, 3),
                Block::Free(1),
                Block::Id(4, 2),
                Block::Free(1),
                Block::Id(5, 4),
                Block::Free(1),
                Block::Id(6, 4),
                Block::Free(1),
                Block::Id(7, 3),
                Block::Free(1),
                Block::Id(8, 4),
                Block::Id(9, 2)
            ]
        )
    }

    #[test]
    fn defrag_test() {
        let disk_map = VecDeque::from(vec![
            Block::Id(0, 1),
            Block::Free(2),
            Block::Id(1, 3),
            Block::Free(4),
            Block::Id(2, 5),
        ]);

        let disk_map = defragment(disk_map);

        assert_eq!(
            vec![
                Block::Id(0, 1),
                Block::Id(2, 2),
                Block::Id(1, 3),
                Block::Id(2, 3)
            ],
            disk_map
        );
    }
    #[test]
    fn defrag_test2() {
        let disk_map = VecDeque::from(vec![
            Block::Id(0, 2),
            Block::Free(3),
            Block::Id(1, 3),
            Block::Free(3),
            Block::Id(2, 1),
            Block::Free(3),
            Block::Id(3, 3),
            Block::Free(1),
            Block::Id(4, 2),
            Block::Free(1),
            Block::Id(5, 4),
            Block::Free(1),
            Block::Id(6, 4),
            Block::Free(1),
            Block::Id(7, 3),
            Block::Free(1),
            Block::Id(8, 4),
            Block::Id(9, 2),
        ]);

        let disk_map = defragment(disk_map);

        assert_eq!(
            vec![
                Block::Id(0, 2),
                Block::Id(9, 2),
                Block::Id(8, 1),
                Block::Id(1, 3),
                Block::Id(8, 3),
                Block::Id(2, 1),
                Block::Id(7, 3),
                Block::Id(3, 3),
                Block::Id(6, 1),
                Block::Id(4, 2),
                Block::Id(6, 1),
                Block::Id(5, 4),
                Block::Id(6, 1),
                Block::Id(6, 1),
            ],
            disk_map
        );
    }

    #[test]
    fn breack_up_test() {
        let defragmented = vec![
            Block::Id(0, 2),
            Block::Id(9, 2),
            Block::Id(8, 1),
            Block::Id(1, 3),
            Block::Id(8, 3),
            Block::Id(2, 1),
            Block::Id(7, 3),
            Block::Id(3, 3),
            Block::Id(6, 1),
            Block::Id(4, 2),
            Block::Id(6, 1),
            Block::Id(5, 4),
            Block::Id(6, 1),
            Block::Id(6, 1),
        ];

        let hash = hash(defragmented);

        assert_eq!(1928, hash);
    }

    #[test]
    fn part2_example() {}
}
