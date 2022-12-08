#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Vec<u32>>;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn get_scenic_score(grid: &[Vec<u32>], x: usize, y: usize) -> usize {
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return 0;
    }

    DIRECTIONS.iter().fold(1, |result, (dx, dy)| {
        let mut px = ((x as i32) + *dx) as usize;
        let mut py = ((y as i32) + *dy) as usize;
        let mut view_dist = 0;
        loop {
            view_dist += 1;
            if px == 0
                || py == 0
                || px == grid.len() - 1
                || py == grid[px].len() - 1
                || grid[px][py] >= grid[x][y]
            {
                break;
            }
            px = ((px as i32) + *dx) as usize;
            py = ((py as i32) + *dy) as usize;
        }
        result * view_dist
    })
}

fn is_tree_visible(grid: &[Vec<u32>], x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return true;
    }
    for (dx, dy) in &DIRECTIONS {
        let mut px = ((x as i32) + *dx) as usize;
        let mut py = ((y as i32) + *dy) as usize;
        while grid[x][y] > grid[px][py] {
            if px == 0
                || py == 0
                || px == grid.len() - 1
                || py == grid[px].len() - 1
            {
                return true;
            }
            px = ((px as i32) + *dx) as usize;
            py = ((py as i32) + *dy) as usize;
        }
    }
    false
}

fn solve_part_one(input: &ParsedInput) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, _)| is_tree_visible(input, x, *y))
                .count()
        })
        .sum::<usize>()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| get_scenic_score(input, x, y))
                .max()
        })
        .max()
        .flatten()
        .unwrap()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| line.chars().map_while(|c| c.to_digit(10)).collect())
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
        assert_eq!(result, 21);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1684);
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
        assert_eq!(result, 8);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 486540);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
