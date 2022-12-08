#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput<'input> = Vec<Dir<'input>>;

struct Dir<'input> {
    size: usize,
    dirs: Vec<usize>,
    name: &'input str,
    parent: Option<usize>,
}

impl<'input> Dir<'input> {
    fn new(name: &'input str, parent: Option<usize>) -> Self {
        Self {
            name,
            parent,
            size: 0,
            dirs: Vec::new(),
        }
    }
}

fn get_dir_size(dirs: &[Dir], idx: usize) -> usize {
    dirs[idx].size
        + dirs[idx]
            .dirs
            .iter()
            .map(|&d| get_dir_size(dirs, d))
            .sum::<usize>()
}

fn solve_part_one(input: &ParsedInput) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(idx, _)| get_dir_size(input, idx))
        .filter(|&size| size <= 100_000)
        .sum()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let used_space = get_dir_size(input, 0);
    let free_space = 70_000_000 - used_space;
    let space_to_free = 30_000_000 - free_space;

    input
        .iter()
        .enumerate()
        .map(|(idx, _)| get_dir_size(input, idx))
        .filter(|&size| size >= space_to_free)
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .fold(
            (vec![Dir::new("/", None)], 0usize),
            |(mut fs, mut cwd_id), line| {
                match *line.split_whitespace().collect::<Vec<_>>() {
                    ["$", "cd", dir_name] => {
                        cwd_id = match dir_name {
                            ".." => fs[cwd_id].parent.unwrap(),
                            "/" => cwd_id,
                            _ => *fs[cwd_id]
                                .dirs
                                .iter()
                                .find(|&&d| fs[d].name == dir_name)
                                .unwrap(),
                        }
                    }
                    ["dir", dir_name] => {
                        let idx = fs.len();
                        fs.push(Dir::new(dir_name, Some(cwd_id)));
                        fs[cwd_id].dirs.push(idx);
                    }
                    [size, ..] if size.parse::<usize>().is_ok() => {
                        fs[cwd_id].size += size.parse::<usize>().unwrap()
                    }
                    _ => {}
                };
                (fs, cwd_id)
            },
        )
        .0
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
        assert_eq!(result, 95437);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1206825);
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
        assert_eq!(result, 24933642);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 9608311);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
