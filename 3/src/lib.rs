#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput<'i> = Vec<&'i str>;

fn priority(c: &char) -> u8 {
    match c {
        'a'..='z' => *c as u8 - 96,
        'A'..='Z' => *c as u8 - 38,
        _ => panic!("Invalid char: {}", c),
    }
}

fn solve_part_one(input: &ParsedInput) -> usize {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            let mut common =
                a.chars().filter(|c| b.contains(*c)).collect::<Vec<_>>();
            common.dedup();
            common.iter().map(|c| priority(c) as usize).sum::<usize>()
        })
        .sum()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    input
        .chunks_exact(3)
        .map(|chunk| {
            if let &[a, b, c] = chunk {
                let common: char = a
                    .chars()
                    .find(|ch| b.contains(*ch) && c.contains(*ch))
                    .unwrap();
                return priority(&common) as usize;
            }
            unreachable!()
        })
        .sum()
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().collect()
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
        assert_eq!(result, 157);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 7691);
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
        assert_eq!(result, 70);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2508);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
