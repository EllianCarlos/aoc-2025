use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|line| line.chars().map(|s| s.to_string()).collect()).collect()
}

fn traverse(
    grid: &[Vec<String>], 
    mut visited: HashSet<(usize, usize)>, 
    r: usize, 
    c: usize
) -> HashSet<(usize, usize)> {
    if r >= grid.len() {
        return visited;
    }

    if visited.contains(&(r, c)) {
        return visited;
    }

    visited.insert((r, c));

    match grid[r][c].as_str() {
        "^" => {
            let next_steps = [(r, c - 1), (r, c + 1)];

            next_steps.iter()
                .fold(visited, |acc_set, &(nr, nc)| {
                    traverse(grid, acc_set, nr, nc)
                })
        },
        _ => {
            traverse(grid, visited, r + 1, c)
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &[Vec<String>]) -> u64 {
    traverse(input, HashSet::new(), 0, input[0].len() / 2)
        .iter()
        .filter(|&&(r, c)| input[r][c] == "^")
        .count() as u64
}

#[aoc(day7, part2)]
pub fn part2(_input: &[Vec<String>]) -> u64 {
    0
}
