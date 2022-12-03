#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<usize>;

fn solve_part_one(input: &ParsedInput) -> usize {
    *input.iter().max().unwrap()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut sorted = input.clone();
    sorted.sort_unstable();
    sorted.iter().rev().take(3).sum()
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().fold(vec![0], |mut acc, line| {
        if line.is_empty() {
            acc.push(0);
            return acc;
        }
        let num = line.parse::<usize>().unwrap();
        let last = acc.last_mut().unwrap();
        *last += num;
        acc
    })
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
        assert_eq!(result, 24000);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 27027);
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
        assert_eq!(result, 45000);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 36975);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
