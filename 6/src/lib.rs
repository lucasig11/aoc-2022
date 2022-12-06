#![feature(test)]

use std::collections::BTreeSet;
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput<'input> = &'input str;

fn get_first_unique_win_offset(bytes: &[u8], sz: usize) -> usize {
    bytes
        .iter()
        .enumerate()
        .find_map(|(i, _)| {
            (bytes[i..i + sz].iter().collect::<BTreeSet<_>>().len() == sz)
                .then_some(i + sz)
        })
        .unwrap()
}

fn solve_part_one(input: &ParsedInput) -> usize {
    get_first_unique_win_offset(input.as_bytes(), 4)
}

fn solve_part_two(input: &ParsedInput) -> usize {
    get_first_unique_win_offset(input.as_bytes(), 14)
}

fn parse_input(input: &str) -> ParsedInput {
    input
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 7);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1658);
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
        assert_eq!(result, 19);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2260);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
