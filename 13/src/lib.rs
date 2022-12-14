#![feature(array_chunks)]
#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{cmp::Ordering, iter::Peekable};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Packet>;

#[derive(Debug, Clone)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

fn solve_part_one(input: &ParsedInput) -> usize {
    input
        .array_chunks::<2>()
        .enumerate()
        .filter_map(|(i, [lhs, rhs])| (lhs < rhs).then_some(i + 1))
        .sum()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let divider_packets = [
        Packet::parse("[[2]]").unwrap(),
        Packet::parse("[[6]]").unwrap(),
    ];
    input
        .iter()
        .chain(divider_packets.iter())
        .sorted()
        .enumerate()
        .filter_map(|(i, p)| divider_packets.contains(p).then_some(i + 1))
        .product()
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().filter_map(Packet::parse).collect()
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

impl Packet {
    pub fn parse(input: &str) -> Option<Self> {
        let mut chars = input.chars().peekable();
        Packet::parse_rec(&mut chars)
    }

    fn parse_rec<I: Iterator<Item = char>>(
        cursor: &mut Peekable<I>,
    ) -> Option<Packet> {
        match cursor.peek()? {
            '0'..='9' => {
                let mut num = String::with_capacity(2);
                while matches!(cursor.peek(), Some(c) if c.is_ascii_digit()) {
                    num.push(cursor.next()?);
                }
                num.parse().ok().map(Packet::Integer)
            }
            '[' => {
                let _open = cursor.next();
                let mut list = vec![];
                while let Some(items) = Packet::parse_rec(cursor) {
                    list.push(items);
                }
                let _close = cursor.next();
                Some(Packet::List(list))
            }
            ']' => None,
            _ => {
                cursor.next();
                Packet::parse_rec(cursor)
            }
        }
    }
}

impl Eq for Packet {}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Integer(lhs), Integer(rhs)) => lhs.cmp(rhs),
            (List(lhs), List(rhs)) => {
                let mut lhs = lhs.iter();
                let mut rhs = rhs.iter();
                loop {
                    match (lhs.next(), rhs.next()) {
                        (Some(lhs), Some(rhs)) => match lhs.cmp(rhs) {
                            Ordering::Equal => continue,
                            ord => return ord,
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }

            (lhs @ List(_), rhs @ Integer(_)) => {
                lhs.cmp(&List(vec![rhs.clone()]))
            }
            (lhs @ Integer(_), rhs @ List(_)) => {
                List(vec![lhs.clone()]).cmp(rhs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 13);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 5393);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 140);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 26712);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
