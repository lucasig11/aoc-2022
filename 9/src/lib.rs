#![feature(test)]

use std::collections::BTreeSet;
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Step(Direction, i32);

type ParsedInput = Vec<Step>;
type Coord = (i32, i32);

const ADJACENCY_MAP: [(i32, i32); 9] = [
    (0, -1),
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

fn is_adjacent((x, y): Coord, (tx, ty): Coord) -> bool {
    let (dx, dy) = (x - tx, y - ty);
    dx.abs() <= 1 && dy.abs() <= 1
}

fn follow_head(head: Coord, (tx, ty): Coord) -> Coord {
    if is_adjacent(head, (tx, ty)) {
        return (tx, ty);
    }
    let (dx, dy) = (head.0 - tx, head.1 - ty);
    if dx <= 1 || dy <= 1 {
        for (dx, dy) in ADJACENCY_MAP[..4].iter() {
            let px = tx + dx;
            let py = ty + dy;
            let (dx, dy) = (head.0 - px, head.1 - py);
            if is_adjacent(head, (px, py)) && (dx == 0 || dy == 0) {
                return (px, py);
            }
        }
    }
    for (dx, dy) in ADJACENCY_MAP[5..].iter() {
        let px = tx + dx;
        let py = ty + dy;
        if is_adjacent(head, (px, py)) {
            return (px, py);
        }
    }
    unreachable!();
}

fn apply_step((x, y): Coord, step: Step) -> Coord {
    match step {
        Step(Direction::Up, n) => (x, y - n),
        Step(Direction::Down, n) => (x, y + n),
        Step(Direction::Left, n) => (x - n, y),
        Step(Direction::Right, n) => (x + n, y),
    }
}

fn solve_part_one(input: &ParsedInput) -> usize {
    let (mut head, mut tail) = ((0, 4), (0, 4));
    let mut visited = BTreeSet::new();
    for Step(d, c) in input {
        for _ in 0..*c {
            head = apply_step(head, Step(*d, 1));
            tail = follow_head(head, tail);
            visited.insert(tail);
        }
    }
    visited.len()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut knots = [(11, 15); 10];
    let mut visited = BTreeSet::new();
    for Step(d, c) in input {
        for _ in 0..*c {
            knots[0] = apply_step(knots[0], Step(*d, 1));
            for i in 1..knots.len() {
                knots[i] = follow_head(knots[i - 1], knots[i]);
                if i == knots.len() - 1 {
                    visited.insert(knots[i]);
                }
            }
        }
    }
    visited.len()
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().fold(vec![], |mut acc, curr| {
        let (dir, dist) = curr.split_once(' ').unwrap();
        let dist = dist.parse().unwrap();
        match dir {
            "U" => acc.push(Step(Direction::Up, dist)),
            "D" => acc.push(Step(Direction::Down, dist)),
            "L" => acc.push(Step(Direction::Left, dist)),
            "R" => acc.push(Step(Direction::Right, dist)),
            _ => panic!("Unknown direction"),
        };
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
        assert_eq!(result, 88);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 6376);
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
        assert_eq!(result, 36);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2607);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
