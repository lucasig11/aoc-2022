#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<(char, char)>;

fn score_p1((opponent, me): &(char, char)) -> usize {
    match (opponent, me) {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,

        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,

        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,

        _ => unreachable!(),
    }
}

fn solve_part_one(input: &ParsedInput) -> usize {
    input.iter().map(score_p1).sum()
}

fn score_p2((opponent, outcome): &(char, char)) -> usize {
    match (opponent, outcome) {
        // Rock
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,

        // Paper
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,

        // Scissors
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,

        _ => unreachable!(),
    }
}

fn solve_part_two(input: &ParsedInput) -> usize {
    input.iter().map(score_p2).sum()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent = chars.next().unwrap();
            let _ = chars.next().unwrap();
            let me = chars.next().unwrap();
            (opponent, me)
        })
        .collect()
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
        assert_eq!(result, 15);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 10595);
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
        assert_eq!(result, 12);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 9541);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
