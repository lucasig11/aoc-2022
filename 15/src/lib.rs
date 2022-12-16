#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::collections::BTreeSet;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

#[cfg(debug_assertions)]
const ROW: i32 = 10;
#[cfg(not(debug_assertions))]
const ROW: i32 = 2_000_000;

#[cfg(debug_assertions)]
const UPPER_BOUND: i32 = 20;
#[cfg(not(debug_assertions))]
const UPPER_BOUND: i32 = 4_000_000;

type Point = (i32, i32);
type ParsedInput = Vec<Point>;

fn solve_part_one(input: &ParsedInput) -> usize {
    let mut scanned = scan(input, ROW).iter().fold(
        BTreeSet::new(),
        |mut s, &(start, end)| {
            (start..=end).for_each(|x| {
                s.insert(x);
            });
            s
        },
    );

    input
        .iter()
        .tuples()
        .filter_map(|(_, (x, y))| (*y == ROW).then_some(x))
        .for_each(|x| {
            scanned.remove(x);
        });

    scanned.len()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    for y in 0..=UPPER_BOUND {
        let ranges = merge(&scan(input, y), UPPER_BOUND);
        if let &[(_, x), _] = &ranges[..] {
            return ((x + 1) * 4_000_000 + y) as usize;
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .flat_map(|line| {
            line.split(|c: char| c.is_ascii_punctuation() && c != '-')
                .filter_map(|s| s.parse().ok())
                .tuples()
                .collect_vec()
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

fn clamp(range: (i32, i32), max: i32) -> (i32, i32) {
    (range.0.max(0), range.1.min(max))
}

fn intersects(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn scan(points: &[Point], row: i32) -> Vec<(i32, i32)> {
    points
        .iter()
        .tuples()
        .filter_map(|(&(sx, sy), &(bx, by))| {
            let d = i32::abs(sx - bx) + i32::abs(sy - by);
            let t = d - i32::abs(row - sy);
            (t > 0).then_some((sx - t, sx + t))
        })
        .collect()
}

fn merge(ranges: &[(i32, i32)], upper_bound: i32) -> Vec<(i32, i32)> {
    ranges.iter().map(|&r| clamp(r, upper_bound)).sorted().fold(
        vec![],
        |mut merged, (start, end)| {
            if let Some((last_start, last_end)) = merged.last_mut() {
                if intersects((start, end), (*last_start, *last_end)) {
                    *last_end = i32::max(*last_end, end);
                    return merged;
                }
            }
            merged.push((start, end));
            merged
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 26);

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
        assert_eq!(result, 1924);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 36975);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
