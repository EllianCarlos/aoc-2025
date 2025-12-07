use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use std::cmp::max;
use std::cmp::min;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[String]) -> u64 {
    let grid: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows).flat_map(|r| (0..cols).map(move |c| (r,c)))
        .filter(|&(r,c)| grid[r][c] == '@')
        .filter(|&(r,c)| {
            let neighbor_count = (-1..=1).flat_map(|dr| (-1..=1).map(move |dc| (dr,dc)))
                .filter(|&(dr,dc)| dr != 0 || dc != 0)
                .filter(|&(dr,dc)| {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    nr >= 0 && nr < rows as isize &&
                    nc >= 0 && nc < cols as isize &&
                    grid[nr as usize][nc as usize] == '@'
                })
                .count() as u64;

            neighbor_count < 4
        })
        .count() as u64
}

#[aoc(day4, part2)]
pub fn part2(_input: &[String]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day4.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day4.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
