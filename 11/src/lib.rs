#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::collections::VecDeque;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Monkey>;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Square,
    Add(usize),
    Mul(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    test: usize,
    count: usize,
    op: Operation,
    targets: [usize; 2],
    items: VecDeque<usize>,
}

fn do_round<F: Fn(usize) -> usize>(
    mut monkeys: Vec<Monkey>,
    relief_strategy: F,
) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            monkeys[i].count += 1;
            let worry_level = match monkeys[i].op {
                Operation::Add(x) => item + x,
                Operation::Mul(x) => item * x,
                Operation::Square => item * item,
            };
            let worry_level = relief_strategy(worry_level);
            let dest = monkeys[i].targets
                [usize::from(worry_level % monkeys[i].test == 0)];
            monkeys[dest].items.push_back(worry_level);
        }
    }
    monkeys
}

fn solve_part_one(input: &ParsedInput) -> usize {
    (0..20)
        .fold(input.to_owned(), |state, _| do_round(state, |w| w / 3))
        .into_iter()
        .map(|x| x.count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let lcm = input.iter().map(|m| m.test).product::<usize>();
    (0..10_000)
        .fold(input.to_owned(), |state, _| do_round(state, |w| w % lcm))
        .into_iter()
        .map(|x| x.count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .filter_map(|lines| {
            let mut lines = lines.skip(1);
            Some(Monkey {
                count: 0,
                items: parse_items(lines.next()?),
                op: parse_op(lines.next()?)?,
                test: parse_test(lines.next()?)?,
                targets: parse_decisions(lines.next()?, lines.next()?)?,
            })
        })
        .collect::<Vec<_>>()
}

fn parse_items(items: &str) -> VecDeque<usize> {
    items
        .split_whitespace()
        .filter_map(|x| x.trim_end_matches(',').parse().ok())
        .collect()
}

fn parse_op(op: &str) -> Option<Operation> {
    let mut expr = op.split_whitespace().rev().take(2);
    let rhs = expr.next()?;
    let op = expr.next()?;
    Some(match (op, rhs) {
        ("*", "old") => Operation::Square,
        ("+", "old") => Operation::Mul(2),
        ("*", num) => Operation::Mul(num.parse().ok()?),
        ("+", num) => Operation::Add(num.parse().ok()?),
        _ => unreachable!(),
    })
}

fn parse_test(test: &str) -> Option<usize> {
    test.split_whitespace().last()?.parse().ok()
}

fn parse_decisions(if_true: &str, if_false: &str) -> Option<[usize; 2]> {
    let if_true = if_true.split_whitespace().last()?;
    let if_false = if_false.split_whitespace().last()?;
    Some([if_false.parse().ok()?, if_true.parse().ok()?])
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
        assert_eq!(result, 10605);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 58794);
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
        assert_eq!(result, 2713310158);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 20151213744);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
