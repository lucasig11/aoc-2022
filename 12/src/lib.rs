#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = ((usize, usize), Vec<Node>);

#[derive(Debug, Clone)]
struct Node {
    value: u8,
    children: Vec<usize>,
}

fn get_shortest_distance(nodes: &[Node], start: usize, target: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; nodes.len()];
    queue.push_back((start, 0));
    while let Some((node, distance)) = queue.pop_front() {
        if node == target {
            return distance;
        }
        if visited[node] {
            continue;
        }
        visited[node] = true;
        for &child in &nodes[node].children {
            if nodes[child].value <= nodes[node].value + 1 {
                queue.push_back((child, distance + 1));
            }
        }
    }
    usize::MAX
}

fn solve_part_one(((start, end), nodes): &ParsedInput) -> usize {
    get_shortest_distance(nodes, *start, *end)
}

fn solve_part_two(((_, target), nodes): &ParsedInput) -> usize {
    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| (node.value == b'a').then_some(i))
        .map(|i| get_shortest_distance(nodes, i, *target))
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> ParsedInput {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    input.lines().enumerate().fold(
        ((0, 0), vec![]),
        |((mut start, mut end), mut nodes), (y, line)| {
            nodes.extend(line.char_indices().map(|(x, ch)| {
                let value = match ch {
                    'S' => {
                        start = y * width + x;
                        b'a'
                    }
                    'E' => {
                        end = y * width + x;
                        b'z'
                    }
                    ch => ch as u8,
                };

                let children = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter_map(|(dx, dy)| {
                        let x = x as i32 + dx;
                        let y = y as i32 + dy;
                        if x < 0
                            || y < 0
                            || x as usize >= width
                            || y as usize >= height
                        {
                            return None;
                        }
                        let x = x as usize;
                        let y = y as usize;
                        Some(y * width + x)
                    })
                    .collect();

                Node { value, children }
            }));
            ((start, end), nodes)
        },
    )
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
        assert_eq!(result, 31);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 330);
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
        assert_eq!(result, 29);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 321);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
