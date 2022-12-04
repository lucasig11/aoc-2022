#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type Assignment = (usize, usize);
type ParsedInput = Vec<(Assignment, Assignment)>;

fn solve_part_one(input: &ParsedInput) -> usize {
    input
        .iter()
        .filter(|((min_a, max_a), (min_b, max_b))| {
            min_a <= min_b && max_a >= max_b || min_b <= min_a && max_b >= max_a
        })
        .count()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    input
        .iter()
        .filter(|((min_a, max_a), (min_b, max_b))| {
            max_a >= min_b && max_b >= min_a
        })
        .count()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|p| {
                    p.split('-')
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            ((parts[0][0], parts[0][1]), (parts[1][0], parts[1][1]))
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
        assert_eq!(result, 2);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 526);
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
        assert_eq!(result, 4);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 36975);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
