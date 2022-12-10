#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Opcode>;

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Noop,
    AddX(i32),
}

impl Opcode {
    fn execute(&self, acc: i32) -> i32 {
        match self {
            Opcode::Noop => acc,
            Opcode::AddX(x) => acc + x,
        }
    }
}

fn get_pixel_value(x: i32, cycle: usize) -> char {
    if (x - 1..=x + 1).contains(&((cycle as i32) % 40)) {
        '#'
    } else {
        '.'
    }
}

fn solve_part_one(input: &ParsedInput) -> i32 {
    input
        .iter()
        .fold(vec![1], |mut xs, op| {
            let x = *xs.last().unwrap();
            xs.push(op.execute(x));
            xs
        })
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| ((i + 1) as i32) * x)
        .sum()
}

fn solve_part_two(input: &ParsedInput) -> String {
    input
        .iter()
        .enumerate()
        .fold((1, vec!['#']), |(x, mut crt), (cycle, op)| {
            let x = op.execute(x);
            crt.push(get_pixel_value(x, cycle + 1));
            (x, crt)
        })
        .1
        .chunks_exact(40)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .flat_map(
            |curr| match curr.split_whitespace().collect::<Vec<_>>()[..] {
                ["noop"] => vec![Opcode::Noop],
                ["addx", n] => {
                    let n = n.parse::<i32>().unwrap();
                    vec![Opcode::Noop, Opcode::AddX(n)]
                }
                _ => unreachable!(),
            },
        )
        .collect()
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2:\n{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 4512);

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
        assert_eq!(
            result,
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );

        #[cfg(not(debug_assertions))]
        assert_eq!(
            result,
            r#"####..##..####.#..#.####..##..#....###..
#....#..#....#.#..#....#.#..#.#....#..#.
###..#......#..#..#...#..#..#.#....#..#.
#....#.....#...#..#..#...####.#....###..
#....#..#.#....#..#.#....#..#.#....#.#..
####..##..####..##..####.#..#.####.#..#."#
        );
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
