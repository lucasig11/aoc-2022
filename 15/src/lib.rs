#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::collections::BTreeSet;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

#[cfg(debug_assertions)]
const ROW: isize = 10;
#[cfg(not(debug_assertions))]
const ROW: isize = 2_000_000;

#[cfg(debug_assertions)]
const UPPER_BOUND: usize = 20;
#[cfg(not(debug_assertions))]
const UPPER_BOUND: usize = 4_000_000;

type Point = (isize, isize);
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
        let ranges = merge(&scan(input, y as isize), UPPER_BOUND);
        if let &[(_, x), _] = &ranges[..] {
            return (x + 1) * 4_000_000usize + y;
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

fn clamp(range: (isize, isize), max: usize) -> (usize, usize) {
    let low = range.0.max(0) as usize;
    let high = (range.1 as usize).min(max);
    (low, high)
}

fn intersects(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn scan(points: &[Point], row: isize) -> Vec<(isize, isize)> {
    points
        .iter()
        .tuples()
        .filter_map(|(&(sx, sy), &(bx, by))| {
            let d = isize::abs(sx - bx) + isize::abs(sy - by);
            let t = d - isize::abs(row - sy);
            (t > 0).then_some((sx - t, sx + t))
        })
        .collect()
}

fn merge(ranges: &[(isize, isize)], upper_bound: usize) -> Vec<(usize, usize)> {
    ranges.iter().map(|&r| clamp(r, upper_bound)).sorted().fold(
        vec![],
        |mut merged, (low, high)| {
            if let Some((ml, mh)) = merged.last_mut() {
                if intersects((low, high), (*ml, *mh)) {
                    *mh = usize::max(*mh, high);
                    return merged;
                }
            }
            merged.push((low, high));
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
