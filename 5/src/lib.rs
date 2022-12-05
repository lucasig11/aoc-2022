#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;
use std::ops::{Deref, DerefMut};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

#[derive(Debug, Clone, Copy)]
struct Move(usize, usize, usize);

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

type ParsedInput = (Vec<Move>, Vec<Stack>);

fn solve_part_one(input: &ParsedInput) -> String {
    let (mut moves, mut stacks) = input.clone();
    while let Some(Move(amount, src, dst)) = moves.pop() {
        let stack_len = stacks[src].len();
        let slice = stacks[src].split_off(stack_len - amount);
        stacks[dst].extend(slice.iter().rev());
    }
    stacks.iter().map_while(|c| c.last()).collect()
}

fn solve_part_two(input: &ParsedInput) -> String {
    let (mut moves, mut stacks) = input.clone();
    while let Some(Move(amount, src, dst)) = moves.pop() {
        let stack_len = stacks[src].len();
        let slice = stacks[src].split_off(stack_len - amount);
        stacks[dst].extend(slice);
    }
    stacks.iter().map_while(|c| c.last()).collect()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .rev()
        .fold((vec![], vec![]), |(mut ops, mut stacks), line| {
            match line.trim().chars().peekable().peek() {
                Some('[') => {
                    let mut chars = line.char_indices();
                    while let Ok([_, (idx, ch), _]) = chars.next_chunk::<3>() {
                        if ch.is_alphabetic() {
                            stacks[(idx - 1) / 4].push(ch);
                        }
                        chars.next();
                    }
                }
                Some(c) if c.is_ascii_digit() => {
                    stacks = line
                        .split_whitespace()
                        .filter_map(|s| {
                            s.parse::<u8>().map(|_| Stack(vec![])).ok()
                        })
                        .collect()
                }
                Some('m') => {
                    if let &[amount, src, dest] = &line
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<_>>()[..]
                    {
                        ops.push(Move(amount, src - 1, dest - 1));
                    }
                }
                _ => (),
            }
            (ops, stacks)
        })
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

impl Deref for Stack {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(&result, "CMZ");

        #[cfg(not(debug_assertions))]
        assert_eq!(result, "TWSGQHNHL");
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
        assert_eq!(result, "MCD");

        #[cfg(not(debug_assertions))]
        assert_eq!(result, "JNRSCDWPP");
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
