#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::collections::BTreeSet;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Vec<Point>>;
type Point = (i32, i32);

fn drop_sand(cave: &BTreeSet<Point>, x: i32, y: i32, h: i32) -> Point {
    for (nx, ny) in [(0, 1), (-1, 1), (1, 1)].map(|(dx, dy)| (x + dx, y + dy)) {
        if ny > h {
            break;
        }
        if cave.contains(&(nx, ny)) {
            continue;
        }
        return drop_sand(cave, nx, ny, h);
    }
    (x, y)
}

fn add_rock(mut cave: BTreeSet<Point>, points: &[Point]) -> BTreeSet<Point> {
    for point in points.windows(2) {
        let ((sx, sy), (ex, ey)) = (point[0], point[1]);
        let (sx, ex) = (sx.min(ex), sx.max(ex));
        let (sy, ey) = (sy.min(ey), sy.max(ey));
        for y in sy..=ey {
            for x in sx..=ex {
                cave.insert((x, y));
            }
        }
    }
    cave
}

fn solve_part_one(input: &ParsedInput) -> usize {
    let (w, h) = input
        .iter()
        .flatten()
        .fold((0, 0), |(mx, my), &(x, y)| (mx.max(x), my.max(y)));
    let mut cave = input.iter().fold(BTreeSet::new(), |c, r| add_rock(c, r));
    let mut count = 0;
    loop {
        match drop_sand(&cave, 500, 0, h) {
            (x, y) if x >= w || y >= h => break count,
            (x, y) => {
                count += 1;
                cave.insert((x, y));
            }
        }
    }
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let h = input.iter().flatten().fold(0, |my, &(_, y)| my.max(y));
    let mut cave = input.iter().fold(BTreeSet::new(), |t, p| add_rock(t, p));
    let mut count = 0;
    loop {
        match drop_sand(&cave, 500, 0, h + 1) {
            (500, 0) => break count + 1,
            (x, y) => {
                count += 1;
                cave.insert((x, y));
            }
        }
    }
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|points| {
                    points
                        .trim()
                        .split(',')
                        .flat_map(|s| s.parse().ok())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
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
        assert_eq!(result, 24);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 728);
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
        assert_eq!(result, 93);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 27623);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
